// code(core): use napi_derive 's fn napi
// use napi::bindgen_prelude::*;
use napi_derive::napi;

// code(core): def fn fib
// code(core): use napi macro to label it
#[napi]
pub fn fib(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fib(n - 1) + fib(n - 2),
  }
}
