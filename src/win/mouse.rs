use crate::geometry::Point;
use crate::utils::handle_result;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::UI::Input::KeyboardAndMouse::{
  MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
  MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
  MOUSE_EVENT_FLAGS,
};
use windows::Win32::UI::WindowsAndMessaging::{
  GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
};

// code(core): def enum MouseButton
// code(core): use napi(object) macro to label it
// code(core): with Left,Right,Middle prop and value (0,1,2)

#[napi]
pub enum MouseButton {
  Left,
  Right,
  Middle,
}

// code(core): def struct Mouse
// code(core): use napi macro to label it

#[napi]
pub struct Mouse {}

// code(core): impl Mouse
// code(core): use napi macro to label it

#[napi]
impl Mouse {
  // code(core): impl struct Mouse with a method mouse_move
  // code(core): use napi macro to label it and use move as js-name
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  #[napi(js_name = "move")]
  pub async fn mouse_move(x: i32, y: i32) -> Result<()> {
    let task = tokio::spawn(async move {
      mouse_move_inner(x, y);

      Ok(())
    });

    handle_result(task).await
  }
  // code(core): impl struct Mouse with a method press
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  #[napi]
  pub async fn press(button: MouseButton) -> Result<()> {
    let task = tokio::spawn(async move {
      let down = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
        MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN,
      };

      mouse_event(down, 0, 0, 0, 0);

      Ok(())
    });

    handle_result(task).await
  }
  // code(core): impl struct Mouse with a method release
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  #[napi]
  pub async fn release(button: MouseButton) -> Result<()> {
    let task = tokio::spawn(async move {
      let up = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTUP,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
      };

      mouse_event(up, 0, 0, 0, 0);

      Ok(())
    });

    handle_result(task).await
  }
  // code(core): impl struct Mouse with a method click
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  #[napi]
  pub async fn click(button: MouseButton, x: i32, y: i32) -> Result<()> {
    let task = tokio::spawn(async move {
      let (down, up) = match button {
        MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
        MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
        MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
      };

      mouse_move_inner(x, y);
      mouse_event(down, x, y, 0, 0);
      mouse_event(up, x, y, 0, 0);

      Ok(())
    });

    handle_result(task).await
  }
  // code(core): impl struct Mouse with a method get_position
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn utils::handle_result to handle task
  #[napi]
  pub async fn get_position() -> Result<Point> {
    let task = tokio::spawn(async move { Ok(get_mouse_position_inner()) });

    handle_result(task).await
  }
}

// code(core): def inner fn get_mouse_position_inner to get mouse position
// code(core): use struct windows::Win32::Foundation::POINT to set mouse initial value
// code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetCursorPos to get mouse position
// code(core): use fn sophia::geometry::Point::new to make point and as result

fn get_mouse_position_inner() -> Point {
  let mut position = windows::Win32::Foundation::POINT { x: 0, y: 0 };
  unsafe {
    let _ = GetCursorPos(&mut position);
  }

  Point::new(position.x, position.y)
}

// code(core): def inner fn mouse_event to handle mouse envet
// code(core): use const windows::Win32::UI::Input::KeyboardAndMouse::MOUSE_EVENT_FLAGS
// code(core): use const windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN
// code(core): use const windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN
// code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics
// code(core): use fn  windows::Win32::UI::Input::KeyboardAndMouse::mouse_event
fn mouse_event(dw_flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, dw_data: i32, dw_extra_info: usize) {
  unsafe {
    let x = dx * 65536 / GetSystemMetrics(SM_CXSCREEN);
    let y = dy * 65536 / GetSystemMetrics(SM_CYSCREEN);
    windows::Win32::UI::Input::KeyboardAndMouse::mouse_event(
      dw_flags,
      x,
      y,
      dw_data,
      dw_extra_info,
    );
  }
}

// code(core): def inner fn mouse_move_inner to move mouse
// code(core): use const windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_MOVE
// code(core): use const windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE

fn mouse_move_inner(x: i32, y: i32) {
  mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x, y, 0, 0);
}
