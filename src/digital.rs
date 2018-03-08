//! Digital I/O

#[cfg(feature = "unproven")]
use nb;

/// Single digital output pin
pub trait OutputPin {
    /// Is the output pin high?
    fn is_high(&self) -> bool;

    /// Is the output pin low?
    fn is_low(&self) -> bool;

    /// Sets the pin low
    fn set_low(&mut self);

    /// Sets the pin high
    fn set_high(&mut self);
}

#[cfg(feature = "unproven")]
/// TODO
pub enum Event {
    /// TODO
    High,
    /// TODO
    Low,
    /// TODO
    HighToLow,
    /// TODO
    LowToHigh,
}

#[cfg(feature = "unproven")]
/// TODO
pub trait Detector {
    /// Check if event has happened since last call to `poll`. Returns
    /// `Ok(())` if it has or `Err(nb::Error::WouldBlock)` if not.
    fn poll(&self) -> nb::Result<(), !>;
}

/// Single digital input pin
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Is the input pin high?
    fn is_high(&self) -> bool;

    /// Is the input pin low?
    fn is_low(&self) -> bool;
}

#[cfg(feature = "unproven")]
/// TODO
pub trait DetectingInputPin: InputPin {
    /// TODO
    type Detector: Detector;

    /// TODO
    fn detect(self, event: Event) -> Self::Detector;
}
