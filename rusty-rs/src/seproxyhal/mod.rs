#[macro_use]
mod packet;
pub mod event;
pub mod command;
pub mod status;

use pic::Pic;
use syscall::{check_api_level, io_seproxyhal_spi_recv, io_seproxyhal_spi_is_status_sent};
use self::event::Event;
use self::command::Command;
use self::status::Status;

const CX_COMPAT_APILEVEL: u32 = 8;

pub struct MessageLoop {
    running: bool,
}

impl MessageLoop {
    pub fn new() -> Self {
        check_api_level(CX_COMPAT_APILEVEL)
            .expect("API level check failed");

        Self{
            running: false,
        }
    }
}

impl Iterator for MessageLoop {
    type Item = Channel;

    fn next(&mut self) -> Option<Self::Item> {
        let this = self.pic();
        let first_loop = !this.running;

        if first_loop {
            this.running = true;
        }

        let is_status_sent = if first_loop {
            io_seproxyhal_spi_is_status_sent().unwrap()
        } else {
            true
        };

        let ev = if first_loop && !is_status_sent {
            Event::StartLoop
        } else {
            let mut buf = [0; 64];
            let read = io_seproxyhal_spi_recv(&mut buf, 0)
                .expect("Unable to read event data");

            Event::from_bytes(&buf[0..read])
                .expect("Unsupported event")
        };
        Some(Channel::new(ev))
    }
}

pub struct Channel {
    pub event: Event,
    status_sent: bool,
}

impl Channel {
    fn new(event: Event) -> Self {
        Self{
            event,
            status_sent: false,
        }
    }

    pub fn send_command(&mut self, command: Command) {
        packet::send(command).expect("Failed to send command")
    }

    pub fn send_status(mut self, status: Status) {
        self.status_sent = true;
        packet::send(status).expect("Failed to send status")
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        let this = self.pic();
        // Send a general status when the app-level code failed to
        // send a response
        if !this.status_sent {
            packet::send(status::GeneralStatus{}).is_ok();
        }
    }
}