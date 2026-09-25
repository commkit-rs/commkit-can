use commkit::DataLink;

use crate::config::CanBusConfig;
use crate::error::CanError;
use crate::frame::CanFrame;

/// Fault confinement state of the controller, as defined by ISO 11898-1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanBusState {
    /// The bus is not open
    Closed,

    /// Normal operation, both error counters below 128
    ErrorActive,

    /// An error counter reached 128. The controller still communicates but signals errors passively
    ErrorPassive,

    /// The transmit error counter passed 255. The controller has disconnected from the bus
    BusOff,
}

/// Transmit and receive error counters (TEC / REC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanErrorCounters {
    pub tx: u8,
    pub rx: u8,
}

/// A CAN interface: sends and receives [`CanFrame`]s and can be opened, closed and monitored
pub trait CanDataLink: DataLink<Packet = CanFrame, PhysicalConfig = CanBusConfig, Error = CanError> {
    /// Applies `config` and joins the bus
    fn open(&mut self, config: &CanBusConfig) -> Result<(), CanError>;

    /// Leaves the bus. Frames can no longer be sent or received until reopened.
    fn close(&mut self) -> Result<(), CanError>;

    fn state(&self) -> CanBusState;

    fn error_counters(&self) -> CanErrorCounters;

    fn is_open(&self) -> bool {
        self.state() != CanBusState::Closed
    }
}
