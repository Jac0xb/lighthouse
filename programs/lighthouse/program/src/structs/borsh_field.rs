use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};

use super::Operator;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum BorshField {
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

impl BorshField {
    pub fn is_supported_operator(&self, operator: &Operator) -> bool {
        match self {
            BorshField::U8 => true,
            BorshField::I8 => true,
            BorshField::U16 => true,
            BorshField::I16 => true,
            BorshField::U32 => true,
            BorshField::I32 => true,
            BorshField::U64 => true,
            BorshField::I64 => true,
            BorshField::U128 => true,
            BorshField::I128 => true,
            BorshField::Bytes(_) => matches!(operator, Operator::Equal | Operator::NotEqual),
        }
    }
}
