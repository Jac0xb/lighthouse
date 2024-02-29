use crate::utils::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataValue {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    Bytes(Vec<u8>),
    Pubkey(Pubkey),
}

impl DataValue {
    pub fn serialize(self) -> Result<Vec<u8>> {
        Ok(match self {
            DataValue::Bool(value) => vec![value as u8],
            DataValue::U8(value) => value.try_to_vec()?,
            DataValue::I8(value) => value.try_to_vec()?,
            DataValue::U16(value) => value.try_to_vec()?,
            DataValue::I16(value) => value.try_to_vec()?,
            DataValue::U32(value) => value.try_to_vec()?,
            DataValue::I32(value) => value.try_to_vec()?,
            DataValue::U64(value) => value.try_to_vec()?,
            DataValue::I64(value) => value.try_to_vec()?,
            DataValue::U128(value) => value.try_to_vec()?,
            DataValue::I128(value) => value.try_to_vec()?,
            DataValue::Bytes(value) => value,
            DataValue::Pubkey(value) => value.to_bytes().to_vec(),
        })
    }
}
