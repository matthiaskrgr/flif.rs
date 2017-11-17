use std::io::prelude::*;
use std::io;
use error::*;
use num_traits::{PrimInt, Unsigned};

pub mod rac;
pub mod varint;
pub mod symbol;
pub mod near_zero;

pub trait FlifReadExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_varint<T: PrimInt + Unsigned>(&mut self) -> Result<T>;
}

impl<R: Read> FlifReadExt for R {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut byte_buf = [0; 1];
        self.read_exact(&mut byte_buf)?;
        Ok(byte_buf[0])
    }

    fn read_varint<T: PrimInt + Unsigned>(&mut self) -> Result<T> {
        varint::read_varint(self)
    }
}
