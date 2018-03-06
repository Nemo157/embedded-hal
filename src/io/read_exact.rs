use core::mem;

use nb;

use io::Read;

/// Creates a helper that will read from a provided reader until the provided buffer is filled.
///
/// Once the buffer has been filled the helper will return the reader + buffer,
///
/// In the case of an error from the reader the helper will discard both reader and buffer and
/// return the error.
///
/// ```
/// # #[macro_use]
/// # extern crate nb;
/// # extern crate embedded_hal;
/// # fn main () {
/// use embedded_hal::io::read_exact;
///
/// let source = [5, 6, 7];
/// let reader = &mut &source[..];
///
/// let mut buffer = [0, 0];
/// let mut reading = read_exact(reader, buffer);
/// let (_, buffer) = block!(reading.poll()).unwrap();
/// assert_eq!(source[0..2], buffer);
/// # }
/// ```
pub fn read_exact<R: Read, B: AsMut<[u8]>>(reader: R, buffer: B) -> ReadExact<R, B> {
    ReadExact {
        state: State::Reading {
            reader,
            buffer,
            position: 0
        },
    }
}

/// TODO
pub struct ReadExact<R: Read, B: AsMut<[u8]>> {
    state: State<R, B>,
}

enum State<R: Read, B: AsMut<[u8]>> {
    Reading {
        reader: R,
        buffer: B,
        position: usize,
    },
    Empty,
}

impl<R: Read, B: AsMut<[u8]>> ReadExact<R, B> {
    /// Poll the associated reader for new bytes.
    ///
    /// If buffer has been filled then will return the reader + buffer,
    ///
    /// In the case of an error from the reader will discard both reader and buffer and return the
    /// error.
    ///
    /// Otherwise will return `nb::Error::WouldBlock`.
    pub fn poll(&mut self) -> nb::Result<(R, B), R::Error> {
        if let State::Reading { ref mut reader, ref mut buffer, ref mut position } = self.state {
            let buffer = buffer.as_mut();
            while *position < buffer.len() {
                *position += reader.read(&mut buffer[*position..])?;
            }
        } else {
            panic!("wait a ReadExact after it's done");
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Reading { reader, buffer, .. } => Ok((reader, buffer).into()),
            State::Empty => panic!(),
        }
    }
}
