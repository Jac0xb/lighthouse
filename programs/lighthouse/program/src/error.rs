use solana_program::program_error::ProgramError;
use thiserror::Error;

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

impl From<LighthouseError> for ProgramError {
    fn from(e: LighthouseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

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
