#[macro_use]
mod packet;
pub mod event;
pub mod command;
pub mod status;

use error::SystemError;
use syscall::{check_api_level, io_seproxyhal_spi_recv, io_seproxyhal_spi_send, io_seproxyhal_spi_is_status_sent};
use self::event::Event;
use self::command::Command;
use self::status::Status;
use self::packet::Packet;

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
        let first_loop = !self.running;

        if first_loop {
            self.running = true;
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

fn send_packet<T: Packet>(packet: T) -> Result<(), SystemError> {
    let total = packet.bytes_size() as usize;
    let mut offset = 0;
    let mut buf = [0; 64];

    while offset < total {
        let n = packet.to_bytes(&mut buf, offset);
        offset += n;

        if let Err(err) = io_seproxyhal_spi_send(&buf[0..n]) {
            return Err(err);
        }
    }

    Ok(())
}

pub struct Channel {
    pub event: Event,
    status_sent: bool,
}

impl Channel {
    fn new(event: Event) -> Self {
        Self{
            event: event,
            status_sent: false,
        }
    }

    pub fn send_command(&mut self, command: Command) {
        send_packet(command).expect("Failed to send command")
    }

    pub fn send_status(mut self, status: Status) {
        self.status_sent = true;
        send_packet(status).expect("Failed to send status")
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        // Send a general status when the app-level code failed to
        // send a response
        if !self.status_sent {
            send_packet(status::GeneralStatus{}).is_ok();
        }
    }
}