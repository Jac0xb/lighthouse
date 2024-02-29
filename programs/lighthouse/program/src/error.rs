use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LighthouseError {
    #[error("Invalid instruction")]
    InvalidInstructionData = 6000,
    #[error("Invalid market parameters error")]
    Unimplemented = 6001,
    #[error("AssertionFailed")]
    AssertionFailed = 6002,
    #[error("NotEnoughAccounts")]
    NotEnoughAccounts = 6003,
    #[error("DataValueMismatch")]
    DataValueMismatch = 6004,
    #[error("UnsupportedOperator")]
    UnsupportedOperator = 6005,
    #[error("OutOfRange")]
    OutOfRange = 6006,
    #[error("AccountBorrowFailed")]
    AccountBorrowFailed = 6007,
    #[error("AccountNotTokenAccount")]
    AccountOwnerMismatch = 6008,
    #[error("AccountNotInitialized")]
    AccountNotInitialized = 6009,
    #[error("UnauthorizedIxEntry")]
    UnauthorizedIxEntry = 6010,
    #[error("InvalidDataLength")]
    InvalidDataLength = 6011,
    #[error("FailedToDeserialize")]
    FailedToDeserialize = 6012,
    #[error("AccountOwnerValidationFailed")]
    AccountOwnerValidationFailed = 6013,
    #[error("AccountFundedValidationFailed")]
    AccountFundedValidationFailed = 6014,
    #[error("AccountDiscriminatorValidationFailed")]
    AccountDiscriminatorValidationFailed = 6015,
    #[error("AccountValidaitonFailed")]
    AccountValidaitonFailed = 6016,
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress = 6017,
    #[error("SerializationFailed")]
    SerializationFailed = 6018,
}

impl From<LighthouseError> for ProgramError {
    fn from(e: LighthouseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[cfg(test)]
pub fn assert_is_program_error(err: ProgramError, expected_error: ProgramError) {
    assert_eq!(err, expected_error);
}

#[macro_export]
macro_rules! err {
    ($error:expr) => {
        ::solana_program::program_error::ProgramError::from($error)
    };
}

#[macro_export]
macro_rules! err_msg {
    ($msg:expr, $error:expr) => {
        // Print the message and error
        msg!("{}: {:?}", $msg, $error);
    };
}
