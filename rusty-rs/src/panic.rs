use core::panic::PanicInfo;
use syscall::os_sched_exit;

#[panic_implementation]
#[allow(private_no_mangle_fns)]
#[no_mangle]
extern fn panic(_info: &PanicInfo) -> ! {
    // TODO: Test if panic is raised for PIC related memory exceptions

    // TODO: Test if sending a GeneralStatus here (if no status has been sent)
    //       helps with exiting the app smoothily.

    // TODO: Implement a sub-eventloop that displays the panic message to the
    //       developer (only in debug mode)
    loop {
        os_sched_exit(1).is_ok();
    };
}
