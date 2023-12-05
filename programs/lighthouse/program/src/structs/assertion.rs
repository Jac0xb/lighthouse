use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use solana_program::pubkey::Pubkey;

use super::{AccountInfoData, BorshField, BorshValue, Operator, OptionalAccountInfoData};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum Assertion {
    // offset, borsh type, operator
    BorshAccountData(u64, BorshField, Operator, BorshValue),

    RawAccountData(u64, Operator, Vec<u8>),

    // balance, operator
    AccountBalance(u64, Operator),

    AccountExists,

    AccountOwnedBy(Pubkey),

    // token balance, operator
    TokenAccountBalance(u64, Operator),

    AccountInfo(OptionalAccountInfoData),
}
