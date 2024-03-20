use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
    Silent = 0,
    PlaintextMessage = 1,
    EncodedMessage = 2,
    EncodedNoop = 3,
}

impl LogLevel {
    pub fn is_silent(&self) -> bool {
        self == &LogLevel::Silent
    }

    pub fn is_plaintext_message(&self) -> bool {
        self == &LogLevel::PlaintextMessage
    }
}
