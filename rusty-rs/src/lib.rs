#![no_std]
#![feature(asm)]

extern crate panic_abort;

mod error;
mod syscall;

use core::slice;

#[no_mangle]
pub extern "C" fn update_label(ptr: *mut u8, len: usize) {
    let buf = unsafe { slice::from_raw_parts_mut(ptr, len) };

    if let Err(_) = syscall::cx_rng(buf) {
        buf[0] = 'E' as u8;
        buf[1] = 'r' as u8;
        buf[2] = 'r' as u8;
        buf[3] = 0;
        return;
    }

    buf[0] = 'a' as u8 + (buf[0] % ('z' as u8 - 'a' as u8));
    buf[1] = 'a' as u8 + (buf[1] % ('z' as u8 - 'a' as u8));
    buf[2] = 'a' as u8 + (buf[2] % ('z' as u8 - 'a' as u8));
    buf[3] = 'a' as u8 + (buf[3] % ('z' as u8 - 'a' as u8));
    buf[4] = 0;
}
