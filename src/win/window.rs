use crate::geometry::Rect;
use crate::utils::{decode_wide, encode_wide, handle_result};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT, TRUE};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{
  EnumChildWindows, FindWindowW, GetClassNameW, GetClientRect, GetDesktopWindow,
  GetForegroundWindow, GetWindowLongPtrW, GetWindowTextLengthW, GetWindowTextW,
  GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, SetForegroundWindow, SetWindowPos,
  ShowWindow, ShowWindowAsync, GWL_EXSTYLE, GWL_STYLE, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD,
  SWP_NOMOVE, SWP_NOSIZE, SW_MAXIMIZE, SW_MINIMIZE, SW_SHOWNORMAL, WS_CHILD, WS_EX_TOOLWINDOW,
};

// https://itecnotes.com/tecnote/c-how-to-verify-if-a-window-of-another-program-is-minimized

use crate::screen::ImageData;
// use windows::Win32::Graphics::Gdi::{
//   BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
//   ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
// };

use std::ptr;

// use win_screenshot::prelude::*;
// use win_screenshot::capture::capture_window;

// pub async fn get_window_rect(hwnd: HWND) -> Result<Rect> {
//   let task = tokio::spawn(async move {
//     let mut rect = windows::Win32::Foundation::RECT::default();

//     unsafe {
//       let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
//     }
//     Ok(Rect::new(rect.left, rect.top, rect.right, rect.bottom))
//   });

//   handle_result(task).await
// }

pub fn get_window_rect_sync(hwnd: HWND) -> Rect {
  let mut rect = windows::Win32::Foundation::RECT::default();

  unsafe {
    let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
  }
  Rect::new(rect.left, rect.top, rect.right, rect.bottom)
}

// use active_win_pos_rs::get_active_window;
// code(core): def struct Window
// code(core): use napi macro to label it
// code(core): with hwnd prop
// code(core): use struct windows::Win32::Foundation::HWND

#[napi]
pub struct Window {
  hwnd: HWND,
}

// feat(core): window minimize/maximize
// feat(core): window get title
// feat(core): window get class name
// feat(core): window get window rect
// feat(core): window set_position
// feat(core): window set_size
// feat(core): window is_foreground
// feat(core): window set_foreground
// feat(core): window is_open
// feat(core): window is_minimized
// feat(core): window is_visible
// feat(core): window show
// feat(core): get_foreground_window
// feat(core): find_window_by_title
// feat(core): window get_id
// feat(core): from_name
// feat(core): from_contains_name
// feat(core): find_window_by_class_name
// feat(core): window capture
// feat(core): window capture rect

// feat(core): get_windows
// feat(core): get_window_by_name
// feat(core): from_contains_name
// feat(core): get_foreground_window
// feat(core): find_window_by_class_name

// code(core): impl Window
// code(core): use napi macro to label it

#[napi]
impl Window {
  // code(core): impl struct Window with a method minimize
  // code(core): use napi macro to label it
  // code(core): use inner fn self.show_window
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_MINIMIZE
  #[napi]
  pub async fn minimize(&self) -> Result<()> {
    self.show_window(SW_MINIMIZE).await
  }

  // code(core): impl struct Window with a method maximize
  // code(core): use napi macro to label it
  // code(core): use inner fn self.show_window
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_MINIMIZE
  #[napi]
  pub async fn maximize(&self) -> Result<()> {
    self.show_window(SW_MAXIMIZE).await
  }

  // code(core): impl struct Window with a method get_title
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetWindowTextLengthW
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetWindowTextW
  // code(core): use fn sophia::utils::decode_wide

  #[napi]
  pub async fn get_title(&self) -> Result<String> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      unsafe {
        let len = GetWindowTextLengthW(hwnd);
        let mut buffer = vec![0u16; len as usize + 1];
        GetWindowTextW(hwnd, &mut buffer);
        Ok(decode_wide(&buffer))
      }
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method get_class_name
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetClassNameW
  // code(core): use fn sophia::utils::decode_wide

