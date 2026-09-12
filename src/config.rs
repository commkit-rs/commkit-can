/// Physical-layer configuration for a CAN bus.
///
/// Optional features (hardware specific) exposed through capability traits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanBusConfig {
    /// Nominal (arbitration phase) bit rate, in bit/s
    pub baud_rate: u32,

    /// Whether CAN FD is enabled on this bus
    pub fd_enabled: bool,

    /// Data phase bit rate, in bit/s. Only meaningful when `fd_enabled` is set
    pub fd_baud_rate: Option<u32>,

    /// Listen-only (silent) mode: no ACKs or error frames are transmitted
    pub listen_only: bool,
}
