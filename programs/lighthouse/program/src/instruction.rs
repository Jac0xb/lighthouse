use borsh::{BorshDeserialize, BorshSerialize};
use kaigan::types::RemainderVec;
use shank::ShankInstruction;

use crate::{
    processor::{CreateMemoryAccountParameters, WriteParameters},
    types::{
        AccountDataAssertion, AccountDataDiffAssertion, AccountInfoAssertion, MintAccountAssertion,
        SysvarClockAssertion, TokenAccountAssertion,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Clone, ShankInstruction)]
#[rustfmt::skip]
pub enum LighthouseInstruction {
    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "payer", desc = "Payer account")]
    #[account(2, name = "memory_account", desc = "Memory account")]
    #[account(3, name = "system_program", desc = "System program")]
    CreateMemoryAccount(CreateMemoryAccountParameters),

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "payer", desc = "Payer account")]
    #[account(2, name = "memory_account", desc = "Memory account")]
    #[account(3, name = "source_account", desc = "System program")]
    Write(WriteParameters),

    #[account(0, name = "target_account", desc = "Target account")]
    AssertAccountData(AccountDataAssertion),

    #[account(0, name = "left_account", desc = "Left account")]
    #[account(1, name = "right_account", desc = "Right account")]
    AssertAccountDataDiff(AccountDataDiffAssertion),

    #[account(0, name = "target_account", desc = "Target account")]
    AssertAccountInfo(AccountInfoAssertion),

    #[account(0, name = "target_account", desc = "Target account")]
    AssertMintAccount(MintAccountAssertion),

    #[account(0, name = "target_account", desc = "Target account")]
    AssertMintAccountMulti(RemainderVec<MintAccountAssertion>),

    #[account(0, name = "target_account", desc = "Target account")]
    AssertTokenAccount(TokenAccountAssertion),

    #[account(0, name = "target_account", desc = "Target account")]
    #[account(1, name = "lighthouse_program", desc = "Lighthouse Program")]
    AssertTokenAccountMulti(RemainderVec::<TokenAccountAssertion>),

    // No accounts
    AssertSysvarClock(SysvarClockAssertion),
}
