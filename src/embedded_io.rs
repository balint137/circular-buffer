// Copyright © 2023, 2024 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

#![cfg(feature = "embedded-io")]

use crate::CircularBuffer;
use embedded_io::{BufRead, ErrorType, Read, Write};

impl<const N: usize> ErrorType for CircularBuffer<N, u8> {
    type Error = core::convert::Infallible;
}

impl<const N: usize> Write for CircularBuffer<N, u8> {
    #[inline]
    fn write(&mut self, src: &[u8]) -> Result<usize, Self::Error> {
        self.extend_from_slice(src);
        Ok(src.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<const N: usize> Read for CircularBuffer<N, u8> {
    fn read(&mut self, dst: &mut [u8]) -> Result<usize, Self::Error> {
        let (mut front, mut back) = self.as_slices();
        let mut count = front.read(dst)?;
        count += back.read(&mut dst[count..])?;
        self.truncate_front(self.len() - count);
        Ok(count)
    }
}

impl<const N: usize> BufRead for CircularBuffer<N, u8> {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        let (front, back) = self.as_slices();
        if !front.is_empty() {
            Ok(front)
        } else {
            Ok(back)
        }
    }

    fn consume(&mut self, amt: usize) {
        let amt = if amt <= self.len() { amt } else { self.len() };
        self.drain(..amt);
    }
}
