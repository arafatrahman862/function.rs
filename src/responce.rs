use bin_layout::Encoder;

#[derive(Clone, Copy)]
pub enum Status {
    Ok = 0,
    Err = 1,
    Close = 2,
}

impl Encoder for Status {
    fn encoder(&self, c: &mut impl std::io::Write) -> std::io::Result<()> {
        (*self as u8).encoder(c)
    }
}

#[derive(Encoder)]
pub struct Responce<'a> {
    status: Status,
    data: &'a [u8],
}

#[test]
fn test_name() {}
