use anchor_lang::prelude::*;

#[error_code]
pub enum BlackhatError {
    #[msg("Unimplemented")]
    Unimplemented,

    #[msg("NumericalOverflowError")]
    NumericalOverflowError,
}
