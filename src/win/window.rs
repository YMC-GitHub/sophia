use crate::geometry::{LParamFlag, Point, Rect, WindowMetaInfo, WindowView};
// todo: move crate::screen::ImageData to crate::image::ImageData
use crate::screen::ImageData;
use crate::utils::handle_result;
use crate::win::utils::{
  close_hwnd, coords_move, get_hwnd_by_class_name, get_hwnd_by_title_hstring, get_hwnd_class_name,
  get_hwnd_meta_info, get_hwnd_pid, get_hwnd_rect, get_hwnd_title_next, get_hwnd_view,
  get_mouse_position_in_window, is_foreground_hwnd, is_minimize_hwnd, is_open_hwnd,
  keyboard_print_char_in_window_inner, keyboard_send_combine_key_in_window_inner, kill_hwnd,
  list_hwnd, lparam_u32_from_flag, lparam_u32_to_flag, mouse_move_in_window_inner,
  mouse_toggle_in_window_inner, mouse_wheel_scroll_in_window_inner, set_active_hwnd, set_hwnd_pos,
  show_hwnd, vk_from_u16,
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
// ...

// set class static method
// ...

// set class public method
// ...

// feat(core): define struct Window
// code(core): def struct Window
// code(core): use napi macro to label it
// code(core): with hwnd prop
// code(core): use struct windows::Win32::Foundation::HWND

#[napi]
pub struct Window {
  hwnd: HWND,
  last_coords: Point,
}
// code(core): impl Window
// code(core): use napi macro to label it

impl Window {
  pub fn new(hwnd: HWND) -> Self {
    Self {
      hwnd,
      last_coords: Point::new(0, 0),
    }
  }
}

// [as raw HWND of the window and hwnd id](https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#L223)
// [from raw HWND of the window and hwnd id](https://github.com/NiiightmareXD/windows-capture/blob/main/src/window.rs#217)

///
/// NOTE
///
/// hwnd id vs pid, is the same ?
pub fn get_window_from_hwnd_id(hwnd: isize) -> Window {
  // Window { hwnd: HWND(hwnd) }
  Window::new(HWND(hwnd))
}
pub fn get_window_from_hwnd(hwnd: HWND) -> Window {
  // Window { hwnd }
  Window::new(hwnd)
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
  // Window { hwnd: window.hwnd }
  Window::new(window.hwnd)
}

pub fn find_window_contains_title_inner(title: String) -> Window {
  let windows = list_window_inner();
  let window: &Window = windows
    .iter()
    .find(|i| get_hwnd_title_next(i.hwnd).contains(&title))
    .unwrap();
  // Window { hwnd: window.hwnd }
  Window::new(window.hwnd)
}

pub fn find_window_contains_class_name_inner(name: String) -> Window {
  let windows = list_window_inner();
  let window: &Window = windows
    .iter()
    .find(|i| get_hwnd_class_name(i.hwnd).contains(&name))
    .unwrap();
  // Window { hwnd: window.hwnd }
  Window::new(window.hwnd)
}

// feat(core): define fn list_window to list all window that is valid

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

// feat(core): define fn get_foreground_window to get foreground window

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
      // Ok(Some(Window { hwnd }))
      Ok(Some(Window::new(hwnd)))
    }
  });
  handle_result(task).await
}

// feat(core): define fn find_window_by_pid to find window by process id

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
        // Ok(Some(Window { hwnd }))
        Ok(Some(Window::new(hwnd)))
      }
    });
  handle_result(task).await

  // let task = tokio::spawn(async move { Ok(Some(find_window_by_pid_inner(pid))) });
  // handle_result(task).await
}

// feat(core): define fn find_window_by_title to find window by window title

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
      // Ok(Some(Window { hwnd }))
      Ok(Some(Window::new(hwnd)))
    }
  });
  handle_result(task).await
}

// feat(core): define fn find_window_by_class_name to find window by window class name

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
      // Ok(Some(Window { hwnd }))
      Ok(Some(Window::new(hwnd)))
    }
  });
  handle_result(task).await
}

// feat(core): define fn find_window_contains_title to find window that contains title substring

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
        // Ok(Some(Window { hwnd }))
        Ok(Some(Window::new(hwnd)))
      }
    });
  handle_result(task).await

  // let task = tokio::spawn(async move { Ok(Some(get_window_contains_title_inner(title))) });
  // handle_result(task).await
}

