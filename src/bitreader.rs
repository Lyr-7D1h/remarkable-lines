use std::io::Read;

use crate::ParseError;

pub struct Bitreader<N: Read> {
    bits: N,
}

impl<N: Read> Bitreader<N> {
    pub fn new(bits: N) -> Bitreader<N> {
        Bitreader { bits }
    }

    pub fn read_bytes(&mut self, amount: usize) -> Result<Vec<u8>, ParseError> {
        let mut buffer = vec![0; amount];
        self.bits.read_exact(&mut buffer)?;
        return Ok(buffer);
    }

    pub fn read_f32(&mut self) -> Result<f32, ParseError> {
        let mut buffer = [0; 4];
        self.bits.read_exact(&mut buffer)?;
        return Ok(f32::from_le_bytes(buffer));
    }

    pub fn read_u32(&mut self) -> Result<u32, ParseError> {
        let mut buffer = [0; 4];
        self.bits.read_exact(&mut buffer)?;
        return Ok(u32::from_le_bytes(buffer));
    }

    pub fn read_u8(&mut self) -> Result<u8, ParseError> {
        let mut buffer = [0];
        self.bits.read_exact(&mut buffer)?;
        return Ok(u8::from_le_bytes(buffer));
    }
}
