use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::types::{ComparableOperator, EquatableOperator};

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

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AccountInfoFieldAssertion {
    Key(Pubkey, EquatableOperator),
    Lamports(u64, ComparableOperator),
    DataLength(u64, ComparableOperator),
    Owner(Pubkey, EquatableOperator),
    RentEpoch(u64, ComparableOperator),
    IsSigner(bool, EquatableOperator),
    IsWritable(bool, ComparableOperator),
    Executable(bool, ComparableOperator),
}
