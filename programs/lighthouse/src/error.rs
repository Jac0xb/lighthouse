use std::{io, ops::Range};

use solana_program::{msg, program_error::ProgramError};
use thiserror::Error;

#[macro_export]
macro_rules! err {
    ($error:expr) => {
        solana_program::program_error::ProgramError::from($error)
    };
}

#[macro_export]
macro_rules! err_msg {
    ($msg:expr, $error:expr) => {
        // Print the message and error
        solana_program::msg!("{}: {:?}", $msg, $error);
    };
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LighthouseError {
    // Processor errors
    #[error("Invalid instruction")]
    InvalidInstructionData = 6000,
    #[error("AssertionFailed")]
    AssertionFailed = 6001,
    #[error("NotEnoughAccounts")]
    NotEnoughAccounts = 6002,
    #[error("BumpNotFound")]
    BumpNotFound = 6003,

    // Solana program error handling
    #[error("AccountBorrowFailed")]
    AccountBorrowFailed = 6004,

    // Slice access errors
    #[error("RangeOutOfBounds")]
    RangeOutOfBounds = 6005,
    #[error("IndexOutOfBounds")]
    IndexOutOfBounds = 6006,

    // De(ser)ialization errors
    #[error("FailedToDeserialize")]
    FailedToDeserialize = 6007,
    #[error("FailedToSerialize")]
    FailedToSerialize = 6008,

    // Account validation errors
    #[error("AccountOwnerMismatch")]
    AccountOwnerMismatch = 6009,
    #[error("AccountKeyMismatch")]
    AccountKeyMismatch = 6010,
    #[error("AccountNotInitialized")]
    AccountNotInitialized = 6011,
    #[error("AccountOwnerValidationFailed")]
    AccountOwnerValidationFailed = 6012,
    #[error("AccountFundedValidationFailed")]
    AccountFundedValidationFailed = 6013,
    #[error("AccountDiscriminatorValidationFailed")]
    AccountDiscriminatorValidationFailed = 6014,
    #[error("AccountValidaitonFailed")]
    AccountValidationFailed = 6015,

    // Guards
    #[error("CrossProgramInvokeViolation")]
    CrossProgramInvokeViolation = 6016,
}

impl LighthouseError {
    pub fn map_multi_err(e: ProgramError, i: u32) -> ProgramError {
        if e == LighthouseError::AssertionFailed.into() {
            ProgramError::Custom(0x1900 + i)
        } else {
            e
        }
    }

    pub fn failed_borrow_err(e: ProgramError) -> ProgramError {
        err_msg!("Failed to borrow data for target account", e);
        err!(LighthouseError::AccountBorrowFailed)
    }

    pub fn stake_deser_err(e: io::Error) -> ProgramError {
        err_msg!("Failed to deserialize stake account state", e);
        err!(LighthouseError::FailedToDeserialize)
    }

    pub fn oob_err(r: Range<usize>) -> ProgramError {
        msg!("Failed to access account data range {:?}: out of bounds", r);
        LighthouseError::RangeOutOfBounds.into()
    }

    pub fn serialize_err(e: io::Error) -> ProgramError {
        err_msg!("Failed to serialize data", e);
        err!(LighthouseError::FailedToSerialize)
    }
}

impl From<LighthouseError> for ProgramError {
    fn from(e: LighthouseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
