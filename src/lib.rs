// use libc::c_char;
// use core::ffi::CStr;
use ndarray::{array, Array2};

#[no_mangle]
pub extern "C" fn test() -> i32 {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let a = Array2::from_shape_vec((3, 3), arr).unwrap();
    let b = array![[9, 8, 7], [6, 5, 4], [3, 2, 1],];
    return a.dot(&b).sum();
}
