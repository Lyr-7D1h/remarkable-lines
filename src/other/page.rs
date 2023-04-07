use crate::{Parse, ParseError};

use super::layer::Layer;

#[derive(Debug)]
pub struct Page {
    pub layers: Vec<Layer>,
}

impl Parse for Page {
    fn parse<N: std::io::Read>(
        version: u32,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, crate::ParseError> {
        let amount_layers = reader.read_u32()?;

        Ok(Page {
            layers: (0..amount_layers)
                .map(|_| Layer::parse(version, reader))
                .collect::<Result<Vec<Layer>, ParseError>>()?,
        })
    }
}
