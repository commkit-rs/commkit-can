/// A CAN identifier, in either the 11-bit standard format or the 29-bit extended format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanId {
    Standard(u16),
    Extended(u32),
}

impl CanId {
    /// Whether this is an extended (29-bit) identifier.
    pub fn is_extended(&self) -> bool {
        matches!(self, CanId::Extended(_))
    }

    /// The raw numeric value of the identifier, regardless of format.
    pub fn raw(&self) -> u32 {
        match self {
            CanId::Standard(id) => *id as u32,
            CanId::Extended(id) => *id,
        }
    }
}
