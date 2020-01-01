use crate::error::*;
use crate::writer::{attributes::code::*, encoding::*};

pub struct TableSwitchWriter<'a, 'b> {
    code_writer: &'b mut CodeWriter<'a>,
    state: WriteState,
    remaining: u32,
}

impl<'a, 'b> TableSwitchWriter<'a, 'b> {
    pub(super) fn new(
        code_writer: &'b mut CodeWriter<'a>,
        offset: Offset,
    ) -> Result<Self, EncodeError> {
        code_writer.class_writer.encoder.write(0xaau8)?;
        for _ in 0..3 - (offset.get() & 3) {
            code_writer.class_writer.encoder.write(0u8)?;
        }

        Ok(TableSwitchWriter {
            code_writer,
            state: WriteState::Default,
            remaining: 0,
        })
    }

    pub(super) fn finish(self) -> Result<&'b mut CodeWriter<'a>, EncodeError> {
        EncodeError::result_from_state(self.state, &WriteState::Finished, Context::Code)?;
        Ok(self.code_writer)
    }

    pub fn write_default(&mut self, label: LabelRef) -> Result<&mut Self, EncodeError> {
        EncodeError::result_from_state(self.state, &WriteState::Default, Context::Code)?;

        self.code_writer.class_writer.encoder.write(label.0)?;
        self.state = WriteState::Low;
        Ok(self)
    }

    pub fn write_low(&mut self, low: i32) -> Result<&mut Self, EncodeError> {
        EncodeError::result_from_state(self.state, &WriteState::Low, Context::Code)?;

        self.code_writer.class_writer.encoder.write(low)?;
        self.remaining = low as u32;

        self.state = WriteState::High;
        Ok(self)
    }

    pub fn write_high(&mut self, high: i32) -> Result<&mut Self, EncodeError> {
        EncodeError::result_from_state(self.state, &WriteState::High, Context::Code)?;

        self.code_writer.class_writer.encoder.write(high)?;

        let low = self.remaining as i32;
        if low > high {
            return Err(EncodeError::with_context(
                EncodeErrorKind::IncorrectBounds,
                Context::Code,
            ));
        }

        self.remaining = (high - low + 1) as u32;

        self.state = WriteState::Jumps;
        Ok(self)
    }

    pub fn write_jump(&mut self, label: LabelRef) -> Result<&mut Self, EncodeError> {
        EncodeError::result_from_state(self.state, &WriteState::Jumps, Context::Code)?;
        self.code_writer.class_writer.encoder.write(label.0)?;
        if self.remaining == 1 {
            self.state = WriteState::Finished;
        } else if self.remaining == 0 {
            return Err(EncodeError::with_context(
                EncodeErrorKind::CantChangeAnymore,
                Context::Code,
            ));
        }

        self.remaining -= 1;

        Ok(self)
    }
}

/// What's written next
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WriteState {
    Default,
    Low,
    High,
    Jumps,
    Finished,
}