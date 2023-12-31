use anchor_lang::prelude::*;

#[error_code]
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
