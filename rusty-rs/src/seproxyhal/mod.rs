#[macro_use]
mod packet;
pub mod event;
pub mod command;
pub mod status;

use error::SystemError;
use syscall::{os_sched_exit, io_seproxyhal_spi_send};
use self::event::Event;
use self::command::Command;
use self::status::Status;
use self::packet::Packet;

pub fn process<F>(buf: &[u8], handler: F) where
    F: Fn(Channel) -> ()
{
    let ev = match Event::from_bytes(buf) {
        Some(e) => e,
        None => {
            os_sched_exit(1).is_ok();
            return;
        },
    };

    let ch = Channel::new(&ev);
    handler(ch);
}

pub struct Channel<'a> {
    pub event: &'a Event,
    status_sent: bool,
}

impl<'a> Channel<'a> {
    fn new(event: &'a Event) -> Self {
        Self{
            event: event,
            status_sent: false,
        }
    }

    fn send_packet<T: Packet>(&mut self, packet: T) -> Result<(), SystemError> {
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

    pub fn send_command(&mut self, command: Command) -> Result<(), SystemError> {
        self.send_packet(command)
    }

    pub fn send_status(mut self, status: Status) -> Result<(), SystemError> {
        self.status_sent = true;
        self.send_packet(status)
    }
}

impl<'a> Drop for Channel<'a> {
    fn drop(&mut self) {
        // Send a general status when the app-level code failed to
        // send a response
        if !self.status_sent {
            let s = status::GeneralStatus{};
            self.send_packet(s).is_ok();
        }
    }
}