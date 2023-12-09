use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use solana_program::pubkey::Pubkey;

use super::{DataValue, Operator, OptionalAccountInfoData};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum Assertion {
    // memory offset, assertion
    Memory(u16, Operator, DataValue),

    // account data offset, borsh type, operator
    AccountData(u16, Operator, DataValue),

    // balance, operator
    AccountBalance(u64, Operator),

    AccountOwnedBy(Pubkey),

    // token balance, operator
    TokenAccountBalance(u64, Operator),

    AccountInfo(OptionalAccountInfoData),
}
