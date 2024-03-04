use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

use crate::{
    processor::{AssertMerkleLeafParameters, CreateMemoryAccountParameters, WriteParameters},
    types::{
        AccountDataAssertion, AccountDataDeltaAssertion, AccountInfoAssertion, LogLevel,
        MintAccountAssertion, StakeAccountAssertion, SysvarClockAssertion, TokenAccountAssertion,
        UpgradeableLoaderStateAssertion,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Clone, ShankInstruction, Debug)]
#[rustfmt::skip]
pub enum LighthouseInstruction {
    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "payer", desc = "Payer account", signer)]
    #[account(2, name = "memory_account", desc = "Memory account", writable)]
    #[account(3, name = "system_program", desc = "System program")]
    CreateMemoryAccount(CreateMemoryAccountParameters),

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "payer", desc = "Payer account", signer)]
    #[account(2, name = "memory_account", desc = "Memory account", writable)]
    #[account(3, name = "source_account", desc = "System program")]
    Write(WriteParameters),

    #[account(0, name = "target_account", desc = "Target account")]
    AssertAccountData { log_level: LogLevel, assertion: AccountDataAssertion },

    #[account(0, name = "left_account", desc = "Left account")]
    #[account(1, name = "right_account", desc = "Right account")]
    AssertAccountDataDelta { log_level: LogLevel, assertion: AccountDataDeltaAssertion },

    #[account(0, name = "target_account", desc = "Target account")]
    AssertAccountInfo { log_level: LogLevel, assertion: AccountInfoAssertion },

    #[account(0, name = "target_account", desc = "Target account")]
    AssertMintAccount { log_level: LogLevel, assertion: MintAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account")]
    AssertMintAccountMulti { log_level: LogLevel, assertions: Vec<MintAccountAssertion> },

    #[account(0, name = "target_account", desc = "Target account")]
    AssertTokenAccount { log_level: LogLevel, assertion: TokenAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account")]
    #[account(1, name = "lighthouse_program", desc = "Lighthouse Program")]
    AssertTokenAccountMulti { log_level: LogLevel, assertions: Vec<TokenAccountAssertion> },

    #[account(0, name = "target_account", desc = "Target account")]
    AssertStakeAccount { log_level: LogLevel, assertion: StakeAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account")]
    AssertUpgradeableLoaderAccount { log_level: LogLevel, assertion: UpgradeableLoaderStateAssertion },

    // No accounts
    AssertSysvarClock { log_level : LogLevel, assertion: SysvarClockAssertion },

    #[account(0, name = "merkle_tree", desc = "Merkle tree account")]
    #[account(1, name = "root", desc = "Root account")]
    #[account(2, name = "spl_account_compression", desc = "SPL account compression program")]
    AssertMerkleTreeAccount(LogLevel, AssertMerkleLeafParameters),
}
