#![deny(clippy::all)]

// code(core): use macro deny(clippy::all)

// code(core): use ./geometry.rs as pub mod geometry
// code(core): use ./screen.rs as pub mod screen
// code(core): use ./utils.rs as pub mod utils
// code(core): use ./fib.rs as pub mod fib

pub mod fib;
pub mod geometry;
pub mod screen;
pub mod utils;

// code(core): use ./win/xx.rs as pub mod win
// code(core): use macros cfg(target_os = "windows") to label it
#[cfg(target_os = "windows")]
pub mod win;

// next:
// refer
// https://github.com/dimusic/active-win-pos-rs/blob/main/src/lib.rs

// code(core): use common/xx as pub mod common
// pub mod common;

// #[cfg(target_os = "linux")]
// mod linux;
// #[cfg(target_os = "macos")]
// mod mac;
// #[cfg(target_os = "windows")]
// mod win;

// #[cfg(target_os = "linux")]
// use linux::init_platform_api;
// #[cfg(target_os = "macos")]
// use mac::init_platform_api;
// #[cfg(target_os = "windows")]
// use win::init_platform_api;
// pub use common::active_window::ActiveWindow;
// use common::platform_api::PlatformApi;
// pub use common::window_position::WindowPosition;

// pub fn get_position() -> Result<WindowPosition, ()> {
//   let api = init_platform_api();
//   api.get_position()
// }

// pub fn get_active_window() -> Result<ActiveWindow, ()> {
//   let api = init_platform_api();
//   api.get_active_window()
// }
