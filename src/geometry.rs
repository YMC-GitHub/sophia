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
}

impl Rect {
  pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
    Self {
      left,
      top,
      right,
      bottom,
    }
  }
}
