use napi_derive::napi;

// code(core): def struct Point
// code(core): use napi(object) macro to label struct Point
// code(core): use derive(Debug, Clone) macro to label struct Point
// code(core): impl struct Point with a method new

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}

// code(core): def struct Rect
// code(core): use napi(object) macro to label struct Rect
// code(core): use derive(Debug, Clone) macro to label struct Rect
// code(core): impl struct Rect with a method new

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Rect {
  pub left: i32,
  pub top: i32,
  pub right: i32,
  pub bottom: i32,
  pub width: i32,
  pub height: i32,
}

impl Rect {
  pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
    Self {
      left,
      top,
      right,
      bottom,
      width: right - left,
      height: bottom - top,
    }
  }
  pub fn default() -> Self {
    Self::new(0, 0, 255, 255)
  }
}

// code(core): def struct WindowView
// code(core): use napi(object) macro to label it
// code(core): use derive(Debug, Clone) macro to label it
// code(core): impl struct WindowView with a method new

#[napi(object)]
#[derive(Debug, Clone)]
pub struct WindowView {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl WindowView {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }
  pub fn default() -> Self {
    Self::new(0, 0, 255, 255)
  }
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct WindowMetaInfo {
  pub id: u32,
  pub title: String,
  pub class_name: String,
  pub rect: Rect,
}

// - code(core): def struct FindWindowProp
// - code(core): use napi(object) macro to label it
// - code(core): use derive(Debug, Clone) macro to label it
// - code(core): with prop psid,name,title,class_name
// - code(core): impl struct FindWindowProp with a method new
// #[napi(object)]
// #[derive(Debug, Clone)]
// pub struct FindWindowProp {
//   pub psid: i32,
//   pub name: String,
//   pub title: String,
//   pub class_name: String,
// }

// impl FindWindowProp {
//   pub fn new(psid: i32, name: String, title: String, class_name: String) -> Self {
//     Self {
//       psid,
//       name,
//       title,
//       class_name,
//     }
//   }
//   pub fn default() -> Self {
//     Self::new(-1 as i32, "".to_string(), "".to_string(), "".to_string())
//   }
// }

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LParamFlag {
  pub scan_code: u32,
  pub repeat_count: u32,
  pub transition_state: bool,
  pub is_extended: bool,
  pub previous_key_state: bool,
  pub context_code: bool,
}

// lparam_from_isize(lparam_isize_from_flag(key, 1, false, false, false))
