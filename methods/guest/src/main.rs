#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

//extern crate libc;
//use libc::c_int;
use std::ffi::CString;
use std::os::raw::{c_int, c_void, c_uchar};
use std::alloc::{alloc, dealloc, Layout};
use risc0_zkvm::guest::env;
use hex;

use blake2b_simd;

risc0_zkvm::guest::entry!(main);

#[repr(C)]
struct MyClass {
    _private: [u8; 128],
}

extern "C" {
    fn addxx(mem: *mut MyClass, a: c_int, b :c_int, c: *mut c_int);
    fn subxxx(a: c_int, b :c_int, c: *mut c_int);
    fn addzz( mem: *mut MyClass, a: c_int, b :c_int, c: *mut c_int );
    fn calculate_hash( data: *const c_uchar, length: c_int, into: *mut c_uchar );
}

#[no_mangle]
pub extern "C" fn _sbrk(shift: u32) -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn blake_hash_calculator_create() -> *mut blake2b_simd::State {
    Box::into_raw(Box::new(blake2b_simd::Params::new().hash_length(32).to_state()))
}

#[no_mangle]
pub extern "C" fn blake_hash_update(hash_state: *mut blake2b_simd::State, data: *const u8, length: usize) {
    if hash_state.is_null() {
        return;
    }

    unsafe {
        let data_slice = std::slice::from_raw_parts(data, length);
        (*hash_state).update(data_slice);
    }
}

#[no_mangle]
pub extern "C" fn blake_hash_digest(hash_state: *mut blake2b_simd::State, into: *mut u8) {
    if hash_state.is_null() {
        return;
    }

    unsafe {
        let hash = (*hash_state).finalize();
        std::ptr::copy_nonoverlapping(hash.as_bytes().as_ptr(), into, 32);
    }
}

#[no_mangle]
pub extern "C" fn blake_hash_calculator_destroy(hash_state: *mut blake2b_simd::State) {
    if !hash_state.is_null() {
        unsafe {
            Box::from_raw(hash_state);
        }
    }
}

pub fn main() {
    let msg = "Kill `em all!";
    let c_string = CString::new(msg).unwrap();
    let msg_ptr = c_string.as_ptr();
    let msg_len = msg.len() as c_int;
    let mut hash = [0u8; 32];
    unsafe {
        calculate_hash(msg_ptr as *const c_uchar, msg_len, hash.as_mut_ptr());
    }
    println!("Hash: {}", hex::encode(hash));
    // let mut hasher = blake2b::new();
    // hasher.update(b"Kill `em all!");
    // let hash = hasher.finalize();
    let hash = blake2b_simd::Params::new()
        .hash_length(32)
        .to_state()
        .update(b"Kill `em all!")
        .finalize();
    println!("Hash: {}", hex::encode(hash));
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
