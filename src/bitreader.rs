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
    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), ParseError> {
        self.offset += buffer.len();
        self.bits.read_exact(buffer)?;
        return Ok(());
    }

    pub fn read_bytes(&mut self, amount: usize) -> Result<Vec<u8>, ParseError> {
        let mut buffer = vec![0; amount];
        self.read_exact(&mut buffer)?;
        return Ok(buffer);
    }

    pub fn read_string(&mut self, length: usize) -> Result<String, ParseError> {
        return Ok(String::from_utf8(self.read_bytes(length)?)
            .map_err(|_| ParseError::invalid("String contains invalid utf-8"))?);
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

    pub fn read_bool(&mut self) -> Result<bool, ParseError> {
        return Ok(self.read_u8()? > 0);
    }

    pub fn read_f32(&mut self) -> Result<f32, ParseError> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        return Ok(f32::from_le_bytes(buffer));
    }

    pub fn read_f64(&mut self) -> Result<f64, ParseError> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        return Ok(f64::from_le_bytes(buffer));
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
        return Ok(u32::from_le_bytes(buffer));
    }

    /// Parse uuid from data in little endian format
    /// Using Variant 2 UUID's with mixed endianess (https://en.wikipedia.org/wiki/Universally_unique_identifier#Encoding)
    pub fn read_uuid(&mut self) -> Result<String, ParseError> {
        let uuid_length = self.read_varuint()?;
        if uuid_length != 16 {
            return Err(ParseError::invalid("Expected UUID length to be 16 bytes"));
        }

        println!("{} {uuid_length}", self.offset());
        let mut uuid_bytes: Vec<u8> = self.read_bytes(uuid_length as usize)?;

        // Set first 3 uuid sections to big endianness
        uuid_bytes[..4].reverse();
        uuid_bytes[4..6].reverse();
        uuid_bytes[6..8].reverse();

        // put bytes in a single number
        let uuid_bytes = u128::from_be_bytes(
            uuid_bytes
                .try_into()
                .map_err(|_| ParseError::invalid("Failed to parse uuid bytes into integer"))?,
        );

        // turn hexidecimals into string
        let uuid = format!("{uuid_bytes:032x}");
        // add slashes
        let uuid = format!(
            "{}-{}-{}-{}-{}",
            &uuid[..8],
            &uuid[8..12],
            &uuid[12..16],
            &uuid[16..20],
            &uuid[20..],
        );

        Ok(uuid)
    }
}
