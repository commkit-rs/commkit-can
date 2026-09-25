use crate::datalink::CanDataLink;
use crate::error::CanError;

/// Optional support for one-shot (no automatic retransmission) mode.
pub trait OneShotMode: CanDataLink {
    fn set_one_shot(&mut self, enable: bool) -> Result<(), CanError>;
    fn one_shot(&self) -> bool;
}

/// Optional support for automatic bus-off recovery.
pub trait BusOffRecovery: CanDataLink {
    fn set_restart_on_bus_off(&mut self, enable: bool) -> Result<(), CanError>;
    fn restart_on_bus_off(&self) -> bool;
}

/// Optional support for tuning the bit timing sample point.
pub trait SamplePointControl: CanDataLink {
    /// `percent` is the sample point as a percentage of bit time (0-100).
    fn set_sample_point(&mut self, percent: u8) -> Result<(), CanError>;
    fn sample_point(&self) -> u8;
}

/// Optional support for internal loopback (self-test) mode.
pub trait LoopbackMode: CanDataLink {
    fn set_loopback(&mut self, enable: bool) -> Result<(), CanError>;
    fn loopback(&self) -> bool;
}
