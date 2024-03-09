use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    bpf_loader, bpf_loader_upgradeable, config, pubkey::Pubkey, rent::Rent, stake, system_program,
    sysvar::SysvarId, vote,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
#[repr(u8)]
pub enum KnownProgram {
    System,
    Token,
    Token2022,
    Rent,
    Stake,
    Vote,
    BpfLoader,
    UpgradeableLoader,
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
            KnownProgram::UpgradeableLoader => bpf_loader_upgradeable::id(),
            KnownProgram::SysvarConfig => config::program::id(),
        }
    }
}
