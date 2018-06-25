use core::panic::PanicInfo;
use super::exit;

#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: Implement a sub-eventloop that displays the panic message to the
    //       developer (only in debug mode)
    exit(1)
}
