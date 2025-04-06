pub trait AsInt {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
    fn as_u128(self) -> u128;
}

impl<T> AsInt for T
where
    T: Into<u8>,
{
    fn as_u8(self) -> u8 {
        self.into()
    }

    fn as_u16(self) -> u16 {
        self.into() as u16
    }

    fn as_u32(self) -> u32 {
        self.into() as u32
    }

    fn as_u64(self) -> u64 {
        self.into() as u64
    }

    fn as_u128(self) -> u128 {
        self.into() as u128
    }
}
