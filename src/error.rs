/// Errors reported by any [`CanDataLink`](crate::CanDataLink) implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanError {
    /// The bus has not been opened, or was closed
    NotOpen,

    /// The controller is bus-off and cannot transmit until it recovers
    BusOff,

    /// No transmit slot is free for this frame right now. Retry later.
    TxBusy,

    /// A received frame was lost because the receive buffer overflowed
    Overrun,

    /// The controller does not support CAN FD, or FD was not enabled when the bus was opened
    FdNotSupported,

    /// The payload does not fit the frame format (8 bytes Classic, 64 bytes FD)
    DataTooLong,

    /// The identifier does not fit its 11-bit or 29-bit format
    InvalidId,

    /// The requested bit rate or sample point cannot be produced by the controller's clock
    UnsupportedBitTiming,

    /// The controller does not support this configuration or capability
    Unsupported,
}
