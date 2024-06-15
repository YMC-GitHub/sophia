// use std::ffi::OsString;
// use std::os::windows::ffi::OsStrExt;
// use windows::core::PCWSTR;
// use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
// use windows::Win32::UI::WindowsAndMessaging::{
//   EnumWindows, FindWindowW, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
// };

// #[derive(Debug)]
// pub struct HwndName {
//   pub hwnd: isize,
//   pub window_name: String,
// }

// #[derive(Debug)]
// pub enum FWError {
//   NotFoundOrFault,
// }

// pub fn find_window(window_name: &str) -> Result<isize, FWError> {
//   unsafe {
//     let w = FindWindowW(
//       PCWSTR::null(),
//       PCWSTR(
//         OsString::from(window_name)
//           .encode_wide()
//           .chain(Some(0))
//           .collect::<Vec<_>>()
//           .as_ptr(),
//       ),
//     );
//     match w {
//       HWND(0) => Err(FWError::NotFoundOrFault),
//       HWND(p) => Ok(p),
//     }
//   }
// }

// unsafe extern "system" fn wl_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
//   let vec = lparam.0 as *mut Vec<HwndName>;

//   if IsWindowVisible(hwnd) == false {
//     return BOOL::from(true);
//   }

//   let gwtl = GetWindowTextLengthW(hwnd);
//   if gwtl == 0 {
//     return BOOL::from(true);
//   }

//   let mut name_buf: Vec<u16> = vec![0; (gwtl + 1) as usize];

//   let gwt = GetWindowTextW(hwnd, &mut name_buf);
//   if gwt == 0 {
//     return BOOL::from(true);
//   }

//   let name_buf = match name_buf.split_last() {
//     Some((_, last)) => last,
//     None => return BOOL::from(true),
//   };

//   let name = String::from_utf16_lossy(name_buf);

//   (*vec).push(HwndName {
//     hwnd: hwnd.0,
//     window_name: name,
//   });

//   BOOL::from(true)
// }

// #[derive(Debug)]
// pub enum WLError {
//   EnumWindowsError,
// }

// pub fn window_list() -> Result<Vec<HwndName>, WLError> {
//   let mut hwnd_name = Vec::new();
//   unsafe {
//     EnumWindows(
//       Some(wl_callback),
//       LPARAM(&mut hwnd_name as *mut Vec<HwndName> as isize),
//     )
//     .map_err(|_| WLError::EnumWindowsError)?
//   }
//   Ok(hwnd_name)
// }

// [get active window in rust](https://hellocode.co/blog/post/tracking-active-process-windows-rust/)
use crate::geometry::{Rect, WindowMetaInfo};
use crate::utils::{decode_wide, encode_wide};

use windows::core::{HSTRING, PCWSTR};
// use windows::Win32::Foundation::HANDLE;
use windows::Win32::{
  Foundation::{FALSE, HANDLE, HWND, LPARAM, WPARAM},
  UI::WindowsAndMessaging::{
    FindWindowW, GetClassNameW, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsIconic, IsWindow, SendMessageA, SetForegroundWindow, SetWindowPos,
    ShowWindow, ShowWindowAsync, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, SW_SHOWNORMAL, WM_CLOSE,
    WM_NULL,
  },
};

// code(core): get the handle to our window with GetForegroundWindow()

// code(core): exchange that handle for a process ID (PID) with GetWindowThreadProcessId
// docs(core): a window handle isn't really useful here, but that PID will come in handy later
// code(core): get the window title with GetWindowTextW
pub fn get_active_window() -> (u32, String) {
  unsafe {
    // 1 get window handle
    let hwnd = GetForegroundWindow();

    // 2 get window thread process id
    // let mut pid: u32 = 0;
    // GetWindowThreadProcessId(hwnd, Some(&mut pid));

    let pid = get_window_pid(hwnd);
    // 3 get window title
    // let mut bytes: [u16; 500] = [0; 500];
    // let len = GetWindowTextW(hwnd, &mut bytes);
    // let title = String::from_utf16_lossy(&bytes[..len as usize]);
    let title = get_window_title(hwnd);

    (pid, title)
  }
}

// todo(core): about HSTRING and PCWSTR
pub fn get_window_by_title(title: String) -> HWND {
  unsafe {
    let hwnd = FindWindowW(None, PCWSTR(encode_wide(title).as_ptr()));
    hwnd
    // or:
    // let hstring_title = HSTRING::from(title);
    // let hwnd = unsafe { FindWindowW(None, &hstring_title) };
    // hwnd
  }
}

pub fn get_window_by_title_hstring(title: String) -> HWND {
  let hstring_title = HSTRING::from(title);
  let hwnd = unsafe { FindWindowW(None, &hstring_title) };
  hwnd
}

pub fn get_window_by_class_name(classname: String) -> HWND {
  let hwnd = unsafe { FindWindowW(PCWSTR(encode_wide(classname).as_ptr()), None) };
  hwnd
}

// process handle -> window thread process id -> window handle

// pub fn get_window_by_pid(pid: u32) -> HWND {
//   unsafe {
//     // pid -> title -> FindWindowW
//     let hwnd = FindWindowW(None, PCWSTR(encode_wide(title).as_ptr()));
//     hwnd
//     // or:
//     // let hstring_title = HSTRING::from(title);
//     // let hwnd = unsafe { FindWindowW(None, &hstring_title) };
//     // hwnd
//   }
// }
// pub fn get_window_by_pid(pid: u32) -> HWND {
//   unsafe {
//     // pid -> title -> FindWindowW
//     let hwnd = FindWindowW(None, PCWSTR(encode_wide(title).as_ptr()));
//     hwnd
//     // or:
//     // let hstring_title = HSTRING::from(title);
//     // let hwnd = unsafe { FindWindowW(None, &hstring_title) };
//     // hwnd
//   }
// }