  #[napi]
  pub async fn get_class_name(&self) -> Result<String> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      unsafe {
        let mut buffer = vec![0u16; 256 as usize];
        GetClassNameW(hwnd, &mut buffer);
        Ok(decode_wide(&buffer))
      }
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method get_window_rect
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use fn windows::Win32::Foundation::RECT::default
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetWindowRect
  // code(core): use struct sophia::geometry::Rect

  #[napi]
  pub async fn get_window_rect(&self) -> Result<Rect> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      let mut rect = windows::Win32::Foundation::RECT::default();

      unsafe {
        let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
      }

      // Ok(Rect {
      //   left: rect.left,
      //   top: rect.top,
      //   right: rect.right,
      //   bottom: rect.bottom,
      // })
      Ok(Rect::new(rect.left, rect.top, rect.right, rect.bottom))
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method inner fn show_window
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::ShowWindow
  async fn show_window(&self, state: SHOW_WINDOW_CMD) -> Result<()> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      unsafe {
        ShowWindow(hwnd, state);
      }

      Ok(())
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method set_position
  // code(core): use napi macro to label it
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE
  // code(core): use inner fn self.set_window_pos

  #[napi]
  pub async fn set_position(&self, x: i32, y: i32) -> Result<()> {
    self.set_window_pos(x, y, 0, 0, SWP_NOSIZE).await
  }

  // code(core): impl struct Window with a method set_size
  // code(core): use napi macro to label it
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
  // code(core): use inner fn self.set_window_pos

  #[napi]
  pub async fn set_size(&self, width: i32, height: i32) -> Result<()> {
    self.set_window_pos(0, 0, width, height, SWP_NOMOVE).await
  }

  // code(core): impl struct Window with a method is_foreground
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task

  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow
  #[napi]
  pub async fn is_foreground(&self) -> Result<bool> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      let res = unsafe { GetForegroundWindow() };

      Ok(res == hwnd)
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method foreground
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::ShowWindowAsync
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow
  #[napi]
  pub async fn foreground(&self) -> Result<bool> {
    let hwnd = self.hwnd;

    let task: tokio::task::JoinHandle<std::result::Result<bool, String>> =
      tokio::spawn(async move {
        unsafe {
          let _ = ShowWindowAsync(hwnd, SW_SHOWNORMAL);
        };
        let res = unsafe { SetForegroundWindow(hwnd) };

        Ok(res.0 != 0)
      });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method set_foreground
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::ShowWindowAsync
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow
  #[napi]
  pub async fn set_foreground(&self) -> Result<bool> {
    let hwnd = self.hwnd;

    let task: tokio::task::JoinHandle<std::result::Result<bool, String>> =
      tokio::spawn(async move {
        unsafe {
          let _ = ShowWindowAsync(hwnd, SW_SHOWNORMAL);
        };
        let res = unsafe { SetForegroundWindow(hwnd) };

        Ok(res.0 != 0)
      });

    handle_result(task).await
  }

  #[napi]
  pub async fn is_open(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      if !unsafe { IsWindow(hwnd).as_bool() } {
        return Ok(false);
      }
      return Ok(true);
    });

