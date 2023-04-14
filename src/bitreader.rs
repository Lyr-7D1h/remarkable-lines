use std::io::Read;

use crate::ParseError;

/// A little endian binary reader
pub struct Bitreader<N: Read> {
    bits: N,
    offset: usize,
}

impl<N: Read> Bitreader<N> {
    pub fn new(bits: N) -> Bitreader<N> {
        Bitreader { bits, offset: 0 }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    // Read bytes and update the offset
    pub fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), ParseError> {
        self.offset += buffer.len();
        self.bits.read_exact(buffer)?;
        return Ok(());
    }

    pub fn read_bytes(&mut self, amount: usize) -> Result<Vec<u8>, ParseError> {
        let mut buffer = vec![0; amount];
        self.read_exact(&mut buffer)?;
        return Ok(buffer);
    }

    // https://en.wikipedia.org/wiki/Variable-length_quantity
    pub fn read_varuint(&mut self) -> Result<u32, ParseError> {
        let mut shift = 0;
        let mut result: u32 = 0;
        let mut i;
        loop {
            i = self.read_bytes(1)?[0];
            result |= ((i & 0x7F) as u32) << shift;
            shift += 7;
            if i & 0x80 != 0x80 {
                break;
            }
        }
        return Ok(result);
    }

    pub fn read_f32(&mut self) -> Result<f32, ParseError> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        return Ok(f32::from_le_bytes(buffer));
    }

    pub fn read_u8(&mut self) -> Result<u8, ParseError> {
        let mut buffer = [0];
        self.read_exact(&mut buffer)?;
        return Ok(u8::from_le_bytes(buffer));
    }

    pub fn read_u16(&mut self) -> Result<u16, ParseError> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        return Ok(u16::from_le_bytes(buffer));
    }

    pub fn read_u32(&mut self) -> Result<u32, ParseError> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        println!("{buffer:?}");
        return Ok(u32::from_le_bytes(buffer));
    }
}
