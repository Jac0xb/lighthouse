use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use solana_program::pubkey::Pubkey;

use crate::structs::{
    operator::Operator, AccountInfoDataField, DataValue, LegacyTokenAccountDataField,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum Assertion {
    AccountInfoField(AccountInfoDataField, Operator),

    // account data offset, borsh type, operator
    AccountData(u16, Operator, DataValue),

    // balance, operator
    AccountBalance(u64, Operator),

    AccountOwnedBy(Pubkey, Operator),

    // token balance, operator
    LegacyTokenAccountField(LegacyTokenAccountDataField, Operator),
}

impl Assertion {
    pub fn format(&self) -> String {
        match self {
            Assertion::AccountData(offset, operator, value) => {
                format!("AccountData[{}|{:?}|{:?}]", offset, operator, value)
            }
            Assertion::AccountBalance(balance, operator) => {
                format!("AccountBalance[{}|{:?}]", balance, operator)
            }
            Assertion::AccountOwnedBy(pubkey, operator) => {
                format!("AccountOwnedBy[{}|{:?}]", pubkey, operator)
            }
            Assertion::LegacyTokenAccountField(field, operator) => {
                format!("LegacyTokenAccountField[{:?}|{:?}]", field, operator)
            }
            Assertion::AccountInfoField(fields, operator) => {
                format!("AccountInfoField[{:?}|{:?}]", fields, operator)
            }
        }
    }
}
