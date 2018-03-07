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

#[cfg(feature = "unproven")]
/// TODO
pub mod io {
    use nb;
    use super::Read;

    /// TODO
    pub struct Reader<R> where R: Read {
        reader: R,
    }

    /// TODO
    pub fn reader<R>(reader: R) -> Reader<R> where R: Read {
        Reader { reader }
    }

    impl<R> nb::io::Read for Reader<R> where R: Read {
        type Error = R::Error;

        fn read(&mut self, buf: &mut [u8]) -> nb::Result<usize, Self::Error> {
            let mut count = 0;
            while count < buf.len() {
                match self.reader.read() {
                    Ok(byte) => {
                        buf[count] = byte;
                        count += 1;
                    }
                    Err(nb::Error::WouldBlock) => {
                        if count > 0 {
                            return Ok(count);
                        } else {
                            return Err(nb::Error::WouldBlock);
                        }
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            return Ok(count);
        }
    }
}
