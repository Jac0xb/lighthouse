use anchor_lang::prelude::*;
use mpl_token_metadata::error::MetadataError;
use num_traits::FromPrimitive;

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
    #[msg("CacheOutOfRange")]
    CacheOutOfRange,
    #[msg("AccountBorrowFailed")]
    AccountBorrowFailed,
    #[msg("InvalidAccount")]
    InvalidAccount,
}
