use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use solana_program::pubkey::Pubkey;

use crate::error::ProgramError;

use super::operator::Operator;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataType {
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
    Bytes,
    Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataValue {
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
    pub fn serialize(self) -> Vec<u8> {
        match self {
            DataValue::U8(value) => value.to_le_bytes().to_vec(),
            DataValue::I8(value) => value.to_le_bytes().to_vec(),
            DataValue::U16(value) => value.to_le_bytes().to_vec(),
            DataValue::I16(value) => value.to_le_bytes().to_vec(),
            DataValue::U32(value) => value.to_le_bytes().to_vec(),
            DataValue::I32(value) => value.to_le_bytes().to_vec(),
            DataValue::U64(value) => value.to_le_bytes().to_vec(),
            DataValue::I64(value) => value.to_le_bytes().to_vec(),
            DataValue::U128(value) => value.to_le_bytes().to_vec(),
            DataValue::I128(value) => value.to_le_bytes().to_vec(),
            DataValue::Bytes(value) => value,
            DataValue::Pubkey(value) => value.to_bytes().to_vec(),
        }
    }
    pub fn deserialize(data_type: DataType, bytes: &[u8]) -> Self {
        match data_type {
            DataType::U8 => DataValue::U8(u8::from_le_bytes(bytes.try_into().unwrap())),
            DataType::I8 => DataValue::I8(i8::from_le_bytes(bytes.try_into().unwrap())),
            DataType::U16 => DataValue::U16(u16::from_le_bytes(bytes.try_into().unwrap())),
            DataType::I16 => DataValue::I16(i16::from_le_bytes(bytes.try_into().unwrap())),
            DataType::U32 => DataValue::U32(u32::from_le_bytes(bytes.try_into().unwrap())),
            DataType::I32 => DataValue::I32(i32::from_le_bytes(bytes.try_into().unwrap())),
            DataType::U64 => DataValue::U64(u64::from_le_bytes(bytes.try_into().unwrap())),
            DataType::I64 => DataValue::I64(i64::from_le_bytes(bytes.try_into().unwrap())),
            DataType::U128 => DataValue::U128(u128::from_le_bytes(bytes.try_into().unwrap())),
            DataType::I128 => DataValue::I128(i128::from_le_bytes(bytes.try_into().unwrap())),
            DataType::Bytes => DataValue::Bytes(bytes.to_vec()),
            DataType::Pubkey => {
                DataValue::Pubkey(Pubkey::new_from_array(bytes.try_into().unwrap()))
            }
        }
    }

    pub fn compare(&self, other: &Self, operator: Operator) -> bool {
        match (self, other) {
            (DataValue::U8(a), DataValue::U8(b)) => operator.is_true(a, b),
            (DataValue::I8(a), DataValue::I8(b)) => operator.is_true(a, b),
            (DataValue::U16(a), DataValue::U16(b)) => operator.is_true(a, b),
            (DataValue::I16(a), DataValue::I16(b)) => operator.is_true(a, b),
            (DataValue::U32(a), DataValue::U32(b)) => operator.is_true(a, b),
            (DataValue::I32(a), DataValue::I32(b)) => operator.is_true(a, b),
            (DataValue::U64(a), DataValue::U64(b)) => operator.is_true(a, b),
            (DataValue::I64(a), DataValue::I64(b)) => operator.is_true(a, b),
            (DataValue::U128(a), DataValue::U128(b)) => operator.is_true(a, b),
            (DataValue::I128(a), DataValue::I128(b)) => operator.is_true(a, b),
            (DataValue::Bytes(a), DataValue::Bytes(b)) => operator.is_true(a, b),
            (DataValue::Pubkey(a), DataValue::Pubkey(b)) => operator.is_true(a, b),
            (_, _) => false,
        }
    }

    pub fn deserialize_and_compare(
        self,
        data: &[u8],
        offset: usize,
        operator: &Operator,
    ) -> Result<(String, String, bool), ProgramError> {
        let mut value_str = String::new();
        let mut expected_value_str = String::new();
        let mut assertion_result = false;

        match self {
            DataValue::U8(expected_value) => {
                let slice = &data[offset as usize..(offset + 1) as usize];
                let value = DataValue::deserialize(DataType::U8, slice);

                let value = match value {
                    DataValue::U8(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = expected_value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::I8(expected_value) => {
                let slice = &data[offset as usize..(offset + 1) as usize];
                let value = DataValue::deserialize(DataType::I8, slice);

                let value = match value {
                    DataValue::I8(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::U16(expected_value) => {
                let slice = &data[offset as usize..(offset + 2) as usize];
                let value = DataValue::deserialize(DataType::U16, slice);

                let value = match value {
                    DataValue::U16(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::I16(expected_value) => {
                let slice = &data[offset as usize..(offset + 2) as usize];
                let value = DataValue::deserialize(DataType::I16, slice);

                let value = match value {
                    DataValue::I16(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = expected_value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::U32(expected_value) => {
                let slice = &data[offset as usize..(offset + 4) as usize];
                let value = DataValue::deserialize(DataType::U32, slice);

                let value = match value {
                    DataValue::U32(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::I32(expected_value) => {
                let slice = &data[offset as usize..(offset + 4) as usize];
                let value = DataValue::deserialize(DataType::I32, slice);

                let value = match value {
                    DataValue::I32(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::U64(expected_value) => {
                let slice = &data[offset as usize..(offset + 8) as usize];
                let value = DataValue::deserialize(DataType::U64, slice);

                let value = match value {
                    DataValue::U64(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::I64(expected_value) => {
                let slice = &data[offset as usize..(offset + 8) as usize];
                let value = DataValue::deserialize(DataType::I64, slice);

                let value = match value {
                    DataValue::I64(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::U128(expected_value) => {
                let slice = &data[offset as usize..(offset + 16) as usize];
                let value = DataValue::deserialize(DataType::U128, slice);

                let value = match value {
                    DataValue::U128(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = expected_value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::I128(expected_value) => {
                let slice = &data[offset as usize..(offset + 16) as usize];
                let value = DataValue::deserialize(DataType::I128, slice);

                let value = match value {
                    DataValue::I128(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                value_str = value.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::Bytes(expected_value) => {
                let slice: &[u8] = &data[offset as usize..(offset + expected_value.len() as usize)];
                let value = DataValue::deserialize(DataType::Bytes, slice);

                match operator {
                    Operator::Equal => {}
                    Operator::NotEqual => {}
                    _ => return Err(ProgramError::UnsupportedOperator.into()),
                }

                let value = match value {
                    DataValue::Bytes(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch.into()),
                };

                // print array
                value_str = value
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<String>>()
                    .join("");
                expected_value_str = expected_value
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<String>>()
                    .join("");
                assertion_result = operator.is_true(&value, &expected_value);
            }
            DataValue::Pubkey(expected_value) => {
                let slice = &data[offset as usize..(offset + 32) as usize];
                let value = DataValue::deserialize(DataType::Pubkey, slice);

                match operator {
                    Operator::Equal => {}
                    Operator::NotEqual => {}
                    _ => return Err(ProgramError::UnsupportedOperator),
                }

                let value = match value {
                    DataValue::Pubkey(value) => value,
                    _ => return Err(ProgramError::DataValueMismatch),
                };

                value_str = value_str.to_string();
                expected_value_str = expected_value.to_string();
                assertion_result = operator.is_true(&value, &expected_value);
            }
        }

        Ok((value_str, expected_value_str, assertion_result))
    }
}
