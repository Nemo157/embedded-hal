//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

#[cfg(feature = "unproven")]
pub use ::Capture as _;
#[cfg(feature = "unproven")]
pub use ::Pwm as _;
pub use ::PwmPin as _;
#[cfg(feature = "unproven")]
pub use ::Qei as _;
pub use ::timer::CountDown as _;
pub use ::blocking::delay::DelayMs as _;
pub use ::blocking::delay::DelayUs as _;
pub use ::blocking::i2c::{
    Read as _,
    Write as _,
    WriteRead as _,
};
pub use ::blocking::serial::Write as _;
pub use ::blocking::spi::{
    Transfer as _,
    Write as _,
};
pub use ::digital::OutputPin as _;
#[cfg(feature = "unproven")]
pub use ::digital::InputPin as _;
pub use ::serial::Read as _;
pub use ::serial::Write as _;
pub use ::spi::FullDuplex as _;
