use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

use crate::{
    processor::{CreateMemoryAccountParameters, WriteParameters},
    types::{
        AccountDataAssertion, AccountDataHashAssertionTuple, AccountInfoFieldAssertion,
        MintAccountFieldAssertion, SysvarClockFieldAssertion, TokenAccountFieldAssertion,
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

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "target_account", desc = "Target account")]
    AssertAccountData(AccountDataAssertion),

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "target_account", desc = "Target account")]
    AssertDataHash(AccountDataHashAssertionTuple),

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "target_account", desc = "Target account")]
    AssertAccountInfo(AccountInfoFieldAssertion),

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "target_account", desc = "Target account")]
    AssertMintAccountField(MintAccountFieldAssertion),

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "target_account", desc = "Target account")]
    AssertTokenAccountField(TokenAccountFieldAssertion),

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    AssertSysvarClockField(SysvarClockFieldAssertion),
}
