#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

//extern crate libc;
//use libc::c_int;
use std::os::raw::c_int;
use std::alloc::{alloc, dealloc, Layout};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

#[repr(C)]
struct MyClass {
    _private: [u8; 16],
}

extern "C" {
    fn addxx(a: c_int, b :c_int, c: *mut c_int);
    fn subxxx(a: c_int, b :c_int, c: *mut c_int);
}

#[no_mangle]
pub extern "C" fn _sbrk(shift: u32) -> u32 {
    println!("sbrk: {}", shift);
    0
}

pub fn main() {
    let a: i32 = 1;
    let b: i32 = 2;
    let mut result: i32 = 0;
    unsafe {
        addxx(a as c_int, 2 as c_int, &mut result as *mut c_int);
        subxxx(result as c_int, 5 as c_int, &mut result as *mut c_int);
    }
    println!("Result: {}", result);


    env::commit(&result);
}
