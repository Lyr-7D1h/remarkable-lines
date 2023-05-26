use crate::{bitreader::Readable, ParseError};

use super::{layer::Layer, Parse};

#[derive(Debug)]
pub struct Page {
    pub layers: Vec<Layer>,
}

impl Parse for Page {
    fn parse(
        version: u32,
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, crate::ParseError> {
        let amount_layers = reader.read_u32()?;

        Ok(Page {
            layers: (0..amount_layers)
                .map(|_| Layer::parse(version, reader))
                .collect::<Result<Vec<Layer>, ParseError>>()?,
        })
    }
}
