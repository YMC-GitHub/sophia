// [get active window in rust](https://hellocode.co/blog/post/tracking-active-process-windows-rust/)
use crate::geometry::{Rect, WindowMetaInfo, WindowView};
use crate::utils::{decode_wide, encode_wide};
use std::path::{Path, PathBuf};
use windows::core::{HSTRING, PCWSTR, PWSTR};
// https://docs.rs/windows-sys/latest/windows_sys/Win32/Storage/FileSystem/index.html
use std::ptr;
use windows::Win32::System::Threading::{
  OpenProcess, QueryFullProcessImageNameW, TerminateProcess, PROCESS_NAME_WIN32,
  PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE,
};
use windows::Win32::{
  Foundation::{CloseHandle, BOOL, HANDLE, HWND, LPARAM, MAX_PATH, RECT, TRUE, WPARAM},
  UI::WindowsAndMessaging::{
    EnumChildWindows, FindWindowW, GetClassNameW, GetClientRect, GetDesktopWindow,
    GetForegroundWindow, GetWindowLongPtrW, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, SendMessageA,
    SetForegroundWindow, SetWindowPos, ShowWindow, ShowWindowAsync, GWL_EXSTYLE, GWL_STYLE,
    SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, SW_HIDE, SW_SHOWNORMAL, WM_CLOSE, WS_CHILD,
    WS_EX_TOOLWINDOW,
  },
};
// SWP_NOMOVE, SWP_NOSIZE, SW_MAXIMIZE, SW_MINIMIZE,

use windows::Win32::System::Threading::GetCurrentProcessId;

// -----------------get hwnd

// code(core): def fn get_hwnd_from_id
/// id -> hwnd
///
/// data flow: isize  ->  HWND
///
pub fn get_hwnd_from_id(id: isize) -> HWND {
  HWND(id)
}

// todo(core): about HSTRING and PCWSTR
/// title -> hwnd
///
/// data flow: String -> buf ->  PCWSTR -> HWND
pub fn get_hwnd_by_title(title: String) -> HWND {
  unsafe {
    let hwnd = FindWindowW(None, PCWSTR(encode_wide(title).as_ptr()));
    hwnd
  }
}

/// title -> hwnd
///
/// data flow: String -> HSTRING ->  HWND
pub fn get_hwnd_by_title_hstring(title: String) -> HWND {
  let hstring_title = HSTRING::from(title);
  let hwnd = unsafe { FindWindowW(None, &hstring_title) };
  hwnd
}

/// class-name -> hwnd
///
/// data flow: String -> buf ->  PCWSTR -> HWND
pub fn get_hwnd_by_class_name(classname: String) -> HWND {
  let hwnd = unsafe { FindWindowW(PCWSTR(encode_wide(classname).as_ptr()), None) };
  hwnd
}
/// get active hwnd
///
/// NOTE:
///
/// WITH GetForegroundWindow
pub fn get_active_hwnd() -> HWND {
  unsafe { GetForegroundWindow() }
}

// code(core): def fn is_valid_hwnd
/// Checks if the hwnd is a valid window
///
/// Returns `true` if the window is valid, `false` otherwise.
///
/// NOTE:
///
/// WITH IsWindowVisible
///
/// WITH GetWindowThreadProcessId,GetCurrentProcessId
///
/// WITH GetClientRect

pub fn is_valid_hwnd(hwnd: HWND) -> bool {
  if !unsafe { IsWindowVisible(hwnd).as_bool() } {
    return false;
  }

  //
  let mut id: u32 = 0;
  unsafe { GetWindowThreadProcessId(hwnd, Some(&mut id)) };
  if id == unsafe { GetCurrentProcessId() } {
    return false;
  }

  let mut rect = RECT::default();
  let result = unsafe { GetClientRect(hwnd, &mut rect) };
  if result.is_ok() {
    let styles = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) };
    let ex_styles = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };

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

