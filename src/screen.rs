use crate::geometry::Point;
use crate::utils::handle_result;
use napi::bindgen_prelude::*;
use napi_derive::napi;

// code(core): use ./geometry.rs 's struct Point
// code(core): use ./utils.rs 's fn handle_result
// code(core): use napi::bindgen_prelude 's all api
// code(core): use napi_derive 's fn napi

// code(core): def struct ImageData
// code(core): use napi(object) macro to label struct ImageData
// code(core): use derive(Debug, Clone) macro to label struct ImageData
// code(core): with data,width,height,pixel_width prop

#[napi]
#[derive(Debug, Clone)]
pub struct ImageData {
  pub data: Vec<u8>,
  pub width: u32,
  pub height: u32,
  pub pixel_width: u8,
}

// code(core): def struct Color
// code(core): use napi(object) macro to label struct Color
// code(core): use derive(Debug, Clone) macro to label struct Color
// code(core): with r,g,b prop

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

// code(core): def const MAGENTA
// code(core): with r,g,b prop and value (255,0,255)
// code(core): use napi macro to label const MAGENTA

#[napi]
pub const MAGENTA: Color = Color {
  r: 255,
  g: 0,
  b: 255,
};

// code(core): def fn read_image_data
// code(core): use napi macro to label it
// code(core): use fn tokio::spawn to make async task
// code(core): use fn utils::handle_result to handle task
// code(core): use fn image::open to open image from path
// code(core): get image with,height,pixel_width,bytes data
// code(core): use struct sophia::ImageData to compose image-data
#[napi]
pub async fn read_image_data(path: String) -> Result<ImageData> {
  let task = tokio::spawn(async move {
    let img = match image::open(path) {
      Ok(img) => img,
      Err(e) => return Err(format!("Error: {:?}", e)),
    };

    let width = img.width();
    let height = img.height();
    let pixel_width = img.color().bytes_per_pixel();
    let data = img.as_bytes().to_vec();

    Ok(ImageData {
      data,
      width,
      height,
      pixel_width,
    })
  });

  handle_result(task).await
}

// code(core): def fn save_image_data
// code(core): use napi macro to label it
// code(core): copy the value of path and image-data
// code(core): use fn tokio::spawn to make async task
// code(core): use fn utils::handle_result to handle task

// code(core): get image with,height,pixel_width,bytes data
// code(core): use struct sophia::ImageData to compose image-data
// code(core): use fn image::RgbaImage::from_raw to transform image-data to buffer
// code(core): use fn image::DynamicImage::save to save image buffer to path

#[napi]
pub async fn save_image_data(path: String, image_data: &ImageData) -> Result<()> {
  let path = path.clone();
  let image_data = image_data.clone();

  let task = tokio::spawn(async move {
    let image_buffer =
      match image::RgbaImage::from_raw(image_data.width, image_data.height, image_data.data) {
        Some(buffer) => buffer,
        None => return Err("Failed to create image buffer".to_string()),
      };

    match image::DynamicImage::ImageRgba8(image_buffer).save(path) {
      Ok(_) => Ok(()),
      Err(e) => Err(format!("Failed to save image: {:?}", e)),
    }
  });

  handle_result(task).await
}

// code(core): def fn image_search
// code(core): use napi macro to label it
// code(core): copy the value of source and target

// code(core): use fn tokio::spawn to make async task
// code(core): use fn utils::handle_result to handle task

#[napi]
pub async fn image_search(
  source: &ImageData,
  target: &ImageData,
  variant: Option<i32>,
  trans_color: Option<Color>,
) -> Result<Option<Point>> {
  let variant = variant.unwrap_or(0);
  let source = source.clone();
  let target = target.clone();

  let task = tokio::spawn(async move {
    Ok(if let Some(trans_color) = trans_color {
      image_search_trans_inner(&source, &target, variant, trans_color)
    } else {
      image_search_inner(&source, &target, variant)
    })
  });

  handle_result(task).await
}

// code(core): def fn multiple_image_search
// code(core): use napi macro to label it
// code(core): copy the value of source and target

// code(core): use fn tokio::spawn to make async task
// code(core): use fn utils::handle_result to handle task
#[napi]
pub async fn multiple_image_search(
  source: &ImageData,
  target: &ImageData,
  variant: Option<i32>,
  trans_color: Option<Color>,
) -> Result<Vec<Point>> {
  let variant = variant.unwrap_or(0);
  let source = source.clone();
  let target = target.clone();

  let task = tokio::spawn(async move {
    Ok(if let Some(trans_color) = trans_color {
      multiple_image_search_trans_inner(&source, &target, variant, trans_color)
    } else {
      multiple_image_search_inner(&source, &target, variant)
    })
  });

  handle_result(task).await
}

