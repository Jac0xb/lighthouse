pub mod account;

pub use account::*;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::ops::Deref;

#[derive(Clone)]
pub struct Program<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> Program<'a, 'info> {
    pub fn new(
        info: &'a AccountInfo<'info>,
        expected_program_id: &Pubkey,
    ) -> Result<Program<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.key == expected_program_id,
            ProgramError::IncorrectProgramId,
            "Incorrect program id",
        )?;
        assert_with_msg(
            info.executable,
            ProgramError::IncorrectProgramId,
            "Isn't a program",
        )?;

        Ok(Self { info })
    }
}

impl<'a, 'info> AsRef<AccountInfo<'info>> for Program<'a, 'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

impl<'a, 'info> Deref for Program<'a, 'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}

#[derive(Clone)]
pub struct Signer<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> Signer<'a, 'info> {
    pub fn new(info: &'a AccountInfo<'info>) -> Result<Signer<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.is_signer,
            ProgramError::MissingRequiredSignature,
            "Missing required signature",
        )?;
        Ok(Self { info })
    }

    pub fn new_with_key(
        info: &'a AccountInfo<'info>,
        key: &Pubkey,
    ) -> Result<Signer<'a, 'info>, ProgramError> {
        let signer = Self::new(info)?;
        assert_with_msg(
            signer.key == key,
            ProgramError::MissingRequiredSignature,
            "Incorrect key for signer",
        )?;
        Ok(signer)
    }

    pub fn new_payer(info: &'a AccountInfo<'info>) -> Result<Signer<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.is_writable,
            ProgramError::InvalidInstructionData,
            "Payer is not writable",
        )?;
        assert_with_msg(
            info.is_signer,
            ProgramError::MissingRequiredSignature,
            "Missing required signature for payer",
        )?;
        Ok(Self { info })
    }
}

impl<'a, 'info> AsRef<AccountInfo<'info>> for Signer<'a, 'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

impl<'a, 'info> Deref for Signer<'a, 'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}

#[track_caller]
#[inline(always)]
pub fn assert_with_msg(v: bool, err: impl Into<ProgramError>, msg: &str) -> ProgramResult {
    if v {
        Ok(())
    } else {
        let caller = std::panic::Location::caller();
        msg!("{}. \n{}", msg, caller);
        Err(err.into())
    }
}