// feat(core): define fn find_window_contains_class_name to find window that contains class name substring

/// create a Window instance with class name substring
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
      // Ok(Some(Window { hwnd }))
      Ok(Some(Window::new(hwnd)))
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

  // feat(core): add fn get_all_windows to Window as static method

  #[napi]
  pub async fn get_all_windows() -> Result<Vec<Window>> {
    list_window().await
  }

  // feat(core): add fn get_foreground_window to Window as static method

  #[napi]
  pub async fn get_foreground_window() -> Result<Option<Window>> {
    get_foreground_window().await
  }

  // feat(core): add fn find_window_by_pid to Window as static method

  #[napi]
  pub async fn find_window_by_pid(pid: u32) -> Result<Option<Window>> {
    // todo: fix err when not this pid ?
    // rust: Task join failed: JoinError::Panic(Id(156)
    // node: triggerUncaughtException(err, true /* fromPromise */)
    find_window_by_pid(pid).await
  }

  // feat(core): add fn find_window_by_title to Window as static method

  #[napi]
  pub async fn find_window_by_title(title: String) -> Result<Option<Window>> {
    find_window_by_title(title).await
  }

  // feat(core): add fn find_window_by_class_name to Window as static method

  #[napi]
  pub async fn find_window_by_class_name(classname: String) -> Result<Option<Window>> {
    find_window_by_class_name(classname).await
    // from_sub_class_name(title).await
  }

  // feat(core): add fn find_window_by_sub_title to Window as static method

  #[napi]
  pub async fn find_window_by_sub_title(title: String) -> Result<Option<Window>> {
    find_window_contains_title(title).await
  }

  // feat(core): add fn find_window_by_sub_class_name to Window as static method

  #[napi]
  pub async fn find_window_by_sub_class_name(title: String) -> Result<Option<Window>> {
    find_window_contains_class_name(title).await
  }

  // set method binding as public in js

  // feat(core): add fn from_active to Window as instance method

  #[napi]
  pub async fn from_active(&self) -> Result<Option<Window>> {
    get_foreground_window().await
  }

  // feat(core): add fn from_title to Window as instance method

  #[napi]
  pub async fn from_title(&self, title: String) -> Result<Option<Window>> {
    find_window_by_title(title).await
  }

  // feat(core): add fn from_class_name to Window as instance method

  #[napi]
  pub async fn from_class_name(&self, name: String) -> Result<Option<Window>> {
    find_window_by_class_name(name).await
  }

  // feat(core): add fn from_pid to Window as instance method

  #[napi]
  pub async fn from_pid(&self, pid: u32) -> Result<Option<Window>> {
    // todo: fix err when not this pid ?
    // rust: Task join failed: JoinError::Panic(Id(156)
    // node: triggerUncaughtException(err, true /* fromPromise */)
    find_window_by_pid(pid).await
  }

  // feat(core): add fn from_sub_title to Window as instance method

  #[napi]
  pub async fn from_sub_title(&self, title: String) -> Result<Option<Window>> {
    find_window_contains_title(title).await
  }

  // feat(core): add fn from_sub_class_name to Window as instance method

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
    // Window {
    //   hwnd: HWND(hwnd as isize),
    // }
    Window::new(HWND(hwnd as isize))
  }

  // feat(core): add fn as_raw_hwnd to Window as instance method to get hwnd id

  #[napi]
  pub fn as_raw_hwnd(&self) -> isize {
    self.hwnd.0
  }

  // feat(core): add fn get_id to Window as instance method to get window id

  // get id,title,classname,
  #[napi]
  pub async fn get_id(&self) -> Result<u32> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(get_hwnd_pid(hwnd)) });
    handle_result(task).await
  }

  // feat(core): add fn get_title to Window as instance method to get window title

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

  // feat(core): add fn get_class_name to Window as instance method to get window class name

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

  // feat(core): add fn get_window_rect to Window as instance method to get window rect

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

  // feat(core): add fn get_window_view to Window as instance method to get window view

  /// like keysender's workwindow.getWindowView
  ///
  #[napi]
  pub async fn get_window_view(&self) -> Result<WindowView> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(get_hwnd_view(hwnd)) });
    handle_result(task).await
  }

  // feat(core): add fn get_window_meta_info to Window as instance method to get window meta info

  //
  #[napi]
  pub async fn get_window_meta_info(&self) -> Result<WindowMetaInfo> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move { Ok(get_hwnd_meta_info(hwnd)) });

    handle_result(task).await
  }

  // feat(core): add fn get_mouse_pos to Window as instance method to get mouse position in window

  #[napi]
  pub async fn get_mouse_pos(&self) -> Result<Point> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(get_mouse_position_in_window(hwnd)) });
    handle_result(task).await
  }

  // feat(core): add fn set_position to Window as instance method to set window postion

  // code(core): impl struct Window with a method set_position
  // code(core): use napi macro to label it
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE
  // code(core): use inner fn self.set_window_pos

  #[napi]
  pub async fn set_position(&self, x: i32, y: i32) -> Result<()> {
    // self.set_window_pos(x, y, 0, 0, SWP_NOSIZE).await
    set_hwnd_pos_async(self.hwnd, x, y, 0, 0, SWP_NOSIZE).await
  }

  // feat(core): add fn set_size to Window as instance method to set window size

  // code(core): impl struct Window with a method set_size
  // code(core): use napi macro to label it
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
  // code(core): use inner fn self.set_window_pos

  #[napi]
  pub async fn set_size(&self, width: i32, height: i32) -> Result<()> {
    // self.set_window_pos(0, 0, width, height, SWP_NOMOVE).await
    set_hwnd_pos_async(self.hwnd, 0, 0, width, height, SWP_NOMOVE).await
  }

  // feat(core): add fn is_foreground to Window as instance method to check if window is foreground

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

  // feat(core): add fn foreground to Window as instance method to set window to foreground (here is active)

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

  // feat(core): add fn set_foreground to Window as instance method to set window to foreground (here is active)

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

  // feat(core): add fn is_open to Window as instance method to check if window is opened (isWindow in cpp)

  /// like keysender's workwindow.isOpen
  ///
  #[napi]
  pub async fn is_open(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(is_open_hwnd(hwnd)) });

    handle_result(task).await
  }

  // feat(core): add fn is_minimized to Window as instance method to check if window is minimized (isIconic in cpp)

  #[napi]
  pub async fn is_minimized(&self) -> Result<bool> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move { Ok(is_minimize_hwnd(hwnd)) });

    handle_result(task).await
  }

  // feat(core): add fn show to Window as instance method to show window

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

  // feat(core): add fn hide to Window as instance method to hide window

  #[napi]
  pub async fn hide(&self) -> Result<()> {
    // self.show_window(SW_MINIMIZE).await
    show_hwnd_async(self.hwnd, SW_HIDE).await
  }

  // feat(core): add fn minimize to Window as instance method to minimize window

  // code(core): impl struct Window with a method minimize
  // code(core): use napi macro to label it
  // code(core): use inner fn self.show_window
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_MINIMIZE
  #[napi]
  pub async fn minimize(&self) -> Result<()> {
    // self.show_window(SW_MINIMIZE).await
    show_hwnd_async(self.hwnd, SW_MINIMIZE).await
  }

  // feat(core): add fn maximize to Window as instance method to maximize window

  // code(core): impl struct Window with a method maximize
  // code(core): use napi macro to label it
  // code(core): use inner fn self.show_window
  // code(core): use const windows::Win32::UI::WindowsAndMessaging::SW_MINIMIZE
  #[napi]
  pub async fn maximize(&self) -> Result<()> {
    // self.show_window(SW_MAXIMIZE).await
    show_hwnd_async(self.hwnd, SW_MAXIMIZE).await
  }

  // feat(core): add fn close to Window as instance method to close window

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

  // feat(core): add fn kill to Window as instance method to kill window

  #[napi]
  pub async fn kill(&self) -> Result<()> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      kill_hwnd(hwnd);
      Ok(())
    });

    handle_result(task).await
  }

  // feat(core): add fn is_visible to Window as instance method to check if window visible (IsWindowVisible in cpp)

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

  // feat(core): add fn mouse_move to Window as instance method to move mouse in window

  #[napi]
  ///
  /// not move coords to last coord in if is_absolute
  pub async fn mouse_move(&self, coords: Point, is_absolute: bool) -> Result<()> {
    let hwnd = self.hwnd;
    let mut last_coords = self.last_coords;
    let task = tokio::spawn(async move {
      if is_absolute {
        mouse_move_in_window_inner(hwnd, coords);
      } else {
        // let last_coords = get_mouse_position_in_window(hwnd);
        last_coords = coords_move(last_coords, coords);
        mouse_move_in_window_inner(hwnd, last_coords);
        // need move mouse in screen ?
        // ...
      }
      Ok(()) //return void in js
    });

    handle_result(task).await
  }

  // feat(core): add fn mouse_toggle to Window as instance method to toggle mouse button in window

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

  // feat(core): add fn mouse_wheel_scroll to Window as instance method to scroll mouse wheel in window

  #[napi]
  pub async fn mouse_wheel_scroll(&self, coords: Point, is_up: bool) -> Result<()> {
    let hwnd = self.hwnd;

    let task = tokio::spawn(async move {
      mouse_wheel_scroll_in_window_inner(hwnd, coords, is_up);

      Ok(()) //return void in js
    });

    handle_result(task).await
  }

  // feat(core): add fn typing to Window as instance method to typing text with keyboard in window

  #[napi]
  pub async fn typing(&self, text: String) -> Result<()> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      keyboard_print_char_in_window_inner(hwnd, text);
      Ok(()) //return void in js
    });

    handle_result(task).await
  }

  // feat(core): add fn keyboard_toggle_key to Window as instance method to toggle keys with keyboard in window

  // toggleKey
  #[napi]
  pub async fn keyboard_toggle_key(
    &self,
    keys: Vec<String>,
    is_key_down: bool,
    is_prev_key_down: bool,
  ) -> Result<()> {
    let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      // keyboard_toggle_key_in_window_inner(hwnd, text, is_key_down);
      for key in keys {
        // keyboard_toggle_key_in_window_inner(hwnd, key, is_key_down, is_prev_key_down);
        keyboard_send_combine_key_in_window_inner(hwnd, key, is_key_down, is_prev_key_down);
      }
      Ok(()) //return void in js
    });

    handle_result(task).await
  }

  // feat(core): add fn decode_lparam_value to Window as instance method to decode lparam

  #[napi]
  pub async fn decode_lparam_value(value: u32) -> Result<LParamFlag> {
    // let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      let res = lparam_u32_to_flag(value as u32);
      Ok(res) //return void in js
    });

    handle_result(task).await
  }

  // feat(core): add fn decode_lparam_value to Window as instance method to encode lparam

  #[napi]
  pub async fn cook_lparam_value(vk: u32, flag: LParamFlag) -> Result<i32> {
    // let hwnd = self.hwnd;
    let task = tokio::spawn(async move {
      //  flag
      // let res = lparam_isize_from_flag(vk_from_u16(key as u16), lparam_flag_preset_keyup());
      let res = lparam_u32_from_flag(vk_from_u16(vk as u16), flag) as i32;

      Ok(res) //return void in js
    });

    handle_result(task).await
  }

  // feat(core): add fn capture to Window as instance method to capture window

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
      // let w: u32 = buf.width;
      // let h: u32 = buf.height;
      // let pixel_width = (4 * w * h) as u8;
      let pixel_width = (4) as u8;

      Ok(ImageData {
        data: buf.pixels,
        width: buf.width,
        height: buf.height,
        pixel_width: pixel_width,
      })
      //
    });

    handle_result(task).await
  }

  // feat(core): add fn capture_area to Window as instance method to capture window rect

  #[napi]
  pub async fn capture_area(&self, x: i32, y: i32, width: i32, height: i32) -> Result<ImageData> {
    let hwnd = self.hwnd;

    // let rect = get_window_rect_sync(hwnd);

    let task = tokio::spawn(async move {
      // let hwnd = GetDesktopWindow();

      // BitBlt dramatically faster, often fails
      // (e.g. firefox, steam, 3d accelerated windows)
      // 0
      let using = win_screenshot::capture::Using::BitBlt;

      // 1
      // PrintWindow much slower, much more reliable
      // let using = win_screenshot::capture::Using::PrintWindow;

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
      // let w: u32 = buf.width;
      // let h: u32 = buf.height;
      // let pixel_width = (4 * w * h) as u8;
      let pixel_width = (4) as u8;

      Ok(ImageData {
        data: buf.pixels,
        width: buf.width,
        height: buf.height,
        pixel_width: pixel_width,
      })
    });

    handle_result(task).await
  }
}
