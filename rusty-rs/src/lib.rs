#![no_std]

extern crate panic_abort;

use core::slice;

#[no_mangle]
pub extern "C" fn update_label(ptr: *mut u8, len: usize) {
    let buf = unsafe { slice::from_raw_parts_mut(ptr, len) };
    buf[0] = 'R' as u8;
    buf[1] = 'u' as u8;
    buf[2] = 's' as u8;
    buf[3] = 't' as u8;
    buf[4] = 0;
}
