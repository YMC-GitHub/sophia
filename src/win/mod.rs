// code(core): use ./keyboard.rs as pub mod keyboard
// code(core): use ./memory.rs as pub mod memory
// code(core): use ./mouse.rs as pub mod mouse
// code(core): use ./screen.rs as pub mod screen
// code(core): use ./window.rs as pub mod window
// - code(core): use ./utils.rs as pub mod utils
pub mod keyboard;
pub mod memory;
pub mod mouse;
pub mod screen;
// pub mod utils;
pub mod window;
// next:
// mod platform_api;
// mod window_position;

// use crate::common::platform_api::PlatformApi;
// use platform_api::WindowsPlatformApi;

// pub fn init_platform_api() -> impl PlatformApi {
//     WindowsPlatformApi {}
// }
