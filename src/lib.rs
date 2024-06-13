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
