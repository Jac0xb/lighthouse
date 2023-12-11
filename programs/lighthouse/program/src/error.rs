use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
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
    #[msg("InvalidAccount")]
    InvalidAccount,

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
