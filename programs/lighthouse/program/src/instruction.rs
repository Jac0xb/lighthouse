use crate::types::{
    assert::{
        AccountDataAssertion, AccountDeltaAssertion, AccountInfoAssertion, LogLevel,
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
    #[account(1, name = "payer", desc = "Payer account", signer)]
    #[account(2, name = "memory_account", desc = "Memory account", writable)]
    MemoryClose { memory_index: u8, memory_account_bump: u8 },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertAccountData { log_level: LogLevel, assertion: AccountDataAssertion },

    #[account(0, name = "account_a", desc = "Account A where the delta is calculated from")]
    #[account(1, name = "account_b", desc = "Account B where the delta is calculated to")]
    AssertAccountDelta { log_level: LogLevel, assertion: AccountDeltaAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertAccountInfo { log_level: LogLevel, assertion: AccountInfoAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertMintAccount { log_level: LogLevel, assertion: MintAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertMintAccountMulti { log_level: LogLevel, assertions: Vec<MintAccountAssertion> },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertTokenAccount { log_level: LogLevel, assertion: TokenAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    #[account(1, name = "lighthouse_program", desc = "Lighthouse Program")]
    AssertTokenAccountMulti { log_level: LogLevel, assertions: Vec<TokenAccountAssertion> },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertStakeAccount { log_level: LogLevel, assertion: StakeAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertUpgradeableLoaderAccount { log_level: LogLevel, assertion: UpgradeableLoaderStateAssertion },

    // No accounts
    AssertSysvarClock { log_level : LogLevel, assertion: SysvarClockAssertion },

    #[account(0, name = "target_merkle_tree", desc = "Target merkle tree account to be asserted")]
    #[account(1, name = "root", desc = "The current root of the merkle tree")]
    #[account(2, name = "spl_account_compression", desc = "SPL account compression program")]
    AssertMerkleTreeAccount { log_level: LogLevel, assertion: MerkleTreeAssertion },
}
