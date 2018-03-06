use core::cmp;

use nb;

/// Non-blocking reader trait
pub trait Read {
    /// An enumeration of possible errors
    ///
    /// May be `!` (`never_type`) for infallible implementations
    type Error;

    /// Pull some bytes from this source into the specified buffer, returning how many bytes were
    /// read.
    ///
    /// If an object needs to block for a read it will return an `Err(nb::Error::WouldBlock)`
    /// return value.
    ///
    /// If the return value of this method is `Ok(n)`, then it must be guaranteed that `0 <= n <=
    /// buf.len()`. The `n` value indicates that the buffer buf has been filled in with `n` bytes
    /// of data from this source.
    fn read(&mut self, buf: &mut [u8]) -> nb::Result<usize, Self::Error>;
}

impl<'a, R: Read> Read for &'a mut R {
    type Error = R::Error;

    fn read(&mut self, buf: &mut [u8]) -> nb::Result<usize, Self::Error> {
        (*self).read(buf)
    }
}

#[derive(Debug)]
pub struct EndOfBuf;

impl<'a> Read for &'a [u8] {
    type Error = EndOfBuf;

    fn read(&mut self, buf: &mut [u8]) -> nb::Result<usize, Self::Error> {
        if self.len() == 0 {
            return Err(nb::Error::Other(EndOfBuf));
        }

        let len = cmp::min(self.len(), buf.len());
        buf[..len].copy_from_slice(&self[..len]);
        *self = &self[len..];
        Ok(len)
    }
}
