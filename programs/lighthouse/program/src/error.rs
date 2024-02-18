use solana_program::program_error::ProgramError;
// use anchor_lang::prelude::*;
use thiserror::Error;

// IntoPrimitive
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
    OwnerMismatch = 6008,
    #[error("AccountNotInitialized")]
    AccountNotInitialized = 6009,
    #[error("UnauthorizedIxEntry")]
    UnauthorizedIxEntry = 6010,
    #[error("InvalidDataLength")]
    InvalidDataLength = 6011,
    #[error("AccountOwnerValidationFailed")]
    AccountOwnerValidationFailed = 6013,
    #[error("AccountFundedValidationFailed")]
    AccountFundedValidationFailed = 6014,
    #[error("AccountDiscriminatorValidationFailed")]
    AccountDiscriminatorValidationFailed = 6015,
    #[error("AccountValidaitonFailed")]
    AccountValidaitonFailed = 6016,
}

impl From<LighthouseError> for ProgramError {
    fn from(e: LighthouseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// #[cfg(test)]
// pub fn assert_is_anchor_error(err: Error, expected_error: LighthouseError) {
//     match err {
//         Error::ProgramError(err) => {
//             assert_eq!(
//                 err.program_error,
//                 ProgramError::Custom(expected_error as u32 + 1)
//             );
//         }
//         Error::AnchorError(err) => {
//             assert_eq!(err.error_code_number, 6000 + expected_error as u32);
//         }
//     }
// }

#[cfg(test)]
pub fn assert_is_program_error(err: ProgramError, expected_error: ProgramError) {
    assert_eq!(err, expected_error);
}