// use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
// pub fn get_process_handle_by_pid(pid: u32) -> Result<HANDLE> {
//   unsafe {
//     match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid) {
//       Ok(handle) => Ok(handle),
//       Err(err) => {
//         return Err(format!("Failed to open process: {:?}", err));
//       }
//     }
//     // phd
//   };
// }
// code(core): def pub fn get_window_title with hwnd

pub fn get_window_title(hwnd: HWND) -> String {
  unsafe {
    // 3 get window title
    let mut bytes: [u16; 500] = [0; 500];
    let len = GetWindowTextW(hwnd, &mut bytes);
    let title = String::from_utf16_lossy(&bytes[..len as usize]);

    title
  }
}

pub fn get_window_title_next(hwnd: HWND) -> String {
  unsafe {
    let len = GetWindowTextLengthW(hwnd);
    let mut buffer = vec![0u16; len as usize + 1];
    GetWindowTextW(hwnd, &mut buffer);
    let title = decode_wide(&buffer);
    title
  }
}
// code(core): def pub fn get_window_pid with hwnd

pub fn get_window_pid(hwnd: HWND) -> u32 {
  unsafe {
    // 3 get window title
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));
    pid
  }
}

pub fn get_window_class_name(hwnd: HWND) -> String {
  unsafe {
    let mut buffer = vec![0u16; 256 as usize];
    GetClassNameW(hwnd, &mut buffer);
    decode_wide(&buffer)
  }
}

pub fn get_window_rect(hwnd: HWND) -> Rect {
  let mut rect = windows::Win32::Foundation::RECT::default();
  unsafe {
    let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
  };
  Rect::new(rect.left, rect.top, rect.right, rect.bottom)
}

pub fn get_window_meta_info(hwnd: HWND) -> WindowMetaInfo {
  let pid = get_window_pid(hwnd);
  let rect = get_window_rect(hwnd);
  let classname = get_window_class_name(hwnd);
  let title = get_window_title_next(hwnd);

  WindowMetaInfo {
    title,
    id: pid,
    rect,
    class_name: classname,
  }
}

pub fn show_window(hwnd: HWND, state: SHOW_WINDOW_CMD) -> () {
  unsafe {
    ShowWindow(hwnd, state);
  }
  ()
}

pub fn is_foreground_window(hwnd: HWND) -> bool {
  let res = unsafe { GetForegroundWindow() };
  res == hwnd
}

pub fn is_open_window(hwnd: HWND) -> bool {
  // 1
  // if !unsafe { IsWindow(hwnd).as_bool() } {
  //   return false;
  // }
  // return true;

  // 2
  let res: bool = unsafe { IsWindow(hwnd).as_bool() };
  res
}

pub fn is_minimize_window(hwnd: HWND) -> bool {
  let res: bool = unsafe { IsIconic(hwnd).as_bool() };
  res
}

// ShowWindowAsync,SW_SHOWNORMAL,SetForegroundWindow
pub fn set_active_window(hwnd: HWND) -> bool {
  set_show_window(hwnd);
  set_foreground_window(hwnd)
}

pub fn set_foreground_window(hwnd: HWND) -> bool {
  let res = unsafe { SetForegroundWindow(hwnd) };
  res.0 != 0
}
pub fn set_show_window(hwnd: HWND) -> bool {
  let res = unsafe { ShowWindowAsync(hwnd, SW_SHOWNORMAL) };
  res.0 != 0
}
pub fn set_window_pos(
  hwnd: HWND,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  flags: SET_WINDOW_POS_FLAGS,
) -> () {
  unsafe {
    let _ = SetWindowPos(hwnd, None, x, y, width, height, flags);
  }
  ()
}
// SendMessage,WM_CLOSE
// [about wparam and lparam in cpp](https://blog.csdn.net/weixin_45525272/article/details/104301731)
// [close a window using the SendMessage in cpp](https://cplusplus.com/forum/windows/171648/)
// SendMessage ( hwnd, WM_CLOSE, NULL, NULL ) ;
// https://github.com/microsoft/windows-rs/issues/1631
pub fn close_window(hwnd: HWND) -> () {
  unsafe {
    SendMessageA(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
  }
  ()
}

// IsWindow(hwnd).as_bool()

use crate::geometry::Point;
use windows::Win32::UI::Input::KeyboardAndMouse::{
  MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
  MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
  MOUSE_EVENT_FLAGS,
};
use windows::Win32::UI::WindowsAndMessaging::{
  GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
};

// code(core): def inner fn get_mouse_position_inner to get mouse position
// code(core): use struct windows::Win32::Foundation::POINT to set mouse initial value
// code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetCursorPos to get mouse position
// code(core): use fn sophia::geometry::Point::new to make point and as result

pub fn get_mouse_position_inner() -> Point {
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
pub fn mouse_event(
  dw_flags: MOUSE_EVENT_FLAGS,
  dx: i32,
  dy: i32,
  dw_data: i32,
  dw_extra_info: usize,
) {
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

pub fn mouse_move_inner(x: i32, y: i32) {
  mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x, y, 0, 0);
}

pub fn get_mouse_position_in_window(hwnd: HWND) -> Point {
  // get mouse position in screen
  let pos = get_mouse_position_inner();
  // get window rect
  let rect = get_window_rect(hwnd);
  // pos
  Point {
    x: pos.x - rect.left,
    y: pos.y - rect.top,
  }
}

// Virtual.mouse
// getPos
// move
// scroll wheel
// click
// press
// release

// Virtual.window

// Virtual.keyboard
