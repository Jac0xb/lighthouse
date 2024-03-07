use crate::types::{
    assert::{
        AccountDataAssertion, AccountDataDeltaAssertion, AccountInfoAssertion, LogLevel,
        MerkleTreeAssertion, MintAccountAssertion, StakeAccountAssertion, SysvarClockAssertion,
        TokenAccountAssertion, UpgradeableLoaderStateAssertion,
    },
    write::WriteType,
};
use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(BorshSerialize, BorshDeserialize, ShankInstruction)]
#[rustfmt::skip]
pub(crate) enum LighthouseInstruction {
    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "system_program", desc = "System program")]
    #[account(2, name = "payer", desc = "Payer account", signer)]
    #[account(3, name = "memory_account", desc = "Memory account", writable)]
    #[account(4, name = "source_account", desc = "System program")]
    MemoryWrite { 
        memory_index: u8,
        memory_account_bump: u8,
        memory_offset: u16,
        write_type: WriteType,
    },

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "system_program", desc = "System program")]
    #[account(2, name = "payer", desc = "Payer account", signer)]
    #[account(3, name = "memory_account", desc = "Memory account", writable)]
    MemoryClose { memory_index: u8, memory_account_bump: u8 },

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
    AssertMerkleTreeAccount { log_level: LogLevel, assertion: MerkleTreeAssertion },
}
