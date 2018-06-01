use error::SystemError;

pub fn cx_rng(buf: &mut [u8]) -> Result<(), SystemError> {
    const SYSCALL_ID_IN: u32 = 0x6000052c;
    const SYSCALL_ID_OUT: u32 = 0x90000567;
    let params = [
        buf.as_ptr() as u32,
        buf.len() as u32,
    ];
    let (ret_id, _) = svc_call(SYSCALL_ID_IN, params.as_ptr());
    if ret_id == SYSCALL_ID_OUT {
        Ok(())
    } else {
        Err(SystemError::Security)
    }
}

fn svc_call(syscall_id: u32, params: *const u32) -> (u32, u32) {
    let ret_id: u32;
    let ret_val: u32;
    unsafe {
        asm!("svc #1"
            : "={r0}"(ret_id), "={r1}"(ret_val)
            : "{r0}"(syscall_id), "{r1}"(params)
            : "r0","r1"
            : "volatile");
    }
    (ret_id, ret_val)
}
