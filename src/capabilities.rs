use commkit::DataLink;

/// Optional support for one-shot (no automatic retransmission) mode.
pub trait OneShotMode: DataLink {
    fn set_one_shot(&mut self, enable: bool) -> Result<(), Self::Error>;
    fn one_shot(&self) -> bool;
}

/// Optional support for automatic bus-off recovery.
pub trait BusOffRecovery: DataLink {
    fn set_restart_on_bus_off(&mut self, enable: bool) -> Result<(), Self::Error>;
    fn restart_on_bus_off(&self) -> bool;
}

/// Optional support for tuning the bit timing sample point.
pub trait SamplePointControl: DataLink {
    /// `percent` is the sample point as a percentage of bit time (0-100).
    fn set_sample_point(&mut self, percent: u8) -> Result<(), Self::Error>;
    fn sample_point(&self) -> u8;
}

/// Optional support for internal loopback (self-test) mode.
pub trait LoopbackMode: DataLink {
    fn set_loopback(&mut self, enable: bool) -> Result<(), Self::Error>;
    fn loopback(&self) -> bool;
}