    handle_result(task).await
  }
  #[napi]
  pub async fn is_minimized(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      if !unsafe { IsIconic(hwnd).as_bool() } {
        return Ok(false);
      }
      return Ok(true);
    });

    handle_result(task).await
  }
  // code(core): impl struct Window with a method show
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::ShowWindowAsync
  #[napi]
  pub async fn show(&self) -> Result<bool> {
    let hwnd = self.hwnd;

    let task: tokio::task::JoinHandle<std::result::Result<bool, String>> =
      tokio::spawn(async move {
        if !unsafe { ShowWindowAsync(hwnd, SW_SHOWNORMAL).as_bool() } {
          return Ok(false);
        }
        return Ok(true);
      });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method inner fn set_window_pos
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SET_WINDOW_POS_FLAGS
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::SetWindowPos
  async fn set_window_pos(
    &self,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    flags: SET_WINDOW_POS_FLAGS,
  ) -> Result<()> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      unsafe {
        let _ = SetWindowPos(hwnd, None, x, y, width, height, flags);
      }

      Ok(())
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method get_foreground_window
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow
  #[napi]
  pub async fn get_foreground_window() -> Result<Option<Window>> {
    let task = tokio::spawn(async move {
      let hwnd = unsafe { GetForegroundWindow() };

      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method find_window_by_title
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use fn sophia::utils::encode_wide to transform str to buf
  // code(core): use struct windows_core::strings::pcwstr::PCWSTR
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::FindWindowW

  #[napi]
  pub async fn find_window_by_title(title: String) -> Result<Option<Window>> {
    let task = tokio::spawn(async move {
      let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };

      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });

    handle_result(task).await
  }

  #[napi]
  pub async fn enumerate(&self) -> Result<Vec<Window>> {
    // let hwnd = self.hwnd;
    let mut windows: Vec<Self> = Vec::new();

    let task = tokio::spawn(async move {
      // let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };
      // if hwnd.0 == 0 {
      //   Ok(None)
      // } else {
      //   Ok(Some(Window { hwnd }))
      // }
      unsafe {
        EnumChildWindows(
          GetDesktopWindow(),
          Some(Self::enum_windows_callback),
          LPARAM(ptr::addr_of_mut!(windows) as isize),
        )
      };

      Ok(windows)
    });

    handle_result(task).await
  }

  pub const fn from_raw_hwnd(hwnd: isize) -> Self {
    Self { hwnd: HWND(hwnd) }
  }
  // Callback used for enumerating all windows.
  unsafe extern "system" fn enum_windows_callback(window: HWND, vec: LPARAM) -> BOOL {
    let windows = &mut *(vec.0 as *mut Vec<Self>);

    if Self::from_raw_hwnd(window.0).is_valid() {
      windows.push(Self { hwnd: window });
    }

    TRUE
  }

  #[napi]
  pub async fn is_visible(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      if !unsafe { IsWindowVisible(hwnd).as_bool() } {
        return Ok(false);
      }
      return Ok(true);
    });

    handle_result(task).await
  }

  #[napi]
  pub async fn get_id(&self) -> Result<u32> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      let mut id: u32 = 0;
      unsafe { GetWindowThreadProcessId(hwnd, Some(&mut id)) };
      let id2: u32 = unsafe { GetCurrentProcessId() };
      if id == id2 {
        return Ok(id2);
      }
      return Ok(id);
    });

    handle_result(task).await
  }
  /// Checks if the window is a valid window.
  ///
  /// # Returns
  ///
  /// Returns `true` if the window is valid, `false` otherwise.
  pub fn is_valid(&self) -> bool {
    if !unsafe { IsWindowVisible(self.hwnd).as_bool() } {
      return false;
    }

    //
    let mut id: u32 = 0;
    unsafe { GetWindowThreadProcessId(self.hwnd, Some(&mut id)) };
    if id == unsafe { GetCurrentProcessId() } {
      return false;
    }

    let mut rect = RECT::default();
    let result = unsafe { GetClientRect(self.hwnd, &mut rect) };
    if result.is_ok() {
      let styles = unsafe { GetWindowLongPtrW(self.hwnd, GWL_STYLE) };
      let ex_styles = unsafe { GetWindowLongPtrW(self.hwnd, GWL_EXSTYLE) };

      if (ex_styles & isize::try_from(WS_EX_TOOLWINDOW.0).unwrap()) != 0 {
        return false;
      }
      if (styles & isize::try_from(WS_CHILD.0).unwrap()) != 0 {
        return false;
      }
    } else {
      return false;
    }

    true
  }

  // https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#L223
  #[napi]
  pub async fn as_raw_hwnd(&self) -> Result<Option<Window>> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      // let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };
      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });

    handle_result(task).await
  }

  // https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#L77
  /// Creates a `Window` instance from a window name.
  ///
  /// # Arguments
  ///
  /// * `title` - The name of the window.
  ///
  /// # Returns
  ///
  /// Returns `None` if the window is not found.
  #[napi]
  pub async fn from_name(title: String) -> Result<Option<Window>> {
    let task = tokio::spawn(async move {
      let hstring_title = HSTRING::from(title);
      let hwnd = unsafe { FindWindowW(None, &hstring_title) };

      // let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };

      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });

    handle_result(task).await
  }

  /// Creates a `Window` instance from a window name substring.
  ///
  /// # Arguments
  ///
  /// * `title` - The substring to search for in window names.
  ///
  /// # Returns
  ///
  /// Returns `None` if the window is not found.
  #[napi]
  pub async fn from_contains_name(title: String) -> Result<Option<Window>> {
    let task = tokio::spawn(async move {
      let hstring_title = HSTRING::from(title);
      let hwnd = unsafe { FindWindowW(None, &hstring_title) };

      // let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };

      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });

    handle_result(task).await
  }

  // code(core): impl struct Window with a method find_window_by_class_name
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use fn sophia::utils::encode_wide to transform str to buf
  // code(core): use struct windows_core::strings::pcwstr::PCWSTR
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::FindWindowW

  #[napi]
  pub async fn find_window_by_class_name(classname: String) -> Result<Option<Window>> {
    let task = tokio::spawn(async move {
      let hwnd = unsafe { FindWindowW(PCWSTR(encode_wide(classname).as_ptr()), None) };

      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });

    handle_result(task).await
  }

  // code(core): def fn capture
  // code(core): use napi macro to label it

  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  // code(core): use fn win_screenshot::capture to capture window
  #[napi]
  pub async fn capture(&self) -> Result<ImageData> {
    let hwnd = self.hwnd;
    // if unsafe { IsIconic(hwnd).as_bool() } {
    // set window to unminmized
    //   unsafe {
    //     let _ = ShowWindowAsync(hwnd, SW_SHOWNORMAL);
    //   };
    //  active
    //   unsafe { SetForegroundWindow(hwnd) };
    // }

    let task = tokio::spawn(async move {
      // let hwnd = GetDesktopWindow();

      let buf = win_screenshot::capture::capture_window(hwnd.0).unwrap();
      let w: u32 = buf.width;
      let h: u32 = buf.height;

      Ok(ImageData {
        data: buf.pixels,
        width: buf.width,
        height: buf.height,
        pixel_width: (4 * w * h) as u8,
      })
    });

    handle_result(task).await
  }

  #[napi]
  pub async fn capture_area(&self, x: i32, y: i32, width: i32, height: i32) -> Result<ImageData> {
    let hwnd = self.hwnd;

    // let rect = get_window_rect_sync(hwnd);

    let task = tokio::spawn(async move {
      // let hwnd = GetDesktopWindow();
      // PrintWindow much slower, much more reliable
      let using = win_screenshot::capture::Using::PrintWindow;

      // Capture client area of window
      let area = win_screenshot::capture::Area::ClientOnly;
      // Capture whole window (not supported with BitBlt)
      // let area = Area::Full;

      // Build-in crop, faster on large windows
      // let crop_xy = None; //Some([100, 100]);
      // let crop_wh = None; //Some([300, 300]);
      let crop_xy = Some([x, y]);
      let crop_wh = Some([width, height]);

      let buf =
        win_screenshot::capture::capture_window_ex(hwnd.0, using, area, crop_xy, crop_wh).unwrap();
      let w: u32 = buf.width;
      let h: u32 = buf.height;

      Ok(ImageData {
        data: buf.pixels,
        width: buf.width,
        height: buf.height,
        pixel_width: (4 * w * h) as u8,
      })
    });

    handle_result(task).await
  }
}