fn multiple_image_search_inner(source: &ImageData, target: &ImageData, variant: i32) -> Vec<Point> {
  let source_pixels = source.data.as_slice();
  let target_pixels = target.data.as_slice();

  let source_width = source.width;
  let source_height = source.height;

  let target_width = target.width;
  let target_height = target.height;

  let source_pixel_width = source.pixel_width as u32;
  let target_pixel_width = target.pixel_width as u32;

  let source_pixel_count = source_width * source_height;
  let target_pixel_count = target_width * target_height;
  let mut points = Vec::new();

  if variant == 0 {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index];
        let source_green = source_pixels[source_index + 1];
        let source_blue = source_pixels[source_index + 2];

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index];
        let green = target_pixels[target_index + 1];
        let blue = target_pixels[target_index + 2];

        is_found = source_red == red && source_green == green && source_blue == blue;

        if !is_found {
          break;
        }
      }

      if is_found {
        points.push(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  } else {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index] as i32;
        let source_green = source_pixels[source_index + 1] as i32;
        let source_blue = source_pixels[source_index + 2] as i32;

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index] as i32;
        let green = target_pixels[target_index + 1] as i32;
        let blue = target_pixels[target_index + 2] as i32;

        let red_low = if source_red < variant {
          0
        } else {
          source_red - variant
        };
        let red_high = if source_red + variant > 255 {
          255
        } else {
          source_red + variant
        };

        let green_low = if source_green < variant {
          0
        } else {
          source_green - variant
        };
        let green_high = if source_green + variant > 255 {
          255
        } else {
          source_green + variant
        };

        let blue_low = if source_blue < variant {
          0
        } else {
          source_blue - variant
        };
        let blue_high = if source_blue + variant > 255 {
          255
        } else {
          source_blue + variant
        };

        is_found = red >= red_low
          && red <= red_high
          && green >= green_low
          && green <= green_high
          && blue >= blue_low
          && blue <= blue_high;

        if !is_found {
          break;
        }
      }

      if is_found {
        points.push(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  }

  points
}

fn image_search_inner(source: &ImageData, target: &ImageData, variant: i32) -> Option<Point> {
  let source_pixels = source.data.as_slice();
  let target_pixels = target.data.as_slice();

  let source_width = source.width;
  let source_height = source.height;

  let target_width = target.width;
  let target_height = target.height;

  let source_pixel_width = source.pixel_width as u32;
  let target_pixel_width = target.pixel_width as u32;

  let source_pixel_count = source_width * source_height;
  let target_pixel_count = target_width * target_height;

  if variant == 0 {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index];
        let source_green = source_pixels[source_index + 1];
        let source_blue = source_pixels[source_index + 2];

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index];
        let green = target_pixels[target_index + 1];
        let blue = target_pixels[target_index + 2];

        is_found = source_red == red && source_green == green && source_blue == blue;

        if !is_found {
          break;
        }
      }

      if is_found {
        return Some(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  } else {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index] as i32;
        let source_green = source_pixels[source_index + 1] as i32;
        let source_blue = source_pixels[source_index + 2] as i32;

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index] as i32;
        let green = target_pixels[target_index + 1] as i32;
        let blue = target_pixels[target_index + 2] as i32;

        let red_low = if source_red < variant {
          0
        } else {
          source_red - variant
        };
        let red_high = if source_red + variant > 255 {
          255
        } else {
          source_red + variant
        };

        let green_low = if source_green < variant {
          0
        } else {
          source_green - variant
        };
        let green_high = if source_green + variant > 255 {
          255
        } else {
          source_green + variant
        };

        let blue_low = if source_blue < variant {
          0
        } else {
          source_blue - variant
        };
        let blue_high = if source_blue + variant > 255 {
          255
        } else {
          source_blue + variant
        };

        is_found = red >= red_low
          && red <= red_high
          && green >= green_low
          && green <= green_high
          && blue >= blue_low
          && blue <= blue_high;

        if !is_found {
          break;
        }
      }

      if is_found {
        return Some(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  }

  None
}

