use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

pub mod error;
pub mod processor;

use crate::processor::*;

declare_id!("Test111111111111111111111111111111111111111");

#[program]
pub mod test_program {
    use super::*;

    pub(crate) fn create_test_account_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateTestAccountV1<'info>>,
        random: bool,
    ) -> Result<()> {
        processor::create_test_account(ctx, random)
    }

    pub(crate) fn write<'info>(
        ctx: Context<'_, '_, '_, 'info, Write<'info>>,
        memory_bump: u8,
    ) -> Result<()> {
        processor::write(ctx, memory_bump)?;

        Ok(())
    }
}
