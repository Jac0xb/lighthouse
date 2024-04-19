use crate::types::{
    assert::{
        AccountDataAssertion, AccountDeltaAssertion, AccountInfoAssertion,
        BubblegumTreeConfigAssertion, LogLevel, MerkleTreeAssertion, MintAccountAssertion,
        StakeAccountAssertion, SysvarClockAssertion, TokenAccountAssertion,
        UpgradeableLoaderStateAssertion,
    },
    write::WriteType,
};
use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(BorshSerialize, BorshDeserialize, ShankInstruction)]
#[rustfmt::skip]
pub(crate) enum lighthausInstruction {
    #[account(0, name = "program_id", desc = "lighthaus program")]
    #[account(1, name = "system_program", desc = "System program")]
    #[account(2, name = "payer", desc = "Payer account", signer, writable)]
    #[account(3, name = "memory", desc = "Memory account", writable)]
    #[account(4, name = "source_account", desc = "Account to be written to memory")]
    MemoryWrite { 
        memory_id: u8,
        memory_bump: u8,
        write_offset: u16,
        write_type: WriteType,
    },

    #[account(0, name = "program_id", desc = "lighthaus program")]
    #[account(1, name = "payer", desc = "Payer account", signer, writable)]
    #[account(2, name = "memory", desc = "Memory account", writable)]
    MemoryClose { memory_id: u8, memory_bump: u8 },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertAccountData { log_level: LogLevel, assertion: AccountDataAssertion },

    #[account(0, name = "account_a", desc = "Account A where the delta is calculated from")]
    #[account(1, name = "account_b", desc = "Account B where the delta is calculated to")]
    AssertAccountDelta { log_level: LogLevel, assertion: AccountDeltaAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertAccountInfo { log_level: LogLevel, assertion: AccountInfoAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertAccountInfoMulti { log_level: LogLevel, assertions: Vec<AccountInfoAssertion> },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertMintAccount { log_level: LogLevel, assertion: MintAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertMintAccountMulti { log_level: LogLevel, assertions: Vec<MintAccountAssertion> },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertTokenAccount { log_level: LogLevel, assertion: TokenAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertTokenAccountMulti { log_level: LogLevel, assertions: Vec<TokenAccountAssertion> },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertStakeAccount { log_level: LogLevel, assertion: StakeAccountAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertStakeAccountMulti { log_level: LogLevel, assertions: Vec<StakeAccountAssertion> },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertUpgradeableLoaderAccount { log_level: LogLevel, assertion: UpgradeableLoaderStateAssertion },

    #[account(0, name = "target_account", desc = "Target account to be asserted")]
    AssertUpgradeableLoaderAccountMulti { log_level: LogLevel, assertions: Vec<UpgradeableLoaderStateAssertion> },

    // No accounts
    AssertSysvarClock { log_level : LogLevel, assertion: SysvarClockAssertion },

    #[account(0, name = "target_merkle_tree", desc = "Target merkle tree account to be asserted")]
    #[account(1, name = "root", desc = "The current root of the merkle tree")]
    #[account(2, name = "spl_account_compression", desc = "SPL account compression program")]
    AssertMerkleTreeAccount { log_level: LogLevel, assertion: MerkleTreeAssertion },


    #[account(0, name = "target_account", desc = "Target mpl-bubblegum tree config account to be asserted")]
    AssertBubblegumTreeConfigAccount { log_level: LogLevel, assertion: BubblegumTreeConfigAssertion },
}

impl lighthausInstruction {
    pub const fn get_name(&self) -> &'static str {
        match self {
            lighthausInstruction::MemoryWrite { .. } => "MemoryWrite",
            lighthausInstruction::MemoryClose { .. } => "MemoryClose",
            lighthausInstruction::AssertAccountData { .. } => "AssertAccountData",
            lighthausInstruction::AssertAccountDelta { .. } => "AssertAccountDelta",
            lighthausInstruction::AssertAccountInfo { .. } => "AssertAccountInfo",
            lighthausInstruction::AssertAccountInfoMulti { .. } => "AssertAccountInfoMulti",
            lighthausInstruction::AssertMintAccount { .. } => "AssertMintAccount",
            lighthausInstruction::AssertMintAccountMulti { .. } => "AssertMintAccountMulti",
            lighthausInstruction::AssertTokenAccount { .. } => "AssertTokenAccount",
            lighthausInstruction::AssertTokenAccountMulti { .. } => "AssertTokenAccountMulti",
            lighthausInstruction::AssertStakeAccount { .. } => "AssertStakeAccount",
            lighthausInstruction::AssertStakeAccountMulti { .. } => "AssertStakeAccountMulti",
            lighthausInstruction::AssertUpgradeableLoaderAccount { .. } => {
                "AssertUpgradeableLoaderAccount"
            }
            lighthausInstruction::AssertUpgradeableLoaderAccountMulti { .. } => {
                "AssertUpgradeableLoaderAccountMulti"
            }
            lighthausInstruction::AssertSysvarClock { .. } => "AssertSysvarClock",
            lighthausInstruction::AssertBubblegumTreeConfigAccount { .. } => {
                "AssertBubblegumTreeConfigAccount"
            }
            lighthausInstruction::AssertMerkleTreeAccount { .. } => "AssertMerkleTreeAccount",
        }
    }

    pub fn get_log_level(&self) -> LogLevel {
        match self {
            lighthausInstruction::MemoryWrite { .. } => LogLevel::Silent,
            lighthausInstruction::MemoryClose { .. } => LogLevel::Silent,
            lighthausInstruction::AssertAccountData { log_level, .. } => *log_level,
            lighthausInstruction::AssertAccountDelta { log_level, .. } => *log_level,
            lighthausInstruction::AssertAccountInfo { log_level, .. } => *log_level,
            lighthausInstruction::AssertAccountInfoMulti { log_level, .. } => *log_level,
            lighthausInstruction::AssertMintAccount { log_level, .. } => *log_level,
            lighthausInstruction::AssertMintAccountMulti { log_level, .. } => *log_level,
            lighthausInstruction::AssertTokenAccount { log_level, .. } => *log_level,
            lighthausInstruction::AssertTokenAccountMulti { log_level, .. } => *log_level,
            lighthausInstruction::AssertStakeAccount { log_level, .. } => *log_level,
            lighthausInstruction::AssertStakeAccountMulti { log_level, .. } => *log_level,
            lighthausInstruction::AssertUpgradeableLoaderAccount { log_level, .. } => *log_level,
            lighthausInstruction::AssertUpgradeableLoaderAccountMulti { log_level, .. } => {
                *log_level
            }
            lighthausInstruction::AssertBubblegumTreeConfigAccount { log_level, .. } => *log_level,
            lighthausInstruction::AssertSysvarClock { log_level, .. } => *log_level,
            lighthausInstruction::AssertMerkleTreeAccount { log_level, .. } => *log_level,
        }
    }
}
