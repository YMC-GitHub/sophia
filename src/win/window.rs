use crate::geometry::{Point, Rect, WindowMetaInfo, WindowView};
use crate::screen::ImageData;
use crate::utils::handle_result;
use crate::win::utils::{
  close_hwnd, coords_move, get_hwnd_by_class_name, get_hwnd_by_title_hstring, get_hwnd_class_name,
  get_hwnd_meta_info, get_hwnd_pid, get_hwnd_rect, get_hwnd_title_next, get_hwnd_view,
  get_mouse_position_in_window, is_foreground_hwnd, is_minimize_hwnd, is_open_hwnd, kill_hwnd,
  list_hwnd, mouse_move_in_window_inner, mouse_toggle_in_window_inner, set_active_hwnd,
  set_hwnd_pos, show_hwnd,
};

use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::Foundation::HWND;
// get window title with GetWindowTextLengthW, GetWindowTextW,
// get window class name with GetClassNameW
// set window hide or show with ShowWindow,SHOW_WINDOW_CMD
// set foreground window with SetForegroundWindow
use windows::Win32::UI::WindowsAndMessaging::{
  GetForegroundWindow, IsWindowVisible, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, SWP_NOMOVE,
  SWP_NOSIZE, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_SHOWNOACTIVATE,
};
// [hide window with ShowWindow or SendMessage in cpp](https://blog.csdn.net/u012486325/article/details/71732362)

// [is-minimized in cpp](https://itecnotes.com/tecnote/c-how-to-verify-if-a-window-of-another-program-is-minimized)

// [about findwindowexa in cpp](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexa)

// use std::ptr;

// use win_screenshot::prelude::*;
// use win_screenshot::capture::capture_window;

async fn set_active_hwnd_async(hwnd: HWND) -> Result<bool> {
  // let hwnd = self.hwnd;
  let task: tokio::task::JoinHandle<std::result::Result<bool, String>> =
    tokio::spawn(async move { Ok(set_active_hwnd(hwnd)) });
  handle_result(task).await
}

async fn set_hwnd_pos_async(
  hwnd: HWND,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  flags: SET_WINDOW_POS_FLAGS,
) -> Result<()> {
  // let hwnd = self.hwnd;

  let task = tokio::spawn(async move {
    set_hwnd_pos(hwnd, x, y, width, height, flags);

    Ok(())
  });

  handle_result(task).await
}

// code(core): def inner fn show_hwnd_async
// code(core): use fn tokio::spawn to make async task
// code(core): use fn sophia::utils::handle_result to handle task
// code(core): use const windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD
// code(core): use fn windows::Win32::UI::WindowsAndMessaging::ShowWindow
async fn show_hwnd_async(hwnd: HWND, state: SHOW_WINDOW_CMD) -> Result<()> {
  // let hwnd = self.hwnd;
  let task = tokio::spawn(async move {
    show_hwnd(hwnd, state);
    Ok(())
  });

  handle_result(task).await
}

// set module method
// feat(core): get_windows
// feat(core): get_window_by_name
// feat(core): from_contains_name
// feat(core): get_foreground_window
// feat(core): find_window_by_class_name

// set class static method
// feat(core): get_foreground_window
// feat(core): find_window_by_title
// feat(core): window get_id
// feat(core): from_name
// feat(core): from_contains_name
// feat(core): find_window_by_class_name
// feat(core): window capture
// feat(core): window capture rect

// set class public method
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

// code(core): def struct Window
// code(core): use napi macro to label it
// code(core): with hwnd prop
// code(core): use struct windows::Win32::Foundation::HWND

#[napi]
pub struct Window {
  hwnd: HWND,
  // last_coords: Point,
}
// code(core): impl Window
// code(core): use napi macro to label it

// [as raw HWND of the window and hwnd id](https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#L223)
// [from raw HWND of the window and hwnd id](https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#217)

///
/// NOTE
///
/// hwnd id vs pid, is the same ?
pub fn get_window_from_hwnd_id(hwnd: isize) -> Window {
  Window { hwnd: HWND(hwnd) }
}
pub fn get_window_from_hwnd(hwnd: HWND) -> Window {
  Window { hwnd }
}
///
/// NOTE
///
/// list_hwnd,get_window_from_hwnd
pub fn list_window_inner() -> Vec<Window> {
  list_hwnd()
    .iter()
    .map(|x| get_window_from_hwnd(*x))
    .collect()
}

