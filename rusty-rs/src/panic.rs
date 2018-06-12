use core::panic::PanicInfo;
use syscall::os_sched_exit;

#[panic_implementation]
#[allow(private_no_mangle_fns)]
#[no_mangle]
extern fn panic(_info: &PanicInfo) -> ! {
    loop {
        os_sched_exit(1).is_ok();
    };
}
