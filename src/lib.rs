#![no_std]
#![feature(c_size_t)]
extern crate nalgebra as na;
use na::{Dyn, Matrix4, OMatrix};

use core::ffi::{c_double, c_int, c_float};
// 二维矩阵
// use ndarray::{array, Array2};
use panic_halt as _;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use static_alloc::Bump;
#[global_allocator]
static A: Bump<[u8; 1 << 16]> = Bump::uninit();
type DMatrixi32 = OMatrix<f32, Dyn, Dyn>;

#[no_mangle]
pub extern "C" fn test() -> f32 {
    let arr = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.3];
    let arr2 = DMatrixi32::from_vec(3, 3, arr);
    let qm = arr2.qr().q();
    qm.sum()
}

#[no_mangle]
pub extern "C" fn add_two(a: c_int, b: c_int) -> c_int {
    return a + b;
}

#[no_mangle]
pub extern "C" fn ptrtest(a: *mut c_double) -> *const c_double {
    unsafe {
        *a = 3.0;
        *a.add(1) = 0_f64;
    }
    a
}