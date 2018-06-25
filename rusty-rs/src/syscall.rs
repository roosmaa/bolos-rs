use error::SystemError;

pub fn check_api_level(api_level: u32) -> Result<(), SystemError> {
    const SYSCALL_ID: (u32, u32) = (0x60000137, 0x900001c6);
    let params = [
        api_level,
    ];
    supervisor_call(SYSCALL_ID, &params)
        .map(|_| ())
}

pub fn os_sched_exit(exit_code: u32) -> Result<(), SystemError> {
    const SYSCALL_ID: (u32, u32) = (0x60005fe1, 0x90005f6f);
    let params = [
        exit_code,
    ];
    supervisor_call(SYSCALL_ID, &params)
        .map(|_| ())
}

pub fn os_ux(params_bytes: &[u8]) -> Result<u32, SystemError> {
    const SYSCALL_ID: (u32, u32) = (0x60006158, 0x9000611f);
    let params = [
        params_bytes.as_ptr() as u32,
    ];
    supervisor_call(SYSCALL_ID, &params)
}

pub fn cx_rng(buf: &mut [u8]) -> Result<(), SystemError> {
    const SYSCALL_ID: (u32, u32) = (0x6000052c, 0x90000567);
    let params = [
        buf.as_ptr() as u32,
        buf.len() as u32,
    ];
    supervisor_call(SYSCALL_ID, &params)
        .map(|_| ())
}

pub fn io_seproxyhal_spi_is_status_sent() -> Result<bool, SystemError> {
    const SYSCALL_ID: (u32, u32) = (0x60006fcf, 0x90006f7f);
    let params = [];
    supervisor_call(SYSCALL_ID, &params)
        .map(|r| r != 0)
}

pub fn io_seproxyhal_spi_recv(buf: &mut [u8], flags: u32) -> Result<usize, SystemError> {
    const SYSCALL_ID: (u32, u32) = (0x600070d1, 0x9000702b);
    let params = [
        buf.as_ptr() as u32,
        buf.len() as u32,
        flags,
    ];
    supervisor_call(SYSCALL_ID, &params)
        .map(|r| r as usize)
}

pub fn io_seproxyhal_spi_send(buf: &[u8]) -> Result<(), SystemError> {
    const SYSCALL_ID: (u32, u32) = (0x60006e1c, 0x90006ef3);
    let params = [
        buf.as_ptr() as u32,
        buf.len() as u32,
    ];
    supervisor_call(SYSCALL_ID, &params)
        .map(|_| ())
}

#[repr(C)]
struct try_context {
    jmp_buf: [u32; 10],
    exception: u16,
}

extern {
    #[link_name = "llvm.eh.sjlj.setjmp"]
    fn setjmp(jmp_buf: *mut u8) -> i32;
}

#[inline(always)]
fn supervisor_call(syscall_id: (u32, u32), params: &[u32]) -> Result<u32, SystemError> {
    let ret_id: u32;
    let ret_val: u32;
    // Supervisor expects the exception handling context
    // to be present, so we create one just for that
    let mut ctx = try_context{
        jmp_buf: [0; 10],
        exception: 0,
    };
    unsafe {
        let jmp_buf_ptr = ctx.jmp_buf.as_mut_ptr() as *mut u8;
        ctx.exception = setjmp(jmp_buf_ptr) as u16;
        if ctx.exception == 0 {
            // Make the exception handler available
            asm!("mov r9, $0" :
                : "r"(&ctx as *const try_context)
                : "r9"
                : "volatile");
            // Invoke the supervisor
            asm!("svc #1"
                : "={r0}"(ret_id), "={r1}"(ret_val)
                : "{r0}"(syscall_id.0),
                  "{r1}"(params.as_ptr())
                : "r0", "r1", "r9"
                : "volatile");
            // Restore the old value of r9 (no exception)
            asm!("mov r9, $0" :
                : "r"(ctx.jmp_buf[5])
                : "r9"
                : "volatile");
            if ret_id != syscall_id.1 {
                Err(SystemError::Security)
            } else {
                Ok(ret_val)
            }
        } else {
            // Restore the old value of r9 (caught exception)
            asm!("mov r9, $0" :
                : "r"(ctx.jmp_buf[5])
                : "r9"
                : "volatile");
            Err(SystemError::from_u16(ctx.exception)
                .unwrap_or(SystemError::Exception))
        }
    }
}
