//! Core IO traits

#![cfg(feature = "unproven")]
// reason: No implementations or users yet

mod read;
mod read_exact;

pub use self::read::Read;
pub use self::read_exact::ReadExact;
pub use self::read_exact::read_exact;
