use core::convert::TryFrom;

use crate::*;
use bin_layout::*;
use CloseCode::*;

pub struct Rsv(pub u8);

impl Rsv {
    /// The first bit of the RSV field.
    pub fn rsv1(&self) -> bool {
        self.0 & 1 == 1
    }

    /// The second bit of the RSV field.
    pub fn rsv2(&self) -> bool {
        self.0 & 2 == 2
    }

    /// The third bit of the RSV field.
    pub fn rsv3(&self) -> bool {
        self.0 & 4 == 4
    }
}

impl Encoder for Header {
    const SIZE: usize = 2;
    fn encoder(self, _c: &mut Cursor<impl Bytes>) {
        todo!()
    }
}

impl<E: Error> Decoder<'_, E> for Header {
    fn decoder(c: &mut Cursor<&[u8]>) -> Result<Self, E> {
        use Opcode::*;
        let [fst, snd] = <[u8; 2]>::decoder(c)?;

        let opcode = match fst & 0b_1111 {
            0 => Continue,
            1 => Text,
            2 => Binary,
            8 => Close,
            9 => Ping,
            10 => Pong,
            // If an unknown opcode is received, the receiving endpoint MUST _Fail the WebSocket Connection_.
            _ => return Err(E::invalid_data()),
        };
        Ok(Self {
            fin: fst & 0b_1000_0000 != 0,
            rsv: Rsv((fst >> 4) & 0b_111),
            opcode,
            payload_len: match snd & 0b_111_1111 {
                126 => {
                    // All control frames MUST have a payload length of 125 bytes or less
                    if opcode.is_control() {
                        return Err(E::invalid_data());
                    }
                    u16::decoder(c)? as usize
                }
                127 => {
                    if opcode.is_control() {
                        return Err(E::invalid_data());
                    }
                    u64::decoder(c)? as usize
                }
                len => len as usize,
            },
            mask: if snd & 0b_1000_0000 != 0 {
                Some(<[u8; 4]>::decoder(c)?)
            } else {
                None
            },
        })
    }
}

impl TryFrom<u16> for CloseCode {
    type Error = CloseCode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1000 => Normal,
            1001 => Away,
            1002 => ProtocolError,
            1003 => Unsupported,
            1005 => NoStatusRcvd,
            1006 => Abnormal,
            1007 => InvalidPayload,
            1008 => PolicyViolation,
            1009 => MessageTooBig,
            1010 => MandatoryExt,
            1011 => InternalError,
            1015 => TLSHandshake,
            _ => return Err(PolicyViolation),
        })
    }
}

impl bin_layout::Error for CloseCode {
    fn insufficient_bytes() -> Self {
        MessageTooBig
    }
    fn invalid_data() -> Self {
        PolicyViolation
    }
    fn utf8_err(_: std::str::Utf8Error) -> Self {
        InvalidPayload
    }
}
