#![no_std]
#![feature(asm, link_llvm_intrinsics)]
#![feature(panic_implementation)]
#![allow(dead_code)]

extern crate byteorder;

pub mod error;
mod syscall;
mod panic;
pub mod time;
pub mod seproxyhal;
pub mod ui;
pub mod pic;

pub fn exit(exit_code: u32) -> ! {
    loop {
        syscall::os_sched_exit(exit_code).is_ok();
    };
}

#[doc(hidden)]
pub unsafe fn init_runtime() {
    // Enable interrupts
    asm!("cpsie i" :::: "volatile");
    // Make sure that the try_context pointer isn't random garbage
    asm!("mov r9, $0" :
        : "r"(0)
        : "r9"
        : "volatile");
}

#[macro_export]
macro_rules! entry {
    ($main:path) => {
        #[link_section=".boot"]
        #[no_mangle]
        pub fn bolos_main() -> ! {
            let main: fn() = $main;

            unsafe {
                $crate::init_runtime();
            }

            main();
            exit(0)
        }
    }
}