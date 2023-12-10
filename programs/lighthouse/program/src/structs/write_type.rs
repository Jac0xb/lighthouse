use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token,
    token::{Mint, TokenAccount},
};
use borsh::{BorshDeserialize, BorshSerialize};

use super::DataValue;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum WriteType {
    AccountBalance,

    // Account Data Offset, Data Length
    AccountData(u16, u16),
    AccountInfo,
    DataValue(DataValue),
    MintAccount,
    TokenAccountLegacy,
    TokenAccount2022,
    Program,
}

impl WriteType {
    pub fn size(&self) -> usize {
        match self {
            WriteType::AccountBalance => 8,
            WriteType::AccountData(_, len) => *len as usize,
            WriteType::AccountInfo => 8,
            WriteType::DataValue(memory_value) => match memory_value {
                DataValue::Bool(_) | DataValue::U8(_) | DataValue::I8(_) => 1,
                DataValue::U16(_) | DataValue::I16(_) => 2,
                DataValue::U32(_) | DataValue::I32(_) => 4,
                DataValue::U64(_) | DataValue::I64(_) => 8,
                DataValue::U128(_) | DataValue::I128(_) => 16,
                DataValue::Bytes(bytes) => bytes.len(),
                DataValue::Pubkey(_) => 32,
            },
            // TODO: It might just be better/make more sense to create variants for mints, token accounts and let them choose what to write in the accounts
            WriteType::MintAccount => Mint::LEN, // TODO: Test
            WriteType::TokenAccountLegacy => TokenAccount::LEN, // TODO: Test
            WriteType::TokenAccount2022 => usize::MAX, // TODO: Get actual size
            WriteType::Program => 8,             // TODO: Get actual size
        }
    }

    pub fn account_validation(&self, account: &AccountInfo<'_>) -> bool {
        match self {
            WriteType::AccountBalance => true,
            WriteType::AccountData(_, _) => true,
            WriteType::AccountInfo => true,
            WriteType::DataValue(_) => true,
            WriteType::MintAccount => {
                account.owner == &spl_token::id() && account.data_len() == Mint::LEN
            }
            WriteType::TokenAccountLegacy => {
                account.owner == &associated_token::ID && account.data_len() == TokenAccount::LEN
            }
            WriteType::TokenAccount2022 => {
                false // TODO: Support
            }
            _ => true,
        }
    }
}

// TODO: probably worth creating a macro that permeates all these size variants so
// sdk can optimize space. Need to make sure its smaller than 256 variants though
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum WriteTypeParameter {
    // Memory offset, write type
    WriteU8(u8, WriteType),
    WriteU16(u16, WriteType),
    WriteU32(u32, WriteType),
}