// https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#L223
pub const fn from_raw_hwnd(hwnd: isize) -> Window {
  Window { hwnd: HWND(hwnd) }
}
unsafe extern "system" fn enum_windows_callback(window: HWND, vec: LPARAM) -> BOOL {
  let windows = &mut *(vec.0 as *mut Vec<Window>);

  if from_raw_hwnd(window.0).is_valid() {
    windows.push(Window { hwnd: window });
  }

  TRUE
}

// todo: make alias get_all_windows
#[napi]
pub async fn get_windows() -> Result<Vec<Window>> {
  // let hwnd = self.hwnd;
  let mut windows: Vec<Window> = Vec::new();

  let task = tokio::spawn(async move {
    unsafe {
      EnumChildWindows(
        GetDesktopWindow(),
        Some(enum_windows_callback),
        LPARAM(ptr::addr_of_mut!(windows) as isize),
      )
    };

    Ok(windows)
  });

  handle_result(task).await
}

// https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#L77
/// Creates a `Window` instance from a window name.
///
/// # Arguments
///
/// * `title` - The name of the window.
///
/// # Returns
///
/// Returns `None` if the window is not found.
#[napi]
pub async fn get_window_by_name(title: String) -> Result<Option<Window>> {
  let task = tokio::spawn(async move {
    let hstring_title = HSTRING::from(title);
    let hwnd = unsafe { FindWindowW(None, &hstring_title) };

    // let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };

    if hwnd.0 == 0 {
      Ok(None)
    } else {
      Ok(Some(Window { hwnd }))
    }
  });

  handle_result(task).await
}