fn image_search_trans_inner(
  source: &ImageData,
  target: &ImageData,
  variant: i32,
  trans_color: Color,
) -> Option<Point> {
  let source_pixels = source.data.as_slice();
  let target_pixels = target.data.as_slice();

  let source_width = source.width;
  let source_height = source.height;

  let target_width = target.width;
  let target_height = target.height;

  let source_pixel_width = source.pixel_width as u32;
  let target_pixel_width = target.pixel_width as u32;

  let source_pixel_count = source_width * source_height;
  let target_pixel_count = target_width * target_height;

  if variant == 0 {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index];
        let source_green = source_pixels[source_index + 1];
        let source_blue = source_pixels[source_index + 2];

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index];
        let green = target_pixels[target_index + 1];
        let blue = target_pixels[target_index + 2];

        is_found = (trans_color.r == red && trans_color.g == green && trans_color.b == blue)
          || (source_red == red && source_green == green && source_blue == blue);

        if !is_found {
          break;
        }
      }

      if is_found {
        return Some(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  } else {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index] as i32;
        let source_green = source_pixels[source_index + 1] as i32;
        let source_blue = source_pixels[source_index + 2] as i32;

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index] as i32;
        let green = target_pixels[target_index + 1] as i32;
        let blue = target_pixels[target_index + 2] as i32;

        if trans_color.r == red as u8 && trans_color.g == green as u8 && trans_color.b == blue as u8
        {
          continue;
        }

        let red_low = if source_red < variant {
          0
        } else {
          source_red - variant
        };
        let red_high = if source_red + variant > 255 {
          255
        } else {
          source_red + variant
        };

        let green_low = if source_green < variant {
          0
        } else {
          source_green - variant
        };
        let green_high = if source_green + variant > 255 {
          255
        } else {
          source_green + variant
        };

        let blue_low = if source_blue < variant {
          0
        } else {
          source_blue - variant
        };
        let blue_high = if source_blue + variant > 255 {
          255
        } else {
          source_blue + variant
        };

        is_found = red >= red_low
          && red <= red_high
          && green >= green_low
          && green <= green_high
          && blue >= blue_low
          && blue <= blue_high;

        if !is_found {
          break;
        }
      }

      if is_found {
        return Some(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  }

  None
}

fn multiple_image_search_trans_inner(
  source: &ImageData,
  target: &ImageData,
  variant: i32,
  trans: Color,
) -> Vec<Point> {
  let source_pixels = source.data.as_slice();
  let target_pixels = target.data.as_slice();

  let source_width = source.width;
  let source_height = source.height;

  let target_width = target.width;
  let target_height = target.height;

  let source_pixel_width = source.pixel_width as u32;
  let target_pixel_width = target.pixel_width as u32;

  let source_pixel_count = source_width * source_height;
  let target_pixel_count = target_width * target_height;
  let mut points = Vec::new();

  if variant == 0 {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index];
        let source_green = source_pixels[source_index + 1];
        let source_blue = source_pixels[source_index + 2];

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index];
        let green = target_pixels[target_index + 1];
        let blue = target_pixels[target_index + 2];

        is_found = (trans.r == red && trans.g == green && trans.b == blue)
          || source_red == red && source_green == green && source_blue == blue;

        if !is_found {
          break;
        }
      }

      if is_found {
        points.push(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  } else {
    for i in 0..source_pixel_count {
      let sx = i % source_width;
      let sy = i / source_width;

      if sx + target_width > source_width || sy + target_height > source_height {
        continue;
      }

      let mut is_found = true;

      for j in 0..target_pixel_count {
        let tx = j % target_width;
        let ty = j / target_width;

        let x = sx + tx;
        let y = sy + ty;

        let source_index = ((y * source_width + x) * source_pixel_width) as usize;
        let source_red = source_pixels[source_index] as i32;
        let source_green = source_pixels[source_index + 1] as i32;
        let source_blue = source_pixels[source_index + 2] as i32;

        let target_index = (j * target_pixel_width) as usize;

        let red = target_pixels[target_index] as i32;
        let green = target_pixels[target_index + 1] as i32;
        let blue = target_pixels[target_index + 2] as i32;

        if trans.r == red as u8 && trans.g == green as u8 && trans.b == blue as u8 {
          continue;
        }

        let red_low = if source_red < variant {
          0
        } else {
          source_red - variant
        };
        let red_high = if source_red + variant > 255 {
          255
        } else {
          source_red + variant
        };

        let green_low = if source_green < variant {
          0
        } else {
          source_green - variant
        };
        let green_high = if source_green + variant > 255 {
          255
        } else {
          source_green + variant
        };

        let blue_low = if source_blue < variant {
          0
        } else {
          source_blue - variant
        };
        let blue_high = if source_blue + variant > 255 {
          255
        } else {
          source_blue + variant
        };

        is_found = red >= red_low
          && red <= red_high
          && green >= green_low
          && green <= green_high
          && blue >= blue_low
          && blue <= blue_high;

        if !is_found {
          break;
        }
      }

      if is_found {
        points.push(Point {
          x: sx as i32,
          y: sy as i32,
        });
      }
    }
  }

  points
}