unsafe extern "system" fn enum_hwnds_callback(hwnd: HWND, vec: LPARAM) -> BOOL {
  let hwnds = &mut *(vec.0 as *mut Vec<HWND>);
  // get_hwnd_from_id -> is_valid_hwnd -> push hwnd
  if is_valid_hwnd(get_hwnd_from_id(hwnd.0)) {
    hwnds.push(hwnd);
  }
  TRUE
}

/// list hwnd
///
/// NOTE:
///
/// WITH EnumChildWindows,GetDesktopWindow,enum_hwnds_callback
///
pub fn list_hwnd() -> Vec<HWND> {
  // let hwnd = self.hwnd;
  let mut hwnds: Vec<HWND> = Vec::new();

  unsafe {
    EnumChildWindows(
      GetDesktopWindow(),
      Some(enum_hwnds_callback),
      LPARAM(ptr::addr_of_mut!(hwnds) as isize),
    )
  };
  hwnds
}

/// list hwnd id
///
/// NOTE:
///
/// WITH list_hwnd,get_hwnd_pid
///
/// let u32 as isize
///
/// IDEA:
///
/// list hwnd -> map with get hwnd pid -> u32 as isize
pub fn list_hwnd_id() -> Vec<isize> {
  // let hwnds = &mut *(vec.0 as *mut Vec<HWND>);
  list_hwnd()
    .iter()
    .map(|x| get_hwnd_pid(*x) as isize)
    .collect()
}

// list window from hwnd
// list window from hwnd id

// -----------------get hwnd info
/// hwnd -> pid
///
/// data flow: HWND -> u32
///
/// NOTE:
///
/// label window thread  process id as pid
pub fn get_hwnd_pid(hwnd: HWND) -> u32 {
  unsafe {
    // 3 get window title
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));
    pid
  }
}

/// hwnd -> title
///
/// data flow: HWND -> buf -> String
///
/// NOTE:
///
/// use 500 as buf length
pub fn get_hwnd_title(hwnd: HWND) -> String {
  unsafe {
    // 3 get window title
    let mut bytes: [u16; 500] = [0; 500];
    let len = GetWindowTextW(hwnd, &mut bytes);
    let title = String::from_utf16_lossy(&bytes[..len as usize]);

    title
  }
}

/// hwnd -> title
///
/// data flow: HWND -> buf -> String
///
/// NOTE:
///
/// get buf length based on title length
pub fn get_hwnd_title_next(hwnd: HWND) -> String {
  unsafe {
    let len = GetWindowTextLengthW(hwnd);
    let mut buffer = vec![0u16; len as usize + 1];
    GetWindowTextW(hwnd, &mut buffer);
    let title = decode_wide(&buffer);
    title
  }
}

/// hwnd -> class-name
///
/// data flow: HWND -> buf -> String
///
/// NOTE:
///
/// with GetClassNameW
///
/// use 256 as buf length
pub fn get_hwnd_class_name(hwnd: HWND) -> String {
  unsafe {
    let mut buffer = vec![0u16; 256 as usize];
    GetClassNameW(hwnd, &mut buffer);
    decode_wide(&buffer)
  }
}

/// hwnd -> rect
///
/// data flow: HWND -> struct
///
pub fn get_hwnd_rect(hwnd: HWND) -> Rect {
  let mut rect: windows::Win32::Foundation::RECT = windows::Win32::Foundation::RECT::default();
  unsafe {
    let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
  };
  Rect::new(rect.left, rect.top, rect.right, rect.bottom)
}

/// hwnd -> windowview
///
/// data flow: HWND -> struct
///
pub fn get_hwnd_view(hwnd: HWND) -> WindowView {
  let mut rect: windows::Win32::Foundation::RECT = windows::Win32::Foundation::RECT::default();
  unsafe {
    let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
  };
  let res = Rect::new(rect.left, rect.top, rect.right, rect.bottom);
  WindowView::new(res.left, res.top, res.width, res.height)
}

