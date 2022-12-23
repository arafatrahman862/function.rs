use bin_layout::Decoder;

type DynErr = Box<dyn std::error::Error + Send + Sync>;
pub struct Ctx<State = ()> {
    pub state: State,
}

pub trait Parse<State, Args>: Sized {
    type Error;
    fn parse(ctx: Ctx<State>, data: &[u8]) -> Result<Self, Self::Error>;
}

impl<State, Args: for<'de> Decoder<'de>> Parse<State, Args> for Args {
    type Error = DynErr;
    fn parse(_: Ctx<State>, data: &[u8]) -> Result<Self, Self::Error> {
        Args::decode(data)
    }
}
