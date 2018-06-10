extern {
    static _nvram: u32;
    static _envram: u32;
}

#[inline(always)]
fn translate(mut addr: u32) -> u32 {
    let nvram_start = unsafe { &_nvram as *const u32 as u32 };
    let nvram_end = unsafe { &_envram as *const u32 as u32 };
    if addr >= nvram_start && addr < nvram_end {
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
        addr -= offset;
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
