use crate::*;
use bin_layout::*;
use Opcode::*;

impl Encoder for Frame {
    const SIZE: usize = 2;
    fn encoder(self, _c: &mut Cursor<impl Bytes>) {
        todo!()
    }
}

impl<E: Error> Decoder<'_, E> for Frame {
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, E> {
        let o2 = u16::decoder(c)?; // 2 Octets
        Ok(Self {
            FIN: o2 >> 15 == 1,
            RSV: Rsv((o2 >> 12) as u8 & 0b_111),
            opcode: match o2 >> 8 & 0b_1111 {
                0 => Continuation,
                1 => Text,
                2 => Binary,
                8 => Close,
                9 => Ping,
                10 => Pong,
                // If an unknown opcode is received, the receiving endpoint MUST _Fail the WebSocket Connection_.
                _ => return Err(E::invalid_data()),
            },
            _MASK: (),
            payload_len: match o2 & 0b_111_1111 {
                126 => u16::decoder(c)? as usize,
                127 => u64::decoder(c)? as usize,
                len => len as usize,
            },
            masking_key: if o2 & 0x80 == 0x80 {
                Some(<[u8; 4]>::decoder(c)?)
            } else {
                None
            },
        })
    }
}