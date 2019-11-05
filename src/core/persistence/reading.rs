use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Reading {
    /// Timestamp when the value was actually measured. This may be earlier than a moment of sending the message.
    pub timestamp: DateTime<Local>,

    /// Attached value.
    pub value: Value,
}
