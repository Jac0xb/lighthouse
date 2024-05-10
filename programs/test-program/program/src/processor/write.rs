use anchor_lang::prelude::*;
use lighthouse_sdk::types::{ClockField, WriteType};

#[derive(Accounts)]
pub(crate) struct Write<'info> {
    #[account(mut)]
    pub(crate) signer: Signer<'info>,
    /// CHECK: Ligma
    #[account(mut)]
    pub(crate) memory: AccountInfo<'info>,
    /// CHECK: Ligma
    pub(crate) source_account: AccountInfo<'info>,
    /// CHECK: Ligma
    pub(crate) lighthouse: AccountInfo<'info>,
    pub(crate) system_program: Program<'info, System>,
}

pub(crate) fn write<'info>(
    ctx: Context<'_, '_, '_, 'info, Write<'info>>,
    memory_bump: u8,
) -> Result<()> {
    lighthouse_sdk::cpi::MemoryWriteCpiBuilder::new(&ctx.accounts.lighthouse)
        .payer(&ctx.accounts.signer.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .source_account(&ctx.accounts.source_account.to_account_info())
        .write_type(WriteType::Clock(ClockField::UnixTimestamp))
        .write_offset(0u8.into())
        .memory_bump(memory_bump)
        .memory(&ctx.accounts.memory.to_account_info())
        .memory_id(0)
        .program_id(&ctx.accounts.lighthouse)
        .invoke()?;

    Ok(())
}
