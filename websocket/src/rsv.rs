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