/// Creates a `Window` instance from a window name substring.
///
/// # Arguments
///
/// * `title` - The substring to search for in window names.
///
/// # Returns
///
/// Returns `None` if the window is not found.
#[napi]
pub async fn from_contains_name(title: String) -> Result<Option<Window>> {
  let task = tokio::spawn(async move {
    let hstring_title = HSTRING::from(title);
    let hwnd = unsafe { FindWindowW(None, &hstring_title) };

    // let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };

    if hwnd.0 == 0 {
      Ok(None)
    } else {
      Ok(Some(Window { hwnd }))
    }
  });

  handle_result(task).await
}

// code(core): impl struct Window with a method find_window_by_class_name
// code(core): use napi macro to label it
// code(core): use fn tokio::spawn to make async task
// code(core): use fn sophia::utils::handle_result to handle task
// code(core): use fn sophia::utils::encode_wide to transform str to buf
// code(core): use struct windows_core::strings::pcwstr::PCWSTR
// code(core): use fn windows::Win32::UI::WindowsAndMessaging::FindWindowW

#[napi]
pub async fn find_window_by_class_name(classname: String) -> Result<Option<Window>> {
  let task = tokio::spawn(async move {
    let hwnd = unsafe { FindWindowW(PCWSTR(encode_wide(classname).as_ptr()), None) };

    if hwnd.0 == 0 {
      Ok(None)
    } else {
      Ok(Some(Window { hwnd }))
    }
  });

  handle_result(task).await
}

// code(core): impl struct Window with a method get_foreground_window
// code(core): use napi macro to label it
// code(core): use fn tokio::spawn to make async task
// code(core): use fn sophia::utils::handle_result to handle task
// code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow
#[napi]
pub async fn get_foreground_window() -> Result<Option<Window>> {
  let task = tokio::spawn(async move {
    let hwnd = unsafe { GetForegroundWindow() };

    if hwnd.0 == 0 {
      Ok(None)
    } else {
      Ok(Some(Window { hwnd }))
    }
  });

  handle_result(task).await
}

// FindWindowProp
// relaive project or code:
// https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs
