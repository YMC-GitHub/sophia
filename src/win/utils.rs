// [get active window in rust](https://hellocode.co/blog/post/tracking-active-process-windows-rust/)
use crate::geometry::{LParamFlag, Rect, WindowMetaInfo, WindowView};
use crate::utils::{decode_wide, encode_wide};
use crate::win::keyboard::{keyboard_press_key_global, keyboard_release_key_global};

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
  UI::Input::KeyboardAndMouse::{
    MapVirtualKeyA, MAP_VIRTUAL_KEY_TYPE, VK_A, VK_ADD, VK_B, VK_BACK, VK_C, VK_CAPITAL,
    VK_CONTROL, VK_D, VK_DECIMAL, VK_DELETE, VK_DIVIDE, VK_DOWN, VK_E, VK_END, VK_ESCAPE, VK_F,
    VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17, VK_F18, VK_F19, VK_F2,
    VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_G,
    VK_H, VK_HOME, VK_I, VK_INSERT, VK_J, VK_K, VK_L, VK_LBUTTON, VK_LCONTROL, VK_LEFT, VK_LMENU,
    VK_LSHIFT, VK_LWIN, VK_M, VK_MBUTTON, VK_MENU, VK_MULTIPLY, VK_N, VK_NEXT, VK_NUMLOCK,
    VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7,
    VK_NUMPAD8, VK_NUMPAD9, VK_O, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6,
    VK_OEM_7, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_P, VK_PAUSE, VK_PRIOR,
    VK_Q, VK_R, VK_RBUTTON, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_S,
    VK_SCROLL, VK_SEPARATOR, VK_SHIFT, VK_SNAPSHOT, VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U,
    VK_UP, VK_V, VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2, VK_Y, VK_Z,
  },
  UI::WindowsAndMessaging::{
    EnumChildWindows, FindWindowW, GetClassNameW, GetClientRect, GetDesktopWindow,
    GetForegroundWindow, GetWindowLongPtrW, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, SendMessageA, SendMessageW,
    SetForegroundWindow, SetWindowPos, ShowWindow, ShowWindowAsync, GWL_EXSTYLE, GWL_STYLE,
    SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, SW_HIDE, SW_SHOWNORMAL, WM_CHAR, WM_CLOSE, WM_KEYDOWN,
    WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE,
    WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_XBUTTONDOWN,
    WM_XBUTTONUP, WS_CHILD, WS_EX_TOOLWINDOW,
  },
};

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

use crate::geometry::Point;
use windows::Win32::UI::Input::KeyboardAndMouse::{
  MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSE_EVENT_FLAGS, VIRTUAL_KEY,
};
//  MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
// MOUSEEVENTF_MIDDLEUP,
//  MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,

use windows::Win32::UI::WindowsAndMessaging::{
  GetCursorPos, GetSystemMetrics, PostMessageA, SM_CXSCREEN, SM_CYSCREEN,
};

