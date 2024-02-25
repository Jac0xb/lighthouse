use anchor_spl::token_interface::spl_token_2022::{self};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    bpf_loader, config, pubkey::Pubkey, rent::Rent, stake, system_program, sysvar::SysvarId, vote,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum KnownProgram {
    System,
    Token,
    Token2022,
    Rent,
    Stake,
    Vote,
    BpfLoader,
    SysvarConfig,
}

impl KnownProgram {
    pub fn to_pubkey(&self) -> Pubkey {
        match self {
            KnownProgram::System => system_program::id(),
            KnownProgram::Token => spl_token::id(),
            KnownProgram::Token2022 => spl_token_2022::id(),
            KnownProgram::Rent => Rent::id(),
            KnownProgram::Stake => stake::program::id(),
            KnownProgram::Vote => vote::program::id(),
            KnownProgram::BpfLoader => bpf_loader::id(),
            KnownProgram::SysvarConfig => config::program::id(),
        }
    }
}