/// hwnd -> meta-info
///
/// data flow: HWND -> struct
///
/// pid,rect,classname,title
pub fn get_hwnd_meta_info(hwnd: HWND) -> WindowMetaInfo {
  let pid = get_hwnd_pid(hwnd);
  let rect = get_hwnd_rect(hwnd);
  let classname = get_hwnd_class_name(hwnd);
  let title = get_hwnd_title_next(hwnd);
  WindowMetaInfo {
    title,
    id: pid,
    rect,
    class_name: classname,
  }
}

// -----------------set hwnd info
pub fn set_hwnd_pos(
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

// -----------------hide/show/kill/active hwnd
pub fn show_hwnd(hwnd: HWND, state: SHOW_WINDOW_CMD) -> () {
  unsafe {
    ShowWindow(hwnd, state);
  }
  ()
}

/// check if hwnd is foreground
///
/// data flow: HWND -> bool
///
pub fn is_foreground_hwnd(hwnd: HWND) -> bool {
  let res = unsafe { GetForegroundWindow() };
  res == hwnd
}
/// check if hwnd is window
///
/// data flow: HWND -> bool
///
/// NOTE:
///
/// with IsWindow
pub fn is_open_hwnd(hwnd: HWND) -> bool {
  // 1
  // if !unsafe { IsWindow(hwnd).as_bool() } {
  //   return false;
  // }
  // return true;

  // 2
  let res: bool = unsafe { IsWindow(hwnd).as_bool() };
  res
}

/// check if window is minimize
///
/// data flow: HWND -> bool
///
/// NOTE:
///
/// with IsIconic
pub fn is_minimize_hwnd(hwnd: HWND) -> bool {
  let res: bool = unsafe { IsIconic(hwnd).as_bool() };
  res
}

/// set active window
///
/// data flow: HWND -> bool
///
/// NOTE:
///
/// with set_show_hwnd,set_foreground_hwnd
///
// ShowWindowAsync,SW_SHOWNORMAL,SetForegroundWindow
pub fn set_active_hwnd(hwnd: HWND) -> bool {
  set_show_hwnd(hwnd);
  set_foreground_hwnd(hwnd)
}

/// set foreground window
///
/// data flow: HWND -> bool
///
/// NOTE:
///
/// with SetForegroundWindow
///
pub fn set_foreground_hwnd(hwnd: HWND) -> bool {
  let res = unsafe { SetForegroundWindow(hwnd) };
  res.0 != 0
}

/// set show window
///
/// data flow: HWND -> bool
///
/// NOTE:
///
/// with ShowWindowAsync
///
pub fn set_show_hwnd(hwnd: HWND) -> bool {
  // SW_SHOWNA vs SW_SHOWNORMAL vs SW_SHOWNOACTIVATE
  let res = unsafe { ShowWindowAsync(hwnd, SW_SHOWNORMAL) };
  res.0 != 0
}

// SendMessage,WM_CLOSE
// [about wparam and lparam in cpp](https://blog.csdn.net/weixin_45525272/article/details/104301731)
// [close a window using the SendMessage in cpp](https://cplusplus.com/forum/windows/171648/)
// SendMessage ( hwnd, WM_CLOSE, NULL, NULL ) ;
// https://github.com/microsoft/windows-rs/issues/1631

// [about ShowWindow and PostMessage to hide window in cpp](https://bbs.csdn.net/topics/50006475)
// [PostMessageW in rust](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.PostMessageW.html)
// [PostMessageA in rust](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.PostMessageA.html)
// [sendmessagea in cpp](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-sendmessagea)
// [showwindow in cpp](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-showwindow)
pub fn close_hwnd(hwnd: HWND) -> () {
  unsafe {
    SendMessageA(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
  }
  ()
}

pub fn hide_hwnd(hwnd: HWND) -> () {
  unsafe {
    // hide hwnd with SendMessageA
    // SendMessageA(hwnd, 0, WPARAM(0), LPARAM(0));
    // hide hwnd with ShowWindowAsync
    ShowWindowAsync(hwnd, SW_HIDE);
  }
  ()
}

/// kill window
///
/// NOTE:
///
/// with get_hwnd_pid,OpenProcess,TerminateProcess,CloseHandle
///
pub fn kill_hwnd(hwnd: HWND) -> bool {
  let pid = get_hwnd_pid(hwnd);
  unsafe {
    let h_thread = OpenProcess(PROCESS_SYNCHRONIZE | PROCESS_TERMINATE, TRUE, pid);
    match h_thread {
      Ok(handle) => {
        let _res = TerminateProcess(handle, 0);
        let _res = CloseHandle(handle);
        return true;
      }
      Err(_err) => false, // open process fail
    }
  }
  // ()
}

// ,PWSTR
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

    let pid = get_hwnd_pid(hwnd);
    // 3 get window title
    // let mut bytes: [u16; 500] = [0; 500];
    // let len = GetWindowTextW(hwnd, &mut bytes);
    // let title = String::from_utf16_lossy(&bytes[..len as usize]);
    let title = get_hwnd_title(hwnd);

    (pid, title)
  }
}

