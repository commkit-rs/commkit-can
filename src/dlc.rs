/// A valid CAN / CAN-FD data length: 0-8 bytes, or one of CAN FD's larger
/// codes (12, 16, 20, 24, 32, 48, 64).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dlc(u8);

impl Dlc {
    pub const fn new(len: u8) -> Option<Self> {
        match len {
            0..=8 | 12 | 16 | 20 | 24 | 32 | 48 | 64 => Some(Self(len)),
            _ => None,
        }
    }

    pub const fn len(self) -> usize {
        self.0 as usize
    }
}
