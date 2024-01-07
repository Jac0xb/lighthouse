use anchor_lang::prelude::*;

#[error_code]
pub enum LighthouseError {
    #[msg("Unimplemented")]
    Unimplemented,
}
