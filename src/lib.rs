// use libc::c_char;
// use core::ffi::CStr;
// use ndarray::{array, Array2};
#![no_std]
use panic_halt as _;
#[no_mangle]
pub extern "C" fn test() -> i32 {
    3
}
