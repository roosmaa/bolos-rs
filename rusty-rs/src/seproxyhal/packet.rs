use pic::Pic;
use error::SystemError;
use syscall::io_seproxyhal_spi_send;

pub trait Packet {
    fn bytes_size(&self) -> u16;
    fn to_bytes(&self, buf: &mut [u8], offset: usize) -> usize;
}

pub fn send<T: Packet>(packet: T) -> Result<(), SystemError> {
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

#[macro_export]
macro_rules! impl_packet {
    (__to_bytes,
        $offset:ident, $buf:ident, $written:ident, $start:expr,
    ) => {{}};
    (__to_bytes,
        $offset:ident, $buf:ident, $written:ident, $start:expr,
        [S] $len:expr => $data:expr, $($rest:tt)*
    ) => {
        #[allow(unused_comparisons)]
        {
            let start: usize = $start;
            let end: usize = start + $len;
            let i = $offset + $written;

            $written += if i >= start && i < end {
                let slice = $data;

                let j = $written;
                let k = i - start;
                let cnt = ::core::cmp::min($buf.len() - j, $len - k);
                $buf[j..j+cnt].copy_from_slice(&slice[k..k+cnt]);
                cnt
            } else {
                0
            };

            impl_packet!(__to_bytes,
                $offset, $buf, $written, end,
                $($rest)*
            );
        }
    };
    (__to_bytes,
        $offset:ident, $buf:ident, $written:ident, $start:expr,
        [I] $len:expr => $data:expr, $($rest:tt)*
    ) => {
        #[allow(unused_comparisons)]
        {
            let start: usize = $start;
            let end: usize = start + $len;
            let i = $offset + $written;

            $written += if i >= start && i < end {
                let it = $data;

                let j = $written;
                let k = i - start;
                let cnt = ::core::cmp::min($buf.len() - j, $len - k);

                let dst_it = $buf.iter_mut().skip(j).take(cnt);
                let src_it = it.skip(k).take(cnt);
                for (dst, src) in dst_it.zip(src_it) {
                    *dst = src;
                }
                cnt
            } else {
                0
            };

            impl_packet!(__to_bytes,
                $offset, $buf, $written, end,
                $($rest)*
            );
        }
    };
    (__bytes_size, ) => {{ 0 }};
    (__bytes_size, [S] $len:expr => $data:expr, $($rest:tt)*) => {{
        $len as u16 + impl_packet!(__bytes_size, $($rest)*)
    }};
    (__bytes_size, [I] $len:expr => $data:expr, $($rest:tt)*) => {{
        $len as u16 + impl_packet!(__bytes_size, $($rest)*)
    }};
    ($self:ident, $packet_tag:expr, {
        $($writes:tt)*
    }) => {
        fn bytes_size(&self) -> u16 {
            // Sometimes when the struct is static, Rust stores it in
            // the memory region that is affected by the Ledger memory
            // model. So we need to fixup the self reference before
            // usage, to be sure we don't accidentally crash.
            #[allow(unused_variables)]
            let $self = {
                use pic::Pic;
                self.pic()
            };

            3 + impl_packet!(__bytes_size, $($writes)*)
        }

        fn to_bytes(&self, buf: &mut [u8], offset: usize) -> usize {
            // See above in bytes_size for the reasoning behind PIC
            #[allow(unused_variables)]
            let $self = {
                use pic::Pic;
                self.pic()
            };

            let mut written = 0;

            impl_packet!(__to_bytes, offset, buf, written, 0,
                [S] 3 => {
                    use ::byteorder::{ByteOrder, BigEndian};

                    let mut hdr: [u8; 3] = [
                        $packet_tag as u8,
                        0, 0,
                    ];
                    let data_size = $self.bytes_size() - hdr.len() as u16;
                    BigEndian::write_u16(&mut hdr[1..3], data_size);
                    hdr
                },
            $($writes)*);

            written
        }
    };
}

pub struct FourByteIterator {
    data: [u8; 4],
    n: usize,
}

impl FourByteIterator {
    pub fn new(data: [u8; 4]) -> Self {
        Self{
            data: data,
            n: 0,
        }
    }
}

impl Iterator for FourByteIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let this = self.pic();
        let n = this.n;
        if n < this.data.len() {
            this.n += 1;
            Some(this.data[n])
        } else {
            None
        }
    }
}
