use crate::geometry::Rect;
use crate::utils::{decode_wide, encode_wide, handle_result};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
  FindWindowW, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, SetForegroundWindow,
  SetWindowPos, ShowWindow, ShowWindowAsync, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, SWP_NOMOVE,
  SWP_NOSIZE, SW_MAXIMIZE, SW_MINIMIZE, SW_SHOWNORMAL,
};

// code(core): def struct Window
// code(core): use napi macro to label it
// code(core): with hwnd prop
// code(core): use struct windows::Win32::Foundation::HWND

#[napi]
pub struct Window {
  hwnd: HWND,
}

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

      Ok(Rect {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
      })
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

    let task = tokio::spawn(async move {
      unsafe {
        let _ = ShowWindowAsync(hwnd, SW_SHOWNORMAL);
      };

      let res = unsafe { SetForegroundWindow(hwnd) };

      Ok(res.0 != 0)
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
}
