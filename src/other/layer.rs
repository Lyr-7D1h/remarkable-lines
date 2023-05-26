use crate::{bitreader::Readable, ParseError};

use super::{line::Line, Parse};

#[derive(Debug)]
pub struct Layer {
    pub lines: Vec<Line>,
}

impl Parse for Layer {
    fn parse(
        version: u32,
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, crate::ParseError> {
        let amount_lines = reader.read_u32()?;
        let lines = (0..amount_lines)
            .map(|_| Line::parse(version, reader))
            .collect::<Result<Vec<Line>, ParseError>>()?;
        Ok(Layer { lines })
    }
}
