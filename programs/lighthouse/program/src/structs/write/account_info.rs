use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountInfoData {
    pub key: Pubkey,
    pub lamports: u64,
    pub data_length: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub is_signer: bool,
    pub is_writable: bool,
    pub executable: bool,
}

impl AccountInfoData {
    // length constant
    pub fn size() -> u64 {
        32 + 8 + 8 + 32 + 8 + 1 + 1 + 1
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AccountInfoDataField {
    Key(Pubkey),
    Lamports(u64),
    DataLength(u64),
    Owner(Pubkey),
    RentEpoch(u64),
    IsSigner(bool),
    IsWritable(bool),
    Executable(bool),
}
