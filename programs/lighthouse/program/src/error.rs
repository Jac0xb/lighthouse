use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum LighthouseError {
    #[msg("Unimplemented")]
    Unimplemented,
    #[msg("AssertionFailed")]
    AssertionFailed,
    #[msg("NotEnoughAccounts")]
    NotEnoughAccounts,
    #[msg("DataValueMismatch")]
    DataValueMismatch,
    #[msg("UnsupportedOperator")]
    UnsupportedOperator,
    #[msg("OutOfRange")]
    OutOfRange,
    #[msg("AccountBorrowFailed")]
    AccountBorrowFailed,

    #[msg("AccountNotTokenAccount")]
    AccountNotTokenAccount,

    #[msg("AccountNotInitialized")]
    AccountNotInitialized,

    #[msg("UnauthorizedIxEntry")]
    UnauthorizedIxEntry,

    #[msg("InvalidDataLength")]
    InvalidDataLength,

    #[msg("AccountOutOfRange")]
    AccountOutOfRange,

    #[msg("AccountOwnerValidationFailed")]
    AccountOwnerValidationFailed,

    #[msg("AccountFundedValidationFailed")]
    AccountFundedValidationFailed,

    #[msg("AccountDiscriminatorValidationFailed")]
    AccountDiscriminatorValidationFailed,
}

#[cfg(test)]
pub fn assert_is_anchor_error(err: Error, expected_error: LighthouseError) {
    match err {
        Error::ProgramError(err) => {
            assert_eq!(
                err.program_error,
                ProgramError::Custom(expected_error as u32 + 1)
            );
        }
        Error::AnchorError(err) => {
            assert_eq!(err.error_code_number, 6000 + expected_error as u32);
        }
    }
}

#[cfg(test)]
pub fn assert_is_program_error(err: Error, expected_error: ProgramError) {
    match err {
        Error::ProgramError(err) => {
            assert_eq!(err.program_error, expected_error);
        }
        Error::AnchorError(err) => {
            panic!("Expected ProgramError, got AnchorError");
        }
    }
}