pub fn get_mouse_position_in_screen() -> Point {
  let mut position = windows::Win32::Foundation::POINT { x: 0, y: 0 };
  unsafe {
    let _ = GetCursorPos(&mut position);
  }

  Point::new(position.x, position.y)
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

// topic: coords relative fn
pub fn coords_from_rect(rect: Rect) -> Point {
  Point {
    x: rect.left,
    y: rect.top,
  }
}
pub fn coords_from_screen_to_window(hwnd: HWND, coords: Point) -> Point {
  // get mouse position in screen
  // let pos = get_mouse_position_in_screen();
  let rect = get_hwnd_rect(hwnd);
  let posi = coords_from_rect(rect);

  // pos
  Point {
    x: coords.x - posi.x,
    y: coords.y - posi.x,
  }
}
pub fn coords_move(base: Point, offset: Point) -> Point {
  Point {
    x: base.x + offset.x,
    y: base.y + offset.y,
  }
}

// topic: mouse relative fn in global
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

// topic: mouse relative fn in some hwnd

// [keysender's mouse in cpp](https://github.com/Krombik/keysender/blob/master/src/addon/mouse.cpp)
// Mouse::getMousePos need   mousePosGetter
// Mouse::toggleMb need mbToggler
// Mouse::scrollWheel need scrollWheeler
// Mouse::move need mover

// [keysender's mouse includ head in cpp](https://github.com/Krombik/keysender/blob/master/src/addon/mouse.hpp)
// use virtual::mousePosGetter as protected method
// use virtual::mbToggler as protected method
// use virtual::scrollWheeler as protected method
// use virtual::mover as protected method

// def Mouse::getMousePos as public method
// def Mouse::toggleMb as public method
// def Mouse::scrollWheel as public method
// def Mouse::move need mover as public method

// use includes.hpp as file head
pub fn make_lparam_for_mouse(coords: Point) -> LPARAM {
  let p = (coords.y << 16) | coords.x;
  let lp = LPARAM(p as isize);
  lp
}

///in keysender using  PostMessageA , here using SendMessageA
pub fn mouse_move_in_window_inner(hwnd: HWND, coords: Point) -> () {
  //   let message = unsafe{
  //     SendMessageW(HWND(0x003E0AF4), WM_KEYDOWN,WPARAM(VK_RETURN.0 as usize), LPARAM(0));
  //     SendMessageW(HWND(0x003E0AF4), WM_KEYUP,WPARAM(VK_RETURN.0 as usize), LPARAM(0))
  // };

  // MOUSEEVENTF_MOVE IN RUST, WM_MOUSEMOVE IN CPP ?
  // PostMessageA ? SendMessageA ?
  // WM_MOUSEMOVE vs MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
  unsafe {
    let _res = SendMessageA(hwnd, WM_MOUSEMOVE, WPARAM(1), make_lparam_for_mouse(coords));
  }
  // mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x, y, 0, 0);
  ()
}

// [the msg in SendMessageW rust](https://rustcc.cn/article?id=514f4f9d-1e7d-45af-92f0-980c492a0d07)
// [about SendMessage and mouse_event in cpp](https://blog.csdn.net/fuhanghang/article/details/118700752)
// [WM_MOUSEMOVE in windows in rust](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/constant.WM_MOUSEMOVE.html)

// 1. define mouse events hash-map stage 1
// [mouse events in cpp in keysender](https://github.com/Krombik/keysender/blob/master/src/addon/helper.cpp)
use std::collections::HashMap;
pub fn mouse_events() -> HashMap<String, [u32; 2]> {
  let teams_list = vec![
    ("left".to_string(), [WM_LBUTTONUP, WM_LBUTTONDOWN]),
    ("right".to_string(), [WM_RBUTTONUP, WM_RBUTTONDOWN]),
    ("middle".to_string(), [WM_MBUTTONUP, WM_MBUTTONDOWN]),
    ("x1".to_string(), [WM_XBUTTONUP, WM_XBUTTONDOWN]),
    ("x2".to_string(), [WM_XBUTTONUP, WM_XBUTTONDOWN]),
  ];

  let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

  teams_map
  // println!("{:?}", teams_map)
}
// [mouse events in rust in windows](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/index.html)

// 2. define mouse events hash-map stage 2
// [define const vars in rust with lazy_static](https://blog.csdn.net/u013623958/article/details/123082922)
use lazy_static::lazy_static;
lazy_static! {
  pub static ref MOUSE_EVENTS: HashMap<String, [u32; 2]> = {
    let events = mouse_events();
    events
  };
}

pub fn mouse_get_event(event: String) -> [u32; 2] {
  let res = MOUSE_EVENTS
    .get(&event)
    .copied()
    .unwrap_or([WM_LBUTTONUP, WM_LBUTTONDOWN]);
  res
}

pub fn mouse_get_event_msg(button: String, is_button_down: bool) -> u32 {
  mouse_get_event(button)[if is_button_down { 1 } else { 0 }]
}

// [mouse buttons in cpp in keysender](https://github.com/Krombik/keysender/blob/master/src/addon/helper.cpp#L132)
// [mouse buttons in rust in windows](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/index.html)
pub fn mouse_buttons() -> HashMap<String, VIRTUAL_KEY> {
  let teams_list = vec![
    ("left".to_string(), VK_LBUTTON),
    ("right".to_string(), VK_RBUTTON),
    ("middle".to_string(), VK_MBUTTON),
    ("x1".to_string(), VK_XBUTTON1),
    ("x2".to_string(), VK_XBUTTON2),
  ];

  let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
  teams_map
}
// VK_LBUTTON,VK_RBUTTON,VK_MBUTTON,VK_XBUTTON1,VK_XBUTTON2
lazy_static! {
  pub static ref MOUSE_BUTTONS: HashMap<String, VIRTUAL_KEY> = {
    let buttons = mouse_buttons();
    buttons
  };
}

pub fn mouse_get_button(button: String) -> VIRTUAL_KEY {
  let res = MOUSE_BUTTONS.get(&button).copied().unwrap_or(VK_LBUTTON);
  res
}

///in keysender using  PostMessageA , here using SendMessageA
pub fn mouse_toggle_in_window_inner(
  hwnd: HWND,
  coords: Point,
  button: String,
  is_button_down: bool,
) -> () {
  // unsafe {
  //   let _res = SendMessageA(hwnd, WM_MOUSEWHEEL, WPARAM(0), make_lparam(coords));
  // }
  unsafe {
    let _res = SendMessageA(
      hwnd,
      mouse_get_event_msg(button, is_button_down),
      WPARAM(0),
      make_lparam_for_mouse(coords),
    );
  }
  // use  WPARAM(0) ? use mouse_get_button(button)?
  // [use WPARAM(0) in this demo](https://blog.csdn.net/fuhanghang/article/details/118700752)
  // [use mouse_get_button(button) in this demo](https://github.com/Krombik/keysender/blob/master/src/addon/virtual.cpp#L26)
  ()
}

fn get_direction(is_up: bool) -> usize {
  if is_up {
    0
  } else {
    1
  }
}
///
/// NOTE
///
/// use last-coords in keysender, here pass coords.
pub fn mouse_wheel_scroll_in_window_inner(hwnd: HWND, coords: Point, is_up: bool) -> () {
  // let mut wcoord = Point {
  //   x: 0,
  //   y: coords.x * 120,
  // };
  // // wcoord.y = get_direction()
  // wcoord.y = 120;
  unsafe {
    let _res = SendMessageA(
      hwnd,
      WM_MOUSEWHEEL,
      WPARAM(get_direction(is_up)),
      make_lparam_for_mouse(coords),
    );
  }
  // SendMessageA(hWnd, WM_MOUSEWHEEL, MAKEWPARAM(x, MK_MBUTTON), MAKELPARAM(lastCoords.x, lastCoords.y));
  // SendMessageA(hWnd, WM_MOUSEWHEEL, MAKEWPARAM(0, x*120), MAKELPARAM(lastCoords.x, lastCoords.y))
  ()
}
// [makewparam in cpp](https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-makewparam)
// [wm-mousewheel in cpp in windows](https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-mousewheel)
// [PostMessage and WM_MOUSEWHEEL Description of simulated wheel events in cpp](https://blog.csdn.net/u013394556/article/details/97026505)
// SPY++ Tool for Windows Motion Capture ?
// [scrollWheel doesn't work in cpp in keysender](https://github.com/Krombik/keysender/issues/25)
// [PostMessage and WM_MOUSEWHEEL](https://forums.codeguru.com/showthread.php?509919-WM_MOUSEWHEEL-messages-never-arrive-Why)
// [SB_LINEUP](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.WPARAM.html?search=SB_LINEUP)
// [WHEEL_DELTA in rust in window](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/constant.WHEEL_DELTA.html)

// mouse click
// mouse move
// mouse move to
// mouse human move to
// mouse get pos
// mouse
// scroll wheel
// [using addon api in ts in keysender](https://github.com/Krombik/keysender/blob/master/src/worker.ts)

// [get delta from LPARAM in rust in emrebicer/mouce](https://github.com/emrebicer/mouce/blob/master/src/windows.rs#L268)
// [get point from LPARAM in rust in emrebicer/mouce](https://github.com/emrebicer/mouce/blob/master/src/windows.rs#L263)

// Virtual.mouse
// getPos
// move
// scroll wheel
// click
// press
// release

// Virtual.window

// Virtual.keyboard

// [SendMessage in cpp and lparam intro](https://blog.csdn.net/yizhe0731/article/details/103194401)
// [find window and send keyboard key to it in cpp demo](https://www.cnblogs.com/lidabo/p/16922309.html)
// PostMessageA(hWnd, isKeyDown ? WM_KEYDOWN : WM_KEYUP, key, 0 | (key << 16) | (0 << 24) | (0 << 29) | ((UINT)!isKeyDown << 30) | ((UINT)!isKeyDown << 31));

// [key toggler in cpp in keysender](https://github.com/Krombik/keysender/blob/master/src/addon/virtual.cpp#L47)
// [char print in cpp in keysender](https://github.com/Krombik/keysender/blob/master/src/addon/virtual.cpp#L49-L54)

// [keysender's keyboard in cpp](https://github.com/Krombik/keysender/blob/master/src/addon/keyboard.cpp)
// Keyboard::toggleKey need keyToggler,getKeyboardKeyCode
// Keyboard::printChar need charPrinter,bufferToWstring

// [keysender's keyboard include head in cpp](https://github.com/Krombik/keysender/blob/master/src/addon/keyboard.hpp)
// use virtual::keyToggler as private method
// use virtual::charPrinter as private method

// def Keyboard::toggleKey as public method
// def Keyboard::printChar as public method

// [assic control char from 0-31](https://blog.csdn.net/justhere_/article/details/108561427)
pub fn keyboard_keys_ascii_control() -> HashMap<String, VIRTUAL_KEY> {
  let teams_list = vec![
    ("A".to_string(), VIRTUAL_KEY(0x1 as u16)),
    ("B".to_string(), VIRTUAL_KEY(0x2 as u16)),
    ("C".to_string(), VIRTUAL_KEY(0x3 as u16)),
    ("D".to_string(), VIRTUAL_KEY(0x4 as u16)),
    ("E".to_string(), VIRTUAL_KEY(0x5 as u16)),
    ("F".to_string(), VIRTUAL_KEY(0x6 as u16)),
    ("G".to_string(), VIRTUAL_KEY(0x7 as u16)),
    ("H".to_string(), VIRTUAL_KEY(0x8 as u16)),
    ("I".to_string(), VIRTUAL_KEY(0x9 as u16)),
    ("J".to_string(), VIRTUAL_KEY(0x0a as u16)),
    ("K".to_string(), VIRTUAL_KEY(0x0b as u16)),
    ("L".to_string(), VIRTUAL_KEY(0x0c as u16)),
    ("M".to_string(), VIRTUAL_KEY(0x0d as u16)),
    ("N".to_string(), VIRTUAL_KEY(0x0e as u16)),
    ("O".to_string(), VIRTUAL_KEY(0x0f as u16)),
    ("P".to_string(), VIRTUAL_KEY(0x10 as u16)),
    ("Q".to_string(), VIRTUAL_KEY(0x11 as u16)),
    ("R".to_string(), VIRTUAL_KEY(0x12 as u16)),
    ("S".to_string(), VIRTUAL_KEY(0x13 as u16)),
    ("T".to_string(), VIRTUAL_KEY(0x14 as u16)),
    ("U".to_string(), VIRTUAL_KEY(0x15 as u16)),
    ("V".to_string(), VIRTUAL_KEY(0x16 as u16)),
    ("W".to_string(), VIRTUAL_KEY(0x17 as u16)),
    ("X".to_string(), VIRTUAL_KEY(0x18 as u16)),
    ("Y".to_string(), VIRTUAL_KEY(0x19 as u16)),
    ("Z".to_string(), VIRTUAL_KEY(0x1a as u16)),
  ];
  let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
  teams_map
}
pub fn keyboard_keys_global() -> HashMap<String, VIRTUAL_KEY> {
  let teams_list = vec![
    ("shift".to_string(), VK_SHIFT),
    ("ctrl".to_string(), VK_CONTROL),
    ("alt".to_string(), VK_MENU),
    ("lWin".to_string(), VK_LWIN),
    ("rWin".to_string(), VK_RWIN),
    ("lShift".to_string(), VK_LSHIFT),
    ("rShift".to_string(), VK_RSHIFT),
    ("lCtrl".to_string(), VK_LCONTROL),
    ("rCtrl".to_string(), VK_RCONTROL),
    ("lAlt".to_string(), VK_LMENU),
    ("rAlt".to_string(), VK_RMENU),
  ];
  let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
  teams_map
}
lazy_static! {
  pub static ref KEYBOARD_KEYS_GLOBAL: HashMap<String, VIRTUAL_KEY> = {
    let buttons = keyboard_keys_global();
    buttons
  };
}

pub fn keyboard_get_vk_g(key: String) -> VIRTUAL_KEY {
  let res: VIRTUAL_KEY = KEYBOARD_KEYS_GLOBAL
    .get(&key)
    .copied()
    .unwrap_or(VIRTUAL_KEY(0x0 as u16));
  res
}

// [keyboard keys in cpp in keysender](https://github.com/Krombik/keysender/blob/master/src/addon/helper.cpp#L134)
// [keyboard keys  in rust in windows](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/index.html#constants)
// VIRTUAL_KEY(key as u16)
pub fn keyboard_keys() -> HashMap<String, VIRTUAL_KEY> {
  let teams_list = vec![
    ("none".to_string(), VIRTUAL_KEY(0x0 as u16)),
    ("backspace".to_string(), VK_BACK),
    ("tab".to_string(), VK_TAB),
    ("enter".to_string(), VK_RETURN),
    ("shift".to_string(), VK_SHIFT),
    ("ctrl".to_string(), VK_CONTROL),
    ("alt".to_string(), VK_MENU),
    ("pause".to_string(), VK_PAUSE),
    ("capsLock".to_string(), VK_CAPITAL),
    ("escape".to_string(), VK_ESCAPE),
    ("space".to_string(), VK_SPACE),
    ("pageUp".to_string(), VK_PRIOR),
    ("pageDown".to_string(), VK_NEXT),
    ("end".to_string(), VK_END),
    ("home".to_string(), VK_HOME),
    ("left".to_string(), VK_LEFT),
    ("up".to_string(), VK_UP),
    ("right".to_string(), VK_RIGHT),
    ("down".to_string(), VK_DOWN),
    ("printScreen".to_string(), VK_SNAPSHOT),
    ("insert".to_string(), VK_INSERT),
    ("delete".to_string(), VK_DELETE),
    ("0".to_string(), VIRTUAL_KEY(0x30 as u16)),
    ("1".to_string(), VIRTUAL_KEY(0x31 as u16)),
    ("2".to_string(), VIRTUAL_KEY(0x32 as u16)),
    ("3".to_string(), VIRTUAL_KEY(0x33 as u16)),
    ("4".to_string(), VIRTUAL_KEY(0x34 as u16)),
    ("5".to_string(), VIRTUAL_KEY(0x35 as u16)),
    ("6".to_string(), VIRTUAL_KEY(0x36 as u16)),
    ("7".to_string(), VIRTUAL_KEY(0x37 as u16)),
    ("8".to_string(), VIRTUAL_KEY(0x38 as u16)),
    ("9".to_string(), VIRTUAL_KEY(0x39 as u16)),
    ("a".to_string(), VIRTUAL_KEY(0x41 as u16)),
    ("b".to_string(), VIRTUAL_KEY(0x42 as u16)),
    ("c".to_string(), VIRTUAL_KEY(0x43 as u16)),
    ("d".to_string(), VIRTUAL_KEY(0x44 as u16)),
    ("e".to_string(), VIRTUAL_KEY(0x45 as u16)),
    ("f".to_string(), VIRTUAL_KEY(0x46 as u16)),
    ("g".to_string(), VIRTUAL_KEY(0x47 as u16)),
    ("h".to_string(), VIRTUAL_KEY(0x48 as u16)),
    ("i".to_string(), VIRTUAL_KEY(0x49 as u16)),
    ("j".to_string(), VIRTUAL_KEY(0x4a as u16)),
    ("k".to_string(), VIRTUAL_KEY(0x4b as u16)),
    ("l".to_string(), VIRTUAL_KEY(0x4c as u16)),
    ("m".to_string(), VIRTUAL_KEY(0x4d as u16)),
    ("n".to_string(), VIRTUAL_KEY(0x4e as u16)),
    ("o".to_string(), VIRTUAL_KEY(0x4f as u16)),
    ("p".to_string(), VIRTUAL_KEY(0x50 as u16)),
    ("q".to_string(), VIRTUAL_KEY(0x51 as u16)),
    ("r".to_string(), VIRTUAL_KEY(0x52 as u16)),
    ("s".to_string(), VIRTUAL_KEY(0x53 as u16)),
    ("t".to_string(), VIRTUAL_KEY(0x54 as u16)),
    ("u".to_string(), VIRTUAL_KEY(0x55 as u16)),
    ("v".to_string(), VIRTUAL_KEY(0x56 as u16)),
    ("w".to_string(), VIRTUAL_KEY(0x57 as u16)),
    ("x".to_string(), VIRTUAL_KEY(0x58 as u16)),
    ("y".to_string(), VIRTUAL_KEY(0x59 as u16)),
    ("z".to_string(), VIRTUAL_KEY(0x5a as u16)),
    ("A".to_string(), VK_A),
    ("B".to_string(), VK_B),
    ("C".to_string(), VK_C),
    ("D".to_string(), VK_D),
    ("E".to_string(), VK_E),
    ("F".to_string(), VK_F),
    ("G".to_string(), VK_G),
    ("H".to_string(), VK_H),
    ("I".to_string(), VK_I),
    ("J".to_string(), VK_J),
    ("K".to_string(), VK_K),
    ("L".to_string(), VK_L),
    ("M".to_string(), VK_M),
    ("N".to_string(), VK_N),
    ("O".to_string(), VK_O),
    ("P".to_string(), VK_P),
    ("Q".to_string(), VK_Q),
    ("R".to_string(), VK_R),
    ("S".to_string(), VK_S),
    ("T".to_string(), VK_T),
    ("U".to_string(), VK_U),
    ("V".to_string(), VK_V),
    ("W".to_string(), VK_W),
    ("X".to_string(), VK_X),
    ("Y".to_string(), VK_Y),
    ("Z".to_string(), VK_Z),
    ("lWin".to_string(), VK_LWIN),
    ("rWin".to_string(), VK_RWIN),
    ("num0".to_string(), VK_NUMPAD0),
    ("num0".to_string(), VK_NUMPAD0),
    ("num1".to_string(), VK_NUMPAD1),
    ("num2".to_string(), VK_NUMPAD2),
    ("num3".to_string(), VK_NUMPAD3),
    ("num4".to_string(), VK_NUMPAD4),
    ("num5".to_string(), VK_NUMPAD5),
    ("num6".to_string(), VK_NUMPAD6),
    ("num7".to_string(), VK_NUMPAD7),
    ("num8".to_string(), VK_NUMPAD8),
    ("num9".to_string(), VK_NUMPAD9),
    ("num*".to_string(), VK_MULTIPLY),
    ("num+".to_string(), VK_ADD),
    ("num,".to_string(), VK_SEPARATOR),
    ("num-".to_string(), VK_SUBTRACT),
    ("num.".to_string(), VK_DECIMAL),
    ("num/".to_string(), VK_DIVIDE),
    ("f1".to_string(), VK_F1),
    ("f2".to_string(), VK_F2),
    ("f3".to_string(), VK_F3),
    ("f4".to_string(), VK_F4),
    ("f5".to_string(), VK_F5),
    ("f6".to_string(), VK_F6),
    ("f7".to_string(), VK_F7),
    ("f8".to_string(), VK_F8),
    ("f9".to_string(), VK_F9),
    ("f10".to_string(), VK_F10),
    ("f11".to_string(), VK_F11),
    ("f12".to_string(), VK_F12),
    ("f13".to_string(), VK_F13),
    ("f14".to_string(), VK_F14),
    ("f15".to_string(), VK_F15),
    ("f16".to_string(), VK_F16),
    ("f17".to_string(), VK_F17),
    ("f18".to_string(), VK_F18),
    ("f19".to_string(), VK_F19),
    ("f20".to_string(), VK_F20),
    ("f21".to_string(), VK_F21),
    ("f22".to_string(), VK_F22),
    ("f23".to_string(), VK_F23),
    ("f24".to_string(), VK_F24),
    ("numLock".to_string(), VK_NUMLOCK),
    ("scrollLock".to_string(), VK_SCROLL),
    ("lShift".to_string(), VK_LSHIFT),
    ("rShift".to_string(), VK_RSHIFT),
    ("lCtrl".to_string(), VK_LCONTROL),
    ("rCtrl".to_string(), VK_RCONTROL),
    ("lAlt".to_string(), VK_LMENU),
    ("rAlt".to_string(), VK_RMENU),
    (";".to_string(), VK_OEM_1),
    ("=".to_string(), VK_OEM_PLUS),
    (".".to_string(), VK_OEM_COMMA),
    ("-".to_string(), VK_OEM_MINUS),
    (".".to_string(), VK_OEM_PERIOD),
    ("/".to_string(), VK_OEM_2),
    ("`".to_string(), VK_OEM_3),
    ("[".to_string(), VK_OEM_4),
    ("\\".to_string(), VK_OEM_5),
    ("]".to_string(), VK_OEM_6),
    ("'".to_string(), VK_OEM_7),
  ];
  let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
  teams_map
}

lazy_static! {
  pub static ref KEYBOARD_KEYS: HashMap<String, VIRTUAL_KEY> = {
    let buttons = keyboard_keys();
    buttons
  };
}

pub fn keyboard_get_vk(key: String) -> VIRTUAL_KEY {
  let res: VIRTUAL_KEY = KEYBOARD_KEYS
    .get(&key)
    .copied()
    .unwrap_or(VIRTUAL_KEY(0x0 as u16));
  res
  // let c = res.0 as _;
  // [transfrom VIRTUAL_KEY to number in rust in window](https://rustcc.cn/article?id=514f4f9d-1e7d-45af-92f0-980c492a0d07)
  // c
  // [getKeyboardKeyCode in cpp in keysender](https://github.com/Krombik/keysender/blob/master/src/addon/helper.cpp#L121)
}

// [cpp send control key to some hwnd](https://www.cnblogs.com/songtaoyu/p/3474628.html)
///  ky_get_key_type(ismod, is_key_down)
fn ky_get_key_type(use_sys: bool, is_key_down: bool) -> u32 {
  if use_sys {
    if is_key_down {
      WM_SYSKEYDOWN
    } else {
      WM_SYSKEYUP
    }
  } else {
    if is_key_down {
      WM_KEYDOWN
    } else {
      WM_KEYUP
    }
  }
}

pub fn info_key(key: &str, code: i32, lparam: u32, is_key_down: bool) {
  let nameofkeyaction = if is_key_down { "down" } else { "up" };
  // ox:{:x}
  print!(
    "name:{},code:{},lparam:{:08X},action:{}\n",
    key, code, lparam, nameofkeyaction
  );
}
///
pub fn key_is_one_of_them(key: &str, modkeys: Vec<&str>) -> bool {
  let mut key_is_mod = false;
  let keylower = key.to_lowercase();
  for x in modkeys.clone().into_iter() {
    if keylower == x {
      key_is_mod = true;
      break;
    }
  }
  key_is_mod
}

// [1. use spy++ to get message for some window](https://www.cnblogs.com/songtaoyu/p/3474628.html)
// [2. transfrom message data for sendMessage](https://www.cnblogs.com/songtaoyu/p/3474628.html)
// [3. the value for VM_KEYDOWN,VM_KEYUP,VM_CHAR](https://www.cnblogs.com/songtaoyu/p/3474628.html)
// WM_KEYDOWN,WM_KEYUP,WM_CHAR in rust in windows::Win32::UI::WindowsAndMessaging
// ox100,0x101,0x102

// WM_SYSKEYDOWN,WM_SYSCHAR,WM_SYSKEYUP in rust in windows::Win32::UI::WindowsAndMessaging
// 0x104,0x106,0x105

// [4. the value for lparam](https://www.cnblogs.com/songtaoyu/p/3474628.html)
// 0-15 bit indicates how many times it was sent
// 16-23 Indicates ALT, CTRL, NUM, CAPS and other scan keys
// 24 bits to indicate left ALT, CTRL or right ALT, CTRL (usually 0)
// 25-28 bits are reserved
// 30 bits indicate the state of the previous key KEY DOWN OR UP, 1 is DWON state before sending, 0 is UP
// 31 bits: 0 means key start press, 1 means key start release

// [1.0 info about winspy](https://amandaguglieri.github.io/hackinglife/winspy/)
// [1.1 install winspy in window with scoop - scoop install winspy]()

// pub struct LparamProp {
//   repeat_count: u32,
//   scan_code: u32,
//   is_extend_key: bool,
//   with_extend_key: bool,
//   is_context_key_down: bool,
//   is_key_down: bool,
// }
// impl LparamProp {
//   pub fn default() -> LparamProp {
//     LparamProp {
//       repeat_count: 0,
//       scan_code: 0,
//       is_extend_key: false,
//       is_key_down: true,
//       is_context_key_down: false,
//       with_extend_key: false,
//     }
//   }
// }

// pub fn lparam_bool_to_u32(is_true: bool, reverse: bool) -> u32 {
//   let flag = if reverse { !is_true } else { is_true };
//   let res = if flag { 1 } else { 0 };
//   if reverse {}
//   res
// }

// pub fn lparam_from_vm_char_docs(key_code: u32, o: LparamProp) -> u32 {
//   // [wm-char intro in cpp in zh](https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-char)
//   // [wm-char intro in cpp in en](https://learn.microsoft.com/en/windows/win32/inputdev/wm-char)
//   let lparam = 0
//     | (key_code << 16)
//     | ((lparam_bool_to_u32(o.is_extend_key, false)) << 24)
//     | ((lparam_bool_to_u32(o.with_extend_key, false)) << 29)
//     | ((lparam_bool_to_u32(o.is_context_key_down, false)) << 30)
//     | ((lparam_bool_to_u32(o.is_key_down, false)) << 31);
//   lparam
// }

// pub fn lparam_from_vm_keydown_docs(key_code: u32, o: LparamProp) -> u32 {
//   // [wm-keydown intro in cpp in zh](https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-keydown)
//   // [wm-keydown intro in cpp in en](https://learn.microsoft.com/en/windows/win32/inputdev/wm-keydown)
//   let lparam = 0
//     | (key_code << 16)
//     | ((lparam_bool_to_u32(o.is_extend_key, false)) << 24)
//     | (0 << 29)
//     | ((lparam_bool_to_u32(o.is_context_key_down, true)) << 30)
//     | (0 << 31);
//   lparam
// }
// pub fn lparam_from_vm_keyup_docs(key_code: u32, o: LparamProp) -> u32 {
//   // [wm-keyup intro in cpp in zh](https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-keyup)
//   // [wm-keyup intro in cpp in en](https://learn.microsoft.com/en/windows/win32/inputdev/wm-keyup)
//   let lparam = 1
//     | (key_code << 16)
//     | ((lparam_bool_to_u32(o.is_extend_key, false)) << 24)
//     | (0 << 29)
//     | (1 << 30)
//     | (1 << 31);
//   lparam
// }

// pub fn lapram_from_keysender(key_code: u32, is_key_down: bool, is_context_key_down: bool) -> u32 {
//   let lparam = 0
//     | (key_code << 16)
//     | (0 << 24)
//     | 0 << 29
//     | ((lparam_bool_to_u32(is_context_key_down, true)) << 30)
//     | ((lparam_bool_to_u32(is_key_down, false)) << 31);
//   lparam
// }

// [PostMessage Background Send key combination in cpp](https://blog.csdn.net/SysProgram/article/details/45171493)
// [sending-right-altc-with-postmessage](https://stackoverflow.com/questions/11384669/sending-right-altc-with-postmessage)
// [use PostMessage with the windows-rs crate?](https://stackoverflow.com/questions/70349397/how-do-you-use-postmessage-with-the-windows-rs-crate)
// [parse wparam and lparam in rust](https://github.com/lhiuming/violet/blob/fcf59c4c6e6e04dd6358257b79d15444127eff32/crates/violet-app/src/window.rs#L359)
// [create lparam, vk to scan code,send_key_up,send_key_down,send_key](https://github.com/dev-boyenn/rulti-linux/blob/822b95193b7872d9d923b12c57d17c36c451fb70/src/keyboardutils.rs#L55)

// pub fn keyboard_send_keypress(hwnd: HWND, key: VIRTUAL_KEY) {
//   keyboard_send_keydown(hwnd, key);
//   keyboard_send_keyup(hwnd, key);
// }

// pub fn lparam_flag_preset_keydown() -> LParamFlag {
//   LParamFlag {
//     scan_code: 0,
//     repeat_count: 1,
//     transition_state: true,
//     previous_key_state: false,
//     context_code: false,
//     is_extended: false,
//   }
// }
// pub fn lparam_flag_preset_keyup() -> LParamFlag {
//   LParamFlag {
//     scan_code: 0,
//     repeat_count: 1,
//     transition_state: true,
//     previous_key_state: false,
//     context_code: false,
//     is_extended: true,
//   }
// }
// pub fn keyboard_send_keydown(hwnd: HWND, key: VIRTUAL_KEY) {
//   unsafe {
//     let res = PostMessageA(
//       hwnd,
//       WM_KEYDOWN,
//       WPARAM(key.0 as usize),
//       lparam_from_u32(lparam_u32_from_flag(key, lparam_flag_preset_keydown())),
//     );

//     // lparam_from_isize(lparam_isize_from_flag(key, 1, false, false, false))

//     match res {
//       Ok(_) => {
//         print!("keyboard send {} keydown done\n", key.0);
//         return ();
//       }
//       Err(err) => {
//         print!("keyboard send {} keydown fail\n", key.0);
//         print!("{}", err);
//         return ();
//       }
//     }
//   }
// }
// pub fn keyboard_send_keyup(hwnd: HWND, key: VIRTUAL_KEY) {
//   unsafe {
//     let res = PostMessageA(
//       hwnd,
//       WM_KEYUP,
//       WPARAM(key.0 as usize),
//       lparam_from_u32(lparam_u32_from_flag(key, lparam_flag_preset_keyup())),
//     );
//     match res {
//       Ok(_) => {
//         print!("keyboard send {} keyup done\n", key.0);
//         return ();
//       }
//       Err(err) => {
//         print!("keyboard send {} keyup fail\n", key.0);
//         print!("{}", err);
//         return ();
//       }
//     }
//   }
// }

// // pub fn click_top_left(hwnd: HWND) {
// //     unsafe { PostMessageA(hwnd, 0x0201, WPARAM(1), LPARAM(0)) };
// // }

/// transform virtual key to scan code and check it if it is extend key
///
/// flow: VIRTUAL_KEY -> u32 -> scan_code
pub fn virtual_key_to_scan_code(virtual_key: VIRTUAL_KEY) -> (u32, bool) {
  // 1
  let scan_code = unsafe { MapVirtualKeyA(virtual_key.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) as u32 };

  // 2
  let is_extended = virtual_key_is_extended(virtual_key);
  (scan_code, is_extended)
}

pub fn virtual_key_is_extended(virtual_key: VIRTUAL_KEY) -> bool {
  // 2
  let mut is_extended = false;
  match virtual_key {
    VK_RMENU | VK_RCONTROL | VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN | VK_PRIOR | VK_NEXT | VK_END
    | VK_HOME | VK_INSERT | VK_DELETE | VK_DIVIDE | VK_NUMLOCK => {
      is_extended = true;
    }
    _ => {}
  }
  is_extended
}

///
/// SAMPLE
///
/// lparam_info(0x20380001);
pub fn lparam_info(l_param: u32) {
  let lpf: LParamFlag = lparam_u32_to_flag(l_param);

  print!(
    "scan_code:{},repeat_count:{},transition_state:{},is_extended:{},previous_key_state:{},context_code:{}\n",
    lpf.scan_code, lpf.repeat_count, lpf.transition_state, lpf.is_extended, lpf.previous_key_state,lpf.context_code
  );

  // ox:{:x}
  // print!("lparam:{:08X}\n", l_param);
}

pub fn lparam_from_u32(val: u32) -> LPARAM {
  LPARAM(val as isize)
}
pub fn lparam_to_u32(val: LPARAM) -> u32 {
  val.0 as u32
}
pub fn lparam_u32_to_flag(v: u32) -> LParamFlag {
  let transition_state = ((v >> 31) & 0x1) > 0;
  let previous_key_state: bool = ((v >> 30) & 0x1) > 0;
  let context_code = ((v >> 29) & 0x1) > 0;
  //   let is_extended = virtual_key_is_extended(VIRTUAL_KEY(scan_code as u16));
  let is_extended = ((v >> 24) & 0x1) > 0;
  let scan_code = (v >> 16) as u32;
  let repeat_count = (v & 0xFFFF) as u32;
  LParamFlag {
    repeat_count,
    scan_code,
    is_extended,
    context_code,
    previous_key_state,
    transition_state,
  }
}
/// keyboard send virtual key util - create l param
///
///
pub fn lparam_u32_from_flag(virtual_key: VIRTUAL_KEY, flag: LParamFlag) -> u32 {
  let (scan_code, is_extended) = virtual_key_to_scan_code(virtual_key);
  let lpv = ((flag.transition_state as u32) << 31)
    | ((flag.previous_key_state as u32) << 30)
    | ((flag.context_code as u32) << 29)
    | ((is_extended as u32) << 24)
    | ((scan_code as u32) << 16)
    | flag.repeat_count as u32;
  lpv
  // let l_param = LPARAM(lpv);
  // l_param
}

pub fn vk_from_u16(i: u16) -> VIRTUAL_KEY {
  VIRTUAL_KEY(i)
}
pub fn vk_to_u16(i: VIRTUAL_KEY) -> u16 {
  i.0
}

// [object-orientation in rust](https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html)
// trait Show {
//   fn show(&self) -> String;
// }

// impl Show for i32 {
//   fn show(&self) -> String {
//       format!("four-byte signed {}", self)
//   }
// }

// impl Show for f64 {
//   fn show(&self) -> String {
//       format!("eight-byte float {}", self)
//   }
// }

// trait Quack {
//   fn quack(&self);
// }

// struct Duck ();

// impl Quack for Duck {
//   fn quack(&self) {
//       println!("quack!");
//   }
// }

// struct RandomBird {
//   is_a_parrot: bool
// }

// impl Quack for RandomBird {
//   fn quack(&self) {
//       if ! self.is_a_parrot {
//           println!("quack!");
//       } else {
//           println!("squawk!");
//       }
//   }
// }
// todo: slpit data,handle,opts

#[derive(Debug, Clone, Copy)]
pub struct LparamUtil {
  c_repeat: u32,
  scan_code: u32,
  f_extended: u32,
  f_alt_down: u32,
  f_repeat: u32,
  f_up: u32,
}

impl LparamUtil {
  /// let lparamf = LparamFlag::new();
  pub fn default() -> LparamUtil {
    LparamUtil {
      c_repeat: 0,
      scan_code: 0,
      f_extended: 0,
      f_alt_down: 0,
      f_repeat: 0,
      f_up: 0,
    }
  }

  /// DEMO
  ///
  /// lparamf.use_key(VK_V);
  pub fn use_key(&mut self, virtual_key: VIRTUAL_KEY) -> () {
    let (scan_code, f_extended) = virtual_key_to_scan_code(virtual_key);
    // let o = LparamFlag::default();
    self.scan_code = scan_code;
    self.f_extended = if f_extended { 1 } else { 0 };
    ()
  }
  /// DEMO
  ///
  ///  lparamf.use_key(VK_V);lparamf.use_keydown_preset();
  ///
  /// DOCS
  ///
  /// https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-keydown
  pub fn use_keydown_preset(&mut self) -> () {
    self.c_repeat = 1;
    self.f_alt_down = 0;
    self.f_repeat = 0;
    self.f_up = 0;
    ()
  }

  /// DEMO
  ///
  ///  lparamf.use_key(VK_V);lparamf.use_keyup_preset();
  ///
  /// DOCS
  ///
  /// https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-keyup
  pub fn use_keyup_preset(&mut self) -> () {
    self.c_repeat = 1;
    self.f_alt_down = 0;
    self.f_repeat = 1;
    self.f_up = 1;
    ()
  }
  //
  /// DEMO
  ///
  ///  lparamf.use_key(VK_V);lparamf.use_char_preset();
  ///
  /// DOCS
  ///
  /// https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-char
  pub fn use_char_preset(&mut self) -> () {
    self.c_repeat = 1;
    self.f_alt_down = 0; // diffrent for keydown and keyup
    self.f_repeat = 1; // diffrent for keydown and keyup
    self.f_up = 1;
    ()
  }

  pub fn use_keysender_preset(&mut self) -> () {
    self.c_repeat = 0; //0-15
    self.f_extended = 0; //24
    self.f_alt_down = 0; //29
    ()
  }

  /// lparamf.clone()
  pub fn clone(self) -> LparamUtil {
    LparamUtil {
      c_repeat: self.c_repeat,
      scan_code: self.scan_code,
      f_extended: self.f_extended,
      f_alt_down: self.f_alt_down,
      f_repeat: self.f_repeat,
      f_up: self.f_up,
    }
  }
  /// lparamf.up()
  pub fn keyup(&mut self) -> () {
    self.f_up = 1;
    ()
  }
  /// lparamf.down()
  pub fn keydown(&mut self) -> () {
    self.f_up = 0;
    ()
  }
  /// demo
  ///
  /// lparamf.use_keydown_preset();lparamf.prev_key_down(false);
  ///
  /// lparamf.use_char_preset(); lparamf.prev_key_down(true);
  ///
  /// docs
  ///
  /// https://learn.microsoft.com/zh-cn/windows/win32/inputdev/wm-keydown
  pub fn prev_key_down(mut self, is_char: bool) -> () {
    self.f_repeat = 1; // 1 for prev_key_down
    if is_char {
      self.f_repeat = 1;
    }
    // self
    ()
  }

  /// demo
  ///
  /// lparamf.use_keydown_preset().prev_key_up(false);
  ///
  pub fn prev_key_up(&mut self, is_char: bool) -> () {
    self.f_repeat = 0; // 0 for prev_key_down
    if is_char {
      self.f_repeat = 0;
    }
    // self
    ()
  }

  pub fn press_key_long(&mut self, is_char: bool) -> () {
    self.f_repeat = 1;
    if is_char {
      self.f_repeat = 0;
    }
    // self
    ()
  }

  pub fn alt_down(&mut self, is_char: bool) -> () {
    self.f_alt_down = 0;
    if is_char {
      self.f_alt_down = 1;
    }
    // self
    ()
  }
  pub fn alt_up(&mut self, is_char: bool) -> () {
    self.f_alt_down = 1;
    if is_char {
      self.f_alt_down = 0;
    }
    ()
  }

  pub fn to_u32(self) -> u32 {
    let lpv = ((self.f_up as u32) << 31)
      | ((self.f_repeat as u32) << 30)
      | ((self.f_alt_down as u32) << 29)
      | ((self.f_extended as u32) << 24)
      | ((self.scan_code as u32) << 16)
      | self.c_repeat as u32;
    lpv
  }
  pub fn to_str(&self) -> String {
    // let lpv = self.to_u32();
    // LParam:{:08X},LParam:{:016b}
    format!(
      "c_repeat:{},scan_code:{:X},f_extended:{},f_alt_down:{},f_repeat:{},f_up:{}",
      self.c_repeat, self.scan_code, self.f_extended, self.f_alt_down, self.f_repeat, self.f_up,
    )
  }
}
///in keysender using  PostMessageA , here using SendMessageA
pub fn keyboard_send_combine_key_in_window_inner(
  hwnd: HWND,
  key: String,
  is_key_down: bool,
  is_prev_key_down: bool,
) -> () {
  let vk = keyboard_get_vk(key.clone());
  unsafe {
    let mut lu = LparamUtil::default();
    lu.use_key(vk);
    // lu.use_keysender_preset();
    if is_key_down {
      lu.use_keydown_preset();
    } else {
      lu.use_keyup_preset();
    }

    if is_prev_key_down {
      lu.prev_key_down(false);
    }

    let lparam_u32 = lu.to_u32();

    let msg = ky_get_key_type(false, is_key_down);
    let wparam = WPARAM(vk.0 as _);
    let lparam = LPARAM(lparam_u32 as isize);
    let debug = false;
    if debug {
      print!(
        "{} [wParam:{:016X} lParam:{:016X}]\n",
        lu.to_str(),
        vk.0 as u32,
        lparam_u32
      );
    }

    // let global_keys = ["ctrl", "alt", "shift", "win"];
    // let is_gk = key_is_one_of_them(&key, global_keys.to_vec());
    // if is_gk.0 > 0 {
    //   keyboard_press_key_global(vk.0);
    //   return ();
    // }

    let vk_g = keyboard_get_vk_g(key.clone());
    if vk_g.0 > 0 {
      if is_key_down {
        keyboard_press_key_global(vk.0);
      } else {
        keyboard_release_key_global(vk.0);
      }
      return ();
    }

    let use_s = false;
    if use_s {
      SendMessageA(hwnd, msg, wparam, lparam);
      ()
    } else {
      let res = PostMessageA(hwnd, msg, wparam, lparam);
      match res {
        Ok(_) => {
          // print!("keyboard send {} key done\n", vk.0);
          return ();
        }
        Err(err) => {
          print!("keyboard send {} key fail\n", vk.0);
          print!("{}", err);
          return ();
        }
      }
    }
  }
  // ()
}

///in keysender using  PostMessageA , here using SendMessageA
pub fn keyboard_toggle_key_in_window_inner(
  hwnd: HWND,
  key: String,
  is_key_down: bool,
  is_prev_key_down: bool,
) -> () {
  let vk = keyboard_get_vk(key.clone());
  // [virtual-key-codes in window in cpp docs](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
  // [mock alt + a in cpp with PostMessage](https://www.cnblogs.com/Jnshushi99/archive/2011/09/03/2164617.html)
  // [it is best to mock mouse buttion in cpp with SendMessage](https://www.cnblogs.com/Jnshushi99/archive/2011/09/03/2164617.html)

  unsafe {
    let mut lu = LparamUtil::default();
    lu.use_key(vk);
    // lu.use_keysender_preset();
    if is_key_down {
      lu.use_keydown_preset();
    } else {
      lu.use_keyup_preset();
    }

    if is_prev_key_down {
      lu.prev_key_down(false);
    }

    let lparam_u32 = lu.to_u32();

    let msg = ky_get_key_type(false, is_key_down);
    let wparam = WPARAM(vk.0 as _);
    let lparam = LPARAM(lparam_u32 as isize);

    // let ch_char_code = 86 as u16 & 0xff;
    // let ch_char_code2 = 86 as u16 >> 8;
    // print!(
    //   "code:{},chCharCode1:{},chCharCode2:{},wparam:{:016X}\n",
    //   86 as u16, ch_char_code, ch_char_code2, 22
    // );

    // print!(
    //   "{} [wParam:{:016X} lParam:{:016X}]\n",
    //   lu.to_str(),
    //   vk.0 as u32,
    //   lparam_u32
    // );

    let use_s = false;
    if use_s {
      SendMessageA(hwnd, msg, wparam, lparam);
      ()
    } else {
      let res = PostMessageA(hwnd, msg, wparam, lparam);
      match res {
        Ok(_) => {
          // print!("keyboard send {} key done\n", vk.0);
          return ();
        }
        Err(err) => {
          print!("keyboard send {} key fail\n", vk.0);
          print!("{}", err);
          return ();
        }
      }
    }
  }
  // ()
}
// [keyboard button types in ts in keysender](https://github.com/Krombik/keysender/blob/master/src/types/index.ts)

///in keysender using  SendMessageW , here using SendMessageW
pub fn keyboard_print_char_in_window_inner(hwnd: HWND, text: String) -> () {
  // WM_CHAR = 258u32 in rust in window.
  // [about std::wstring and std::stringin cpp]()
  //
  unsafe {
    // str from u8 to u16
    // let text = text.encode_utf16().collect::<Vec<_>>();
    let text_u16 = encode_wide(text);
    // print!("text in rust in inner: {}", decode_wide(&text_u16));
    for txtcode in text_u16 {
      let _res = SendMessageW(hwnd, WM_CHAR, WPARAM(txtcode as usize), LPARAM(0));
    }
    // [about the different of SendMessageW and SendMessageA](https://blog.csdn.net/notzuonotdied/article/details/70788937)
    // use SendMessageW to fix unead-able code for non-english char.
  }
  ()
}

pub fn bytes_from_u8_to_u16(bytes: Vec<u8>) -> Vec<u16> {
  // let slice: &[i32] = &[1, 2, 3, 4, 5];
  let mut res = Vec::new();
  let mut index = 0;
  while index < bytes.len() {
    // println!("Number: {}", slice[index]);
    res.push(bytes[index] as u16 | (bytes[index + 1] as u16) << 8);
    index += 2;
  }
  res
}

// pub fn last_coords() -> HashMap<String, u32> {
//   let teams_list = vec![("x".to_string(), 0), ("y".to_string(), 0)];

//   let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
//   teams_map
// }
// lazy_static! {
//   pub static ref VIRTUAL_LAST_COORD: HashMap<String, u32> = {
//     let coords = last_coords();
//     coords
//   };
// }