/// list -> find
pub fn find_window_by_pid_inner(pid: u32) -> Window {
  let windows = list_window_inner();
  let window: &Window = windows
    .iter()
    .find(|i| get_hwnd_pid(i.hwnd) == pid)
    .unwrap();
  Window { hwnd: window.hwnd }
}

pub fn find_window_contains_title_inner(title: String) -> Window {
  let windows = list_window_inner();
  let window: &Window = windows
    .iter()
    .find(|i| get_hwnd_title_next(i.hwnd).contains(&title))
    .unwrap();
  Window { hwnd: window.hwnd }
}

pub fn find_window_contains_class_name_inner(name: String) -> Window {
  let windows = list_window_inner();
  let window: &Window = windows
    .iter()
    .find(|i| get_hwnd_class_name(i.hwnd).contains(&name))
    .unwrap();
  Window { hwnd: window.hwnd }
}

// todo: make alias get_all_windows
// todo: list_window_sync,list_window_async
#[napi]
pub async fn list_window() -> Result<Vec<Window>> {
  let task = tokio::spawn(async move { Ok(list_window_inner()) });
  handle_result(task).await
}

/// alias of list_window
#[napi]
pub async fn get_all_windows() -> Result<Vec<Window>> {
  list_window().await
}

// code(core): def pub fn get_foreground_window
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

/// create a Window instance with pid
#[napi]
pub async fn find_window_by_pid(pid: u32) -> Result<Option<Window>> {
  let task: tokio::task::JoinHandle<std::result::Result<Option<Window>, String>> =
    tokio::spawn(async move {
      // let hwnd = unsafe { GetForegroundWindow() };

      let window = find_window_by_pid_inner(pid);

      let hwnd = window.hwnd;

      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });
  handle_result(task).await

  // let task = tokio::spawn(async move { Ok(Some(find_window_by_pid_inner(pid))) });
  // handle_result(task).await
}

/// create a Window instance with title
///
/// NOTE
///
///
#[napi]
pub async fn find_window_by_title(title: String) -> Result<Option<Window>> {
  let task = tokio::spawn(async move {
    // case 1
    // let hstring_title = HSTRING::from(title);
    // let hwnd = unsafe { FindWindowW(None, &hstring_title) };

    // case 2
    // let hwnd = unsafe { FindWindowW(None, PCWSTR(encode_wide(title).as_ptr())) };

    // case 3
    let hwnd = get_hwnd_by_title_hstring(title);

    // case 4
    // let hwnd = find_window_contains_title_inner(title).hwnd;
    if hwnd.0 == 0 {
      Ok(None)
    } else {
      Ok(Some(Window { hwnd }))
    }
  });
  handle_result(task).await
}

///
#[napi]
pub async fn find_window_by_class_name(classname: String) -> Result<Option<Window>> {
  let task = tokio::spawn(async move {
    // case 2
    // let hwnd = unsafe { FindWindowW(PCWSTR(encode_wide(classname).as_ptr()), None) };

    let hwnd = get_hwnd_by_class_name(classname);
    // case 4
    // let hwnd = find_window_contains_class_name_inner(classname).hwnd;
    if hwnd.0 == 0 {
      Ok(None)
    } else {
      Ok(Some(Window { hwnd }))
    }
  });
  handle_result(task).await
}

/// create a Window instance with title substring
///
/// NOTE
///
/// list window ->  find
#[napi]
pub async fn find_window_contains_title(title: String) -> Result<Option<Window>> {
  let task: tokio::task::JoinHandle<std::result::Result<Option<Window>, String>> =
    tokio::spawn(async move {
      // let hwnd = unsafe { GetForegroundWindow() };
      let hwnd = find_window_contains_title_inner(title).hwnd;
      if hwnd.0 == 0 {
        Ok(None)
      } else {
        Ok(Some(Window { hwnd }))
      }
    });
  handle_result(task).await

  // let task = tokio::spawn(async move { Ok(Some(get_window_contains_title_inner(title))) });
  // handle_result(task).await
}

