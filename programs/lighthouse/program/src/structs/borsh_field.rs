use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};

use super::Operator;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum BorshType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    Bytes(Vec<u8>),
}

impl BorshType {
    pub fn is_supported_operator(&self, operator: &Operator) -> bool {
        match self {
            BorshType::U8 => true,
            BorshType::I8 => true,
            BorshType::U16 => true,
            BorshType::I16 => true,
            BorshType::U32 => true,
            BorshType::I32 => true,
            BorshType::U64 => true,
            BorshType::I64 => true,
            BorshType::U128 => true,
            BorshType::I128 => true,
            BorshType::Bytes(_) => matches!(operator, Operator::Equal | Operator::NotEqual),
        }
    }
}
