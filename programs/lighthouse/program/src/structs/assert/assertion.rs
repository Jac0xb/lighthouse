use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use solana_program::pubkey::Pubkey;

use crate::structs::{operator::Operator, AccountInfoDataField, DataValue};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum Assertion {
    // memory offset, assertion
    Memory(u16, Operator, DataValue),

    AccountInfo(Vec<AccountInfoDataField>, Operator),

    // account data offset, borsh type, operator
    AccountData(u16, Operator, DataValue),

    // balance, operator
    AccountBalance(u64, Operator),

    AccountOwnedBy(Pubkey, Operator),

    // token balance, operator
    TokenAccountBalance(u64, Operator),
    // TODO
    // IsSigner,
}

impl Assertion {
    pub fn format(&self) -> String {
        match self {
            Assertion::Memory(offset, operator, value) => {
                format!(
                    "Memory[{}] {} {}",
                    offset,
                    operator.format(),
                    value.format()
                )
            }
            Assertion::AccountData(offset, operator, value) => {
                format!(
                    "AccountData[{}] {} {}",
                    offset,
                    operator.format(),
                    value.format()
                )
            }
            Assertion::AccountBalance(balance, operator) => {
                format!("AccountBalance {} {}", balance, operator.format())
            }
            Assertion::AccountOwnedBy(pubkey, operator) => {
                format!("AccountOwnedBy {} {}", pubkey, operator.format())
            }
            Assertion::TokenAccountBalance(balance, operator) => {
                format!("TokenAccountBalance {} {}", balance, operator.format())
            }
            Assertion::AccountInfo(fields, operator) => {
                format!("AccountInfo {:?} {}", fields, operator.format())
            }
        }
    }
}
