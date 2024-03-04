use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountInfoData {
    pub key: Pubkey,
    pub lamports: u64,
    pub data_length: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub executable: bool,
}
