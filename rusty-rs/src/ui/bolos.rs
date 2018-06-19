use syscall;

#[repr(u8)]
enum UxType {
    Event = 1,
    WakeUp = 2,
}

#[inline(always)]
fn invoke_ux(params: &[u8]) -> Response {
    syscall::os_ux(params)
        .and_then(|r| {
            Ok(Response::from_u8(r)
                .unwrap_or(Response::Error))
        })
        .unwrap_or(Response::Error)
}


pub fn event() -> Response {
    let params = [
        UxType::Event as u8,
        0,
    ];
    // TODO: Experiment with ROPI/PIC by triggering a crash that happens
    //       when the params array is inlined below
    invoke_ux(&params)
}

pub fn wake_up() -> Response {
    let params = [
        UxType::Event as u8,
        0,
    ];
    invoke_ux(&params)
}

#[repr(u32)]
pub enum Response {
    Continue = 0,
    Redraw = 0xB0105055,
    Ignore = 0xB0105044,
    Error = 0xB0105033,
    Cancel = 0xB0105022,
    Ok = 0xB0105011,
}

impl Response {
    fn from_u8(value: u32) -> Option<Self> {
        if value == Response::Continue as u32 {
            Some(Response::Continue)
        } else if value == Response::Redraw as u32 {
            Some(Response::Redraw)
        } else if value == Response::Ignore as u32 {
            Some(Response::Ignore)
        } else if value == Response::Error as u32 {
            Some(Response::Error)
        } else if value == Response::Cancel as u32 {
            Some(Response::Cancel)
        } else if value == Response::Ok as u32 {
            Some(Response::Ok)
        } else {
            None
        }
    }
}