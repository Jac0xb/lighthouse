use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    bpf_loader, compute_budget, config, pubkey::Pubkey, rent::Rent, stake, system_program,
    sysvar::SysvarId, vote,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum KnownProgram {
    System,
    Token,
    Rent,
    Stake,
    Vote,
    BpfLoader,
    SysvarConfig,
    ComputeBudget,
}

impl KnownProgram {
    pub fn to_pubkey(&self) -> Pubkey {
        match self {
            KnownProgram::System => system_program::id(),
            KnownProgram::Token => spl_token::id(),
            KnownProgram::Rent => Rent::id(),
            KnownProgram::Stake => stake::program::id(),
            KnownProgram::Vote => vote::program::id(),
            KnownProgram::BpfLoader => bpf_loader::id(),
            KnownProgram::SysvarConfig => config::program::id(),
            KnownProgram::ComputeBudget => compute_budget::id(),
        }
    }
}
