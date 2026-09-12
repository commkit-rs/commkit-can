use commkit::{ByteTransfer, Packet};

use crate::id::CanId;

/// Maximum payload length of a CAN FD frame
pub const MAX_DATA_LEN: usize = 64;

/// A single CAN frame: classic or FD
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanFrame {
    pub id: CanId,
    pub fd: bool,
    pub brs: bool,
    pub rtr: bool,
    data: [u8; MAX_DATA_LEN],
    len: u8,
}

impl CanFrame {
    /// Builds a frame from a data slice. Slices longer than [`MAX_DATA_LEN`]
    /// are truncated.
    pub fn new(id: CanId, fd: bool, brs: bool, rtr: bool, data: &[u8]) -> Self {
        let len = core::cmp::min(data.len(), MAX_DATA_LEN);
        let mut buf = [0u8; MAX_DATA_LEN];
        buf[..len].copy_from_slice(&data[..len]);

        Self {
            id,
            fd,
            brs,
            rtr,
            data: buf,
            len: len as u8,
        }
    }

    /// The frame's data payload.
    pub fn data(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
}

impl ByteTransfer for CanFrame {
    fn as_bytes(&self) -> &[u8] {
        self.data()
    }
}

impl Packet for CanFrame {}
