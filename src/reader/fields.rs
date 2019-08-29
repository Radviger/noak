use crate::encoding::{Decode, Decoder};
use crate::error::*;
use crate::header::AccessFlags;
use crate::reader::{attributes, cpool, Attributes};
use std::fmt;
use std::iter::FusedIterator;

pub struct Field<'a> {
    access_flags: AccessFlags,
    name: cpool::Index<cpool::Utf8<'a>>,
    descriptor: cpool::Index<cpool::Utf8<'a>>,
    attributes: Attributes<'a>,
}

impl<'a> Field<'a> {
    pub fn access_flags(&self) -> AccessFlags {
        self.access_flags
    }

    pub fn name(&self) -> cpool::Index<cpool::Utf8<'a>> {
        self.name
    }

    pub fn descriptor(&self) -> cpool::Index<cpool::Utf8<'a>> {
        self.descriptor
    }

    pub fn attribute_indices(&self) -> Attributes<'a> {
        self.attributes.clone()
    }
}

impl<'a> Decode<'a> for Field<'a> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, DecodeError> {
        Ok(Field {
            access_flags: decoder.read()?,
            name: decoder.read()?,
            descriptor: decoder.read()?,
            attributes: decoder.read()?,
        })
    }
}

impl<'a> fmt::Debug for Field<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Field").finish()
    }
}

/// An iterator over the fields of a class
#[derive(Clone)]
pub struct Fields<'a> {
    decoder: Decoder<'a>,
}

impl<'a> Decode<'a> for Fields<'a> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, DecodeError> {
        let mut field_decoder = decoder.clone();
        field_decoder.advance(2)?;
        skip_fields(decoder)?;
        let field_length = field_decoder.bytes_remaining() - decoder.bytes_remaining();

        Ok(Fields {
            decoder: field_decoder.limit(field_length, Context::Fields)?,
        })
    }
}

fn skip_fields(decoder: &mut Decoder) -> Result<(), DecodeError> {
    let count: u16 = decoder.read()?;

    for _ in 0..count {
        // skipping the access flags, name and descriptor
        decoder.advance(6)?;
        attributes::skip_attributes(decoder)?;
    }

    Ok(())
}

impl<'a> Iterator for Fields<'a> {
    type Item = Field<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.decoder.read().ok()
    }
}

impl<'a> FusedIterator for Fields<'a> {}

impl<'a> fmt::Debug for Fields<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Fields").finish()
    }
}
