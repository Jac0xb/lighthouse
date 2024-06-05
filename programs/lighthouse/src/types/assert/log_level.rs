use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
    Silent = 0,
    PlaintextMessage = 1,
    EncodedMessage = 2,
    EncodedNoop = 3,
    FailedPlaintextMessage = 0x10,
    FailedEncodedMessage = 0x11,
    FailedEncodedNoop = 0x12,
}

impl LogLevel {
    #[inline(always)]
    pub fn ignore_success(&self) -> bool {
        *self as u8 & 0x10 != 0
    }
}
