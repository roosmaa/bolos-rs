use core::panic::PanicInfo;
use super::syscall;

#[doc(hidden)]
pub unsafe fn init() {
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
                $crate::runtime::init();
            }

            main();
            exit(0)
        }
    }
}

pub fn exit(exit_code: u32) -> ! {
    loop {
        syscall::os_sched_exit(exit_code).is_ok();
    };
}

#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: Implement a sub-eventloop that displays the panic message to the
    //       developer (only in debug mode)
    exit(1)
}
