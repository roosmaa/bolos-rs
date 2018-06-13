use core::slice;
use core::str;

extern {
    static _nvram: u32;
    static _envram: u32;
}

#[inline(never)]
fn runtime_offset() -> u32 {
    let offset: u32;
    unsafe {
        asm!("mov r1, pc
              ldr $0, =.
              adds $0, $0, #2
              subs $0, $0, r1"
            : "=r" (offset)
            :: "r1"
            : "volatile");
    }
    offset
}

// TODO: Add platform feature gate such that actual pointer translation
//       would only happen on Ledger device. On other targets it should
//       be a noop operation.
#[inline(always)]
fn translate(mut addr: u32) -> u32 {
    let nvram_start = unsafe { &_nvram as *const u32 as u32 };
    let nvram_end = unsafe { &_envram as *const u32 as u32 };
    if addr >= nvram_start && addr < nvram_end {
        addr -= runtime_offset();
    }
    return addr;
}

pub trait Pic {
    fn pic(self) -> Self;
}

impl<T> Pic for *const T {
    #[inline(always)]
    fn pic(self) -> Self {
        translate(self as u32) as Self
    }
}

impl<T> Pic for *mut T {
    #[inline(always)]
    fn pic(self) -> Self {
        translate(self as u32) as Self
    }
}

impl<'a, T> Pic for &'a T {
    #[inline(always)]
    fn pic(self) -> Self {
        let ptr = (self as *const T).pic();
        unsafe { &*ptr }
    }
}

impl<'a, T> Pic for &'a mut T {
    #[inline(always)]
    fn pic(self) -> Self {
        let ptr = (self as *mut T).pic();
        unsafe { &mut *ptr }
    }
}

impl<'a> Pic for &'a str {
    #[inline(always)]
    fn pic(self) -> Self {
        let len = self.len();
        let ptr = self.as_ptr().pic();
        unsafe {
            let bytes = slice::from_raw_parts(ptr, len);
            str::from_utf8_unchecked(bytes)
        }
    }
}

impl<'a, T> Pic for &'a [T] {
    #[inline(always)]
    fn pic(self) -> Self {
        let len = self.len();
        let ptr = self.as_ptr().pic();
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

impl<'a, T> Pic for &'a mut [T] {
    #[inline(always)]
    fn pic(self) -> Self {
        let len = self.len();
        let ptr = self.as_mut_ptr().pic();
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }
}
