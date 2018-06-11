pub trait Packet {
    fn bytes_size(&self) -> u16;
    fn to_bytes(&self, buf: &mut [u8], offset: usize) -> usize;
}

#[macro_export]
macro_rules! impl_packet {
    (__to_bytes,
        $offset:ident, $buf:ident, $written:ident, $start:expr,
    ) => {{}};
    (__to_bytes,
        $offset:ident, $buf:ident, $written:ident, $start:expr,
        $len:expr => $data:expr, $($rest:tt)*
    ) => {
        #[allow(unused_comparisons)]
        {
            let start: usize = $start;
            let end: usize = start + $len;
            let i = $offset + $written;

            $written += if i >= start && i < end {
                let data = $data;

                let j = i - start;
                let cnt = ::core::cmp::min($buf.len() - $written, data.len() - j);
                $buf[i..i+cnt].copy_from_slice(&data[j..j+cnt]);
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
    (__bytes_size, $len:expr => $data:expr, $($rest:tt)*) => {{
        $len as u16 + impl_packet!(__bytes_size, $($rest)*)
    }};
    ($self:ident, $packet_tag:expr, {
        $($writes:tt)*
    }) => {
        fn bytes_size(&$self) -> u16 {
            3 + impl_packet!(__bytes_size, $($writes)*)
        }

        fn to_bytes(&$self, buf: &mut [u8], offset: usize) -> usize {
            use ::byteorder::{ByteOrder, BigEndian};

            let mut written = 0;

            impl_packet!(__to_bytes, offset, buf, written, 0,
                3 => {
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