// [get progress path,name,description,handle](https://github.com/dimusic/active-win-pos-rs/blob/main/src/win/platform_api.rs)
pub fn close_process_handle(process_handle: HANDLE) -> () {
  let _ = unsafe { CloseHandle(process_handle) };
  ()
}
pub fn get_process_handle(process_id: u32) -> Result<HANDLE, ()> {
  let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) };

  handle.map_err(|_| ())
}
// QueryFullProcessImageNameW,PROCESS_NAME_WIN32,PWSTR
pub fn get_process_path(process_id: u32) -> Result<PathBuf, ()> {
  let process_handle = get_process_handle(process_id)?;
  let mut lpdw_size: u32 = MAX_PATH;
  let mut process_path_raw = vec![0; MAX_PATH as usize];
  let process_path_pwstr = PWSTR::from_raw(process_path_raw.as_mut_ptr());

  let process_path = unsafe {
    let success = QueryFullProcessImageNameW(
      process_handle,
      PROCESS_NAME_WIN32,
      process_path_pwstr,
      &mut lpdw_size,
    );

    close_process_handle(process_handle);

    match success {
      Ok(_) => (),
      Err(_) => return Err(()),
    }
    // if !success.as_bool() {
    //   return Err(());
    // }

    process_path_pwstr.to_string().map_err(|_| ())?
  };

  Ok(Path::new(&process_path).to_path_buf())
}

// IsWindow(hwnd).as_bool()

use crate::geometry::Point;
use windows::Win32::UI::Input::KeyboardAndMouse::{
  MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSE_EVENT_FLAGS,
};
//  MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
// MOUSEEVENTF_MIDDLEUP,
//  MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,

use windows::Win32::UI::WindowsAndMessaging::{
  GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
};

// code(core): def inner fn get_mouse_position_in_screen to get mouse position
// code(core): use struct windows::Win32::Foundation::POINT to set mouse initial value
// code(core): use fn windows::Win32::UI::WindowsAndMessaging::GetCursorPos to get mouse position
// code(core): use fn sophia::geometry::Point::new to make point and as result

pub fn get_mouse_position_in_screen() -> Point {
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
pub fn get_mouse_position_in_window(hwnd: HWND) -> Point {
  // get mouse position in screen
  let pos = get_mouse_position_in_screen();
  // get window rect
  let rect = get_hwnd_rect(hwnd);
  // pos
  Point {
    x: pos.x - rect.left,
    y: pos.y - rect.top,
  }
}
// code(core): def inner fn mouse_move_inner to move mouse
// code(core): use const windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_MOVE
// code(core): use const windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE

pub fn mouse_move_inner(x: i32, y: i32) {
  mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x, y, 0, 0);
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
