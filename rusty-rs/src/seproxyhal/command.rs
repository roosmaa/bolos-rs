use super::packet::Packet;

enum CommandTag {
}

pub enum Command {
}

impl Packet for Command {
    fn bytes_size(&self) -> u16 {
        0
    }

    fn to_bytes(&self, _buf: &mut [u8], _offset: usize) -> usize {
        0
    }
}
