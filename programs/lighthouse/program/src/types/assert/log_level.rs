use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
    Silent = 0,
    PlaintextMsgLog = 1,
}

impl LogLevel {
    pub fn is_silent(&self) -> bool {
        self == &LogLevel::Silent
    }

    pub fn is_plaintextmsg_log(&self) -> bool {
        self == &LogLevel::PlaintextMsgLog
    }
}
