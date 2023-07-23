#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

//extern crate libc;
//use libc::c_int;
use std::os::raw::{c_int, c_void};
use std::alloc::{alloc, dealloc, Layout};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

#[repr(C)]
struct MyClass {
    _private: [u8; 128],
}

extern "C" {
    fn addxx(mem: *mut MyClass, a: c_int, b :c_int, c: *mut c_int);
    fn subxxx(a: c_int, b :c_int, c: *mut c_int);
    fn addzz( mem: *mut MyClass, a: c_int, b :c_int, c: *mut c_int );
}

#[no_mangle]
pub extern "C" fn _sbrk(shift: u32) -> u32 {
    0
}

pub fn main() {
    let a = 1;
    let b = 2;
    let mut result = 0;
    unsafe {
        let layout = Layout::new::<MyClass>();
        let ptr = alloc(layout) as *mut MyClass;
        addzz(ptr, a as c_int, b as c_int, &mut result as *mut c_int);
        // addxx( ptr, a, b, &mut result);
        subxxx(result, 5, &mut result);
        dealloc(ptr as *mut u8, layout);
    }
    println!("Result: {}", result);


    env::commit(&result);
}
