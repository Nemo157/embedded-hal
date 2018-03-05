//! Random Number Generator Interface

#[cfg(feature = "unproven")]
use nb;

/// Nonblocking stream of random bytes.
#[cfg(feature = "unproven")]
// reason: No implementations or users yet
pub trait Read {
    /// An enumeration of RNG errors
    ///
    /// May be `!` (`never_type`) for infallible implementations; i.e. software PRNGs
    type Error;

    /// Get a single byte from the RNG
    fn read(&mut self) -> nb::Result<u8, Self::Error>;
}
