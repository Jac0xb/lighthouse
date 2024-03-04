use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, PartialOrd, Clone)]
#[repr(u8)]
pub enum LogLevel {
    Silent = 0,
    PlaintextMsgLog = 1,
    SerializedMsgLog = 2,
    CpiLog = 3,
}

impl LogLevel {
    pub fn is_silent(&self) -> bool {
        self == &LogLevel::Silent
    }

    pub fn is_plaintextmsg_log(&self) -> bool {
        self == &LogLevel::PlaintextMsgLog
    }

    pub fn is_serializedmsg_log(&self) -> bool {
        self == &LogLevel::SerializedMsgLog
    }

    pub fn is_cpi_log(&self) -> bool {
        self == &LogLevel::CpiLog
    }
}
