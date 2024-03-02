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
    #[error("RangeOutOfBounds")]
    RangeOutOfBounds = 6006,
    #[error("IndexOutOfBounds")]
    IndexOutOfBounds = 6007,
    #[error("AccountBorrowFailed")]
    AccountBorrowFailed = 6008,
    #[error("AccountNotTokenAccount")]
    AccountOwnerMismatch = 6009,
    #[error("AccountNotInitialized")]
    AccountNotInitialized = 6010,
    #[error("UnauthorizedIxEntry")]
    UnauthorizedIxEntry = 6011,
    #[error("InvalidDataLength")]
    InvalidDataLength = 6012,
    #[error("FailedToDeserialize")]
    FailedToDeserialize = 6013,
    #[error("FailedToSerialize")]
    FailedToSerialize = 6014,
    #[error("AccountOwnerValidationFailed")]
    AccountOwnerValidationFailed = 6015,
    #[error("AccountFundedValidationFailed")]
    AccountFundedValidationFailed = 6016,
    #[error("AccountDiscriminatorValidationFailed")]
    AccountDiscriminatorValidationFailed = 6017,
    #[error("AccountValidaitonFailed")]
    AccountValidaitonFailed = 6018,
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress = 6019,
    #[error("BumpNotFound")]
    BumpNotFound = 6020,
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
        solana_program::msg!("{}: {:?}", $msg, $error);
    };
}
