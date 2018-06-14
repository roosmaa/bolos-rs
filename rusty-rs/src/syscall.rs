use error::SystemError;

pub fn check_api_level(api_level: u32) -> Result<(), SystemError> {
    const SYSCALL_ID_IN: u32 = 0x60000137;
    const SYSCALL_ID_OUT: u32 = 0x900001c6;
    let params = [
        api_level,
    ];
    let (ret_id, _) = supervisor_call(SYSCALL_ID_IN, &params);
    if ret_id != SYSCALL_ID_OUT {
        Err(SystemError::Security)
    } else {
        Ok(())
    }
}

pub fn os_sched_exit(exit_code: u32) -> Result<(), SystemError> {
    const SYSCALL_ID_IN: u32 = 0x60005fe1;
    const SYSCALL_ID_OUT: u32 = 0x90005f6f;
    let params = [
        exit_code,
    ];
    let (ret_id, _) = supervisor_call(SYSCALL_ID_IN, &params);
    if ret_id != SYSCALL_ID_OUT {
        Err(SystemError::Security)
    } else {
        Ok(())
    }
}

pub fn cx_rng(buf: &mut [u8]) -> Result<(), SystemError> {
    const SYSCALL_ID_IN: u32 = 0x6000052c;
    const SYSCALL_ID_OUT: u32 = 0x90000567;
    let params = [
        buf.as_ptr() as u32,
        buf.len() as u32,
    ];
    let (ret_id, _) = supervisor_call(SYSCALL_ID_IN, &params);
    if ret_id != SYSCALL_ID_OUT {
        Err(SystemError::Security)
    } else {
        Ok(())
    }
}

pub fn io_seproxyhal_spi_is_status_sent() -> Result<bool, SystemError> {
    const SYSCALL_ID_IN: u32 = 0x60006fcf;
    const SYSCALL_ID_OUT: u32 = 0x90006f7f;
    let params = [];
    let (ret_id, ret) = supervisor_call(SYSCALL_ID_IN, &params);
    if ret_id != SYSCALL_ID_OUT {
        Err(SystemError::Security)
    } else {
        Ok(ret != 0)
    }
}

pub fn io_seproxyhal_spi_recv(buf: &mut [u8], flags: u32) -> Result<usize, SystemError> {
    const SYSCALL_ID_IN: u32 = 0x600070d1;
    const SYSCALL_ID_OUT: u32 = 0x9000702b;
    let params = [
        buf.as_ptr() as u32,
        buf.len() as u32,
        flags,
    ];
    let (ret_id, ret) = supervisor_call(SYSCALL_ID_IN, &params);
    if ret_id != SYSCALL_ID_OUT {
        Err(SystemError::Security)
    } else {
        Ok(ret as usize)
    }
}

pub fn io_seproxyhal_spi_send(buf: &[u8]) -> Result<(), SystemError> {
    const SYSCALL_ID_IN: u32 = 0x60006e1c;
    const SYSCALL_ID_OUT: u32 = 0x90006ef3;
    let params = [
        buf.as_ptr() as u32,
        buf.len() as u32,
    ];
    let (ret_id, _) = supervisor_call(SYSCALL_ID_IN, &params);
    if ret_id != SYSCALL_ID_OUT {
        Err(SystemError::Security)
    } else {
        Ok(())
    }
}

#[inline(always)]
fn supervisor_call(syscall_id: u32, params: &[u32]) -> (u32, u32) {
    let ret_id: u32;
    let ret_val: u32;
    unsafe {
        asm!("svc #1"
            : "={r0}"(ret_id), "={r1}"(ret_val)
            : "{r0}"(syscall_id), "{r1}"(params.as_ptr())
            : "r0","r1"
            : "volatile");
    }
    (ret_id, ret_val)
}