/// create a Window instance with title substring
///
/// NOTE
///
/// list window ->  find
#[napi]
pub async fn find_window_contains_class_name(name: String) -> Result<Option<Window>> {
  let task = tokio::spawn(async move {
    // let hwnd = unsafe { GetForegroundWindow() };
    let hwnd = find_window_contains_class_name_inner(name).hwnd;

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

#[napi]
impl Window {
  // bind as static method in js
  // ------------static-----------
  #[napi]
  pub async fn get_foreground_window() -> Result<Option<Window>> {
    get_foreground_window().await
  }
  #[napi]
  pub async fn find_window_by_pid(pid: u32) -> Result<Option<Window>> {
    // todo: fix err when not this pid ?
    // rust: Task join failed: JoinError::Panic(Id(156)
    // node: triggerUncaughtException(err, true /* fromPromise */)
    find_window_by_pid(pid).await
  }

  #[napi]
  pub async fn find_window_by_title(title: String) -> Result<Option<Window>> {
    find_window_by_title(title).await
  }

  #[napi]
  pub async fn find_window_by_class_name(classname: String) -> Result<Option<Window>> {
    find_window_by_class_name(classname).await
    // from_sub_class_name(title).await
  }

  #[napi]
  pub async fn find_window_by_sub_title(title: String) -> Result<Option<Window>> {
    find_window_contains_title(title).await
  }

  #[napi]
  pub async fn find_window_by_sub_class_name(title: String) -> Result<Option<Window>> {
    find_window_contains_class_name(title).await
  }

  // set method binding as public in js
  #[napi]
  pub async fn from_active(&self) -> Result<Option<Window>> {
    get_foreground_window().await
  }
  #[napi]
  pub async fn from_title(&self, title: String) -> Result<Option<Window>> {
    find_window_by_title(title).await
  }

  #[napi]
  pub async fn from_class_name(&self, name: String) -> Result<Option<Window>> {
    find_window_by_class_name(name).await
  }

  #[napi]
  pub async fn from_pid(&self, pid: u32) -> Result<Option<Window>> {
    // todo: fix err when not this pid ?
    // rust: Task join failed: JoinError::Panic(Id(156)
    // node: triggerUncaughtException(err, true /* fromPromise */)
    find_window_by_pid(pid).await
  }

  #[napi]
  pub async fn from_sub_title(&self, title: String) -> Result<Option<Window>> {
    find_window_contains_title(title).await
  }

  #[napi]
  pub async fn from_sub_class_name(&self, name: String) -> Result<Option<Window>> {
    find_window_contains_class_name(name).await
  }

  // #[napi]
  // pub fn from_raw_hwnd(&self, hwnd: u32) -> Self {
  //   Self {
  //     hwnd: HWND(hwnd as isize),
  //   }
  // }
  pub fn from_raw_hwnd(&self, hwnd: u32) -> Window {
    Window {
      hwnd: HWND(hwnd as isize),
    }
  }
  #[napi]
  pub fn as_raw_hwnd(&self) -> isize {
    self.hwnd.0
  }

  // get id,title,classname,
  #[napi]
  pub async fn get_id(&self) -> Result<u32> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(get_hwnd_pid(hwnd)) });
    handle_result(task).await
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
    let task = tokio::spawn(async move { Ok(get_hwnd_title_next(hwnd)) });
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
    let task = tokio::spawn(async move { Ok(get_hwnd_class_name(hwnd)) });
    handle_result(task).await
  }

  // get rect,mouse-pos,
  // code(core): impl struct Window with a method get_window_rect
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use fn windows::Win32::Foundation::RECT::default
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetWindowRect
  // code(core): use struct sophia::geometry::Rect
  // about Result<Rect>,Result<Option<Rect>>
  // docs(core): Result<Option<Rect>> in rust, rect || null in js
  // docs(core): Result<Rect> in rust, rect in js
  #[napi]
  pub async fn get_window_rect(&self) -> Result<Rect> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(get_hwnd_rect(hwnd)) });
    handle_result(task).await
  }

  /// like keysender's workwindow.getWindowView
  ///
  #[napi]
  pub async fn get_window_view(&self) -> Result<WindowView> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(get_hwnd_view(hwnd)) });
    handle_result(task).await
  }

  //
  #[napi]
  pub async fn get_window_meta_info(&self) -> Result<WindowMetaInfo> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move { Ok(get_hwnd_meta_info(hwnd)) });

    handle_result(task).await
  }

  #[napi]
  pub async fn get_mouse_pos(&self) -> Result<Point> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(get_mouse_position_in_window(hwnd)) });
    handle_result(task).await
  }

  // code(core): impl struct Window with a method set_position
  // code(core): use napi macro to label it
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE
  // code(core): use inner fn self.set_window_pos

  #[napi]
  pub async fn set_position(&self, x: i32, y: i32) -> Result<()> {
    // self.set_window_pos(x, y, 0, 0, SWP_NOSIZE).await
    set_hwnd_pos_async(self.hwnd, x, y, 0, 0, SWP_NOSIZE).await
  }

  // code(core): impl struct Window with a method set_size
  // code(core): use napi macro to label it
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
  // code(core): use inner fn self.set_window_pos

  #[napi]
  pub async fn set_size(&self, width: i32, height: i32) -> Result<()> {
    // self.set_window_pos(0, 0, width, height, SWP_NOMOVE).await
    set_hwnd_pos_async(self.hwnd, 0, 0, width, height, SWP_NOMOVE).await
  }

  // code(core): impl struct Window with a method is_foreground
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task

  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow

  /// like keysender's workwindow.isForeground
  ///
  #[napi]
  pub async fn is_foreground(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(is_foreground_hwnd(hwnd)) });
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
    // self.set_active_window().await
    set_active_hwnd_async(self.hwnd).await
  }

  // code(core): impl struct Window with a method set_foreground
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  /// like keysender's workwindow.setForeground
  ///
  #[napi]
  pub async fn set_foreground(&self) -> Result<bool> {
    // self.set_active_window().await
    set_active_hwnd_async(self.hwnd).await
  }
  /// like keysender's workwindow.isOpen
  ///
  #[napi]
  pub async fn is_open(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(is_open_hwnd(hwnd)) });

    handle_result(task).await
  }
  #[napi]
  pub async fn is_minimized(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(is_minimize_hwnd(hwnd)) });

    handle_result(task).await
  }
  // code(core): impl struct Window with a method show
  // code(core): use napi macro to label it
  // code(core): use fn tokio::spawn to make async task
  // code(core): use fn sophia::utils::handle_result to handle task
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL
  // code(core): use fn windows::Win32::UI::WindowsAndMessaging::ShowWindowAsync
  #[napi]
  pub async fn show(&self) -> Result<()> {
    // show_hwnd_async(self.hwnd, SW_SHOWNA).await

    // let hwnd: HWND = self.hwnd;
    // let task: tokio::task::JoinHandle<std::result::Result<bool, String>> =
    //   tokio::spawn(async move { Ok(set_show_hwnd(hwnd)) });
    // handle_result(task).await
    show_hwnd_async(self.hwnd, SW_SHOWNOACTIVATE).await
  }
  #[napi]
  pub async fn hide(&self) -> Result<()> {
    // self.show_window(SW_MINIMIZE).await
    show_hwnd_async(self.hwnd, SW_HIDE).await
  }

  // code(core): impl struct Window with a method minimize
  // code(core): use napi macro to label it
  // code(core): use inner fn self.show_window
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_MINIMIZE
  #[napi]
  pub async fn minimize(&self) -> Result<()> {
    // self.show_window(SW_MINIMIZE).await
    show_hwnd_async(self.hwnd, SW_MINIMIZE).await
  }

  // code(core): impl struct Window with a method maximize
  // code(core): use napi macro to label it
  // code(core): use inner fn self.show_window
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_MINIMIZE
  #[napi]
  pub async fn maximize(&self) -> Result<()> {
    // self.show_window(SW_MAXIMIZE).await
    show_hwnd_async(self.hwnd, SW_MAXIMIZE).await
  }

  #[napi]
  pub async fn close(&self) -> Result<()> {
    let hwnd: HWND = self.hwnd;
    let task = tokio::spawn(async move {
      close_hwnd(hwnd);
      Ok(())
    });

    handle_result(task).await
  }
  // #[napi]
  // pub async fn open(&self) -> Result<()> {
  //   // self.show_window(SW_MINIMIZE).await
  //   // about SW_SHOW SW_RESTORE,SW_SHOWDEFAULT
  //   show_hwnd_async(self.hwnd, SW_SHOWDEFAULT).await
  // }

  #[napi]
  pub async fn kill(&self) -> Result<()> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      kill_hwnd(hwnd);
      Ok(())
    });

    handle_result(task).await
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
  //
  #[napi]
  pub async fn mouse_move(&self, coords: Point, is_absolute: bool) -> Result<()> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      if is_absolute {
        let last_coords = get_mouse_position_in_window(hwnd);
        mouse_move_in_window_inner(hwnd, coords_move(last_coords, coords));
        // need move mouse in screen ?
        // ...
      } else {
        mouse_move_in_window_inner(hwnd, coords);
      }

      Ok(()) //return void in js
    });

    handle_result(task).await
  }
  // mouse_toggler_in_window_inner
  #[napi]
  pub async fn mouse_toggle(
    &self,
    coords: Point,
    button: String,
    is_button_down: bool,
  ) -> Result<()> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      mouse_toggle_in_window_inner(hwnd, coords, button, is_button_down);

      Ok(()) //return void in js
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
    // self.set_active_window();
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
