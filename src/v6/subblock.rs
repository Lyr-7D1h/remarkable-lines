use crate::{bitreader::Readable, Bitreader, ParseError};

use super::{
    tag::{Tag, TagType},
    TypeParse,
};

pub struct SubBlock {
    tag: Tag,
    size: u32,
    position: u64,
}

impl SubBlock {
    pub fn validate_tag(self, index: u32) -> Result<Self, crate::ParseError> {
        self.tag.validate(TagType::Length4, index)?;
        return Ok(self);
    }

    pub fn validate_size(
        &self,
        reader: &mut Bitreader<impl Readable>,
    ) -> Result<(), crate::ParseError> {
        let expected_offset = self.position + self.size as u64;
        let end_offset = reader.position();
        if expected_offset != end_offset {
            return Err(ParseError::invalid(format!(
                "Did not read expected size of subblock. got {end_offset:x} expected {expected_offset:x}" 
            )));
        }
        Ok(())
    }
}

impl TypeParse for SubBlock {
    fn parse(reader: &mut crate::Bitreader<impl Readable>) -> Result<Self, crate::ParseError>
    where
        Self: Sized,
    {
        let tag = Tag::parse(reader)?;
        let size = reader.read_u32()?;
        let position = reader.position();

        Ok(Self {
            tag,
            size,
            position,
        })
    }
}
