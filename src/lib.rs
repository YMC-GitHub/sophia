#![deny(clippy::all)]

/// import the preludes
// use napi::bindgen_prelude::*;
use napi_derive::napi;
#[napi]
fn fib(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fib(n - 1) + fib(n - 2),
  }
}

pub mod geometry;
pub mod screen;
pub mod utils;

#[cfg(target_os = "windows")]
pub mod win;
