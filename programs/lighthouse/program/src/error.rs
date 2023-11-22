use anchor_lang::prelude::*;
use mpl_token_metadata::error::MetadataError;
use num_traits::FromPrimitive;

#[error_code]
pub enum ProgramError {
    #[msg("AssertionFailed")]
    AssertionFailed,
    #[msg("NotEnoughAccounts")]
    NotEnoughAccounts,
    #[msg("BorshValueMismatch")]
    BorshValueMismatch,
    #[msg("UnsupportedOperator")]
    UnsupportedOperator,
}
