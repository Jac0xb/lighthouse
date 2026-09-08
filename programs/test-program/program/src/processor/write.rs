use anchor_lang::prelude::*;
use lighthouse_sdk::instructions::MemoryWriteBuilder;
use lighthouse_sdk::types::{ClockField, WriteType};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};

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
    let instruction = MemoryWriteBuilder::new()
        .payer(ctx.accounts.signer.key().to_bytes().into())
        .system_program(ctx.accounts.system_program.key().to_bytes().into())
        .source_account(ctx.accounts.source_account.key().to_bytes().into())
        .write_type(WriteType::Clock(ClockField::UnixTimestamp))
        .write_offset(0u8.into())
        .memory_bump(memory_bump)
        .memory(ctx.accounts.memory.key().to_bytes().into())
        .memory_id(0)
        .program_id(ctx.accounts.lighthouse.key().to_bytes().into())
        .instruction();

    // Anchor 0.29 uses the Solana 1.x instruction and account types.
    let instruction = Instruction {
        program_id: Pubkey::new_from_array(instruction.program_id.to_bytes()),
        accounts: instruction
            .accounts
            .into_iter()
            .map(|account| AccountMeta {
                pubkey: Pubkey::new_from_array(account.pubkey.to_bytes()),
                is_signer: account.is_signer,
                is_writable: account.is_writable,
            })
            .collect(),
        data: instruction.data,
    };
    invoke(
        &instruction,
        &[
            ctx.accounts.lighthouse.clone(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.memory.clone(),
            ctx.accounts.source_account.clone(),
        ],
    )?;

    Ok(())
}
