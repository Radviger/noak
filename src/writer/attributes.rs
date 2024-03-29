pub mod code;
mod debug;
mod enclosing_method;
mod field;
mod inner_classes;
mod method;

use std::fmt;
use std::marker::PhantomData;

pub use enclosing_method::*;
pub use inner_classes::*;
pub use method::*;

use crate::error::*;
use crate::writer::{cpool, encoding::*};

pub struct AttributeWriter<Ctx, State: AttributeWriterState::State> {
    context: Ctx,
    _marker: PhantomData<State>,
}

impl<Ctx: EncoderContext> AttributeWriter<Ctx, AttributeWriterState::Start> {
    fn attribute_writer<I>(&mut self, name: I) -> Result<LengthWriter<Ctx>, EncodeError>
    where
        I: cpool::Insertable<cpool::Utf8>,
    {
        let index = name.insert(&mut self.context)?;
        self.context.encoder().write(index)?;

        LengthWriter::new(&mut self.context)
    }

    pub fn raw_attribute<I>(
        mut self,
        name: I,
        bytes: &[u8],
    ) -> Result<AttributeWriter<Ctx, AttributeWriterState::End>, EncodeError>
    where
        I: cpool::Insertable<cpool::Utf8>,
    {
        let length = u32::try_from(bytes.len())
            .map_err(|_| EncodeError::with_context(EncodeErrorKind::TooManyBytes, Context::AttributeContent))?;
        let index = name.insert(&mut self.context)?;
        self.context.encoder().write(index)?.write(length)?.write(bytes)?;

        Ok(AttributeWriter {
            context: self.context,
            _marker: PhantomData,
        })
    }
}

impl<Ctx: EncoderContext> WriteAssembler for AttributeWriter<Ctx, AttributeWriterState::Start> {
    type Context = Ctx;

    fn new(context: Self::Context) -> Result<Self, EncodeError> {
        Ok(AttributeWriter {
            context,
            _marker: PhantomData,
        })
    }
}

impl<Ctx: EncoderContext> WriteDisassembler for AttributeWriter<Ctx, AttributeWriterState::End> {
    type Context = Ctx;

    fn finish(self) -> Result<Self::Context, EncodeError> {
        Ok(self.context)
    }
}

impl<Ctx, State: AttributeWriterState::State> fmt::Debug for AttributeWriter<Ctx, State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AttributeWriter").finish()
    }
}

enc_state!(pub mod AttributeWriterState: Start, End);
