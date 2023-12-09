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

// TODO: check data borsh size of this struct
// Created the optionze macro but intellisense sucks
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct OptionalAccountInfoData {
    pub key: Option<Pubkey>,
    pub lamports: Option<u64>,
    pub data_length: Option<u64>,
    pub owner: Option<Pubkey>,
    pub rent_epoch: Option<u64>,
    pub is_signer: Option<bool>,
    pub is_writable: Option<bool>,
    pub executable: Option<bool>,
}
