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
    #[account(0, name = "program_id", desc = "Lighthouse program")]
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

    #[account(0, name = "program_id", desc = "Lighthouse program")]
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
}

impl LighthouseInstruction {
    pub const fn get_name(&self) -> &'static str {
        match self {
            LighthouseInstruction::MemoryWrite { .. } => "MemoryWrite",
            LighthouseInstruction::MemoryClose { .. } => "MemoryClose",
            LighthouseInstruction::AssertAccountData { .. } => "AssertAccountData",
            LighthouseInstruction::AssertAccountDelta { .. } => "AssertAccountDelta",
            LighthouseInstruction::AssertAccountInfo { .. } => "AssertAccountInfo",
            LighthouseInstruction::AssertAccountInfoMulti { .. } => "AssertAccountInfoMulti",
            LighthouseInstruction::AssertMintAccount { .. } => "AssertMintAccount",
            LighthouseInstruction::AssertMintAccountMulti { .. } => "AssertMintAccountMulti",
            LighthouseInstruction::AssertTokenAccount { .. } => "AssertTokenAccount",
            LighthouseInstruction::AssertTokenAccountMulti { .. } => "AssertTokenAccountMulti",
            LighthouseInstruction::AssertStakeAccount { .. } => "AssertStakeAccount",
            LighthouseInstruction::AssertStakeAccountMulti { .. } => "AssertStakeAccountMulti",
            LighthouseInstruction::AssertUpgradeableLoaderAccount { .. } => {
                "AssertUpgradeableLoaderAccount"
            }
            LighthouseInstruction::AssertUpgradeableLoaderAccountMulti { .. } => {
                "AssertUpgradeableLoaderAccountMulti"
            }
            LighthouseInstruction::AssertSysvarClock { .. } => "AssertSysvarClock",
            LighthouseInstruction::AssertMerkleTreeAccount { .. } => "AssertMerkleTreeAccount",
        }
    }

    pub fn get_log_level(&self) -> LogLevel {
        match self {
            LighthouseInstruction::MemoryWrite { .. } => LogLevel::Silent,
            LighthouseInstruction::MemoryClose { .. } => LogLevel::Silent,
            LighthouseInstruction::AssertAccountData { log_level, .. } => *log_level,
            LighthouseInstruction::AssertAccountDelta { log_level, .. } => *log_level,
            LighthouseInstruction::AssertAccountInfo { log_level, .. } => *log_level,
            LighthouseInstruction::AssertAccountInfoMulti { log_level, .. } => *log_level,
            LighthouseInstruction::AssertMintAccount { log_level, .. } => *log_level,
            LighthouseInstruction::AssertMintAccountMulti { log_level, .. } => *log_level,
            LighthouseInstruction::AssertTokenAccount { log_level, .. } => *log_level,
            LighthouseInstruction::AssertTokenAccountMulti { log_level, .. } => *log_level,
            LighthouseInstruction::AssertStakeAccount { log_level, .. } => *log_level,
            LighthouseInstruction::AssertStakeAccountMulti { log_level, .. } => *log_level,
            LighthouseInstruction::AssertUpgradeableLoaderAccount { log_level, .. } => *log_level,
            LighthouseInstruction::AssertUpgradeableLoaderAccountMulti { log_level, .. } => {
                *log_level
            }
            LighthouseInstruction::AssertSysvarClock { log_level, .. } => *log_level,
            LighthouseInstruction::AssertMerkleTreeAccount { log_level, .. } => *log_level,
        }
    }
}
