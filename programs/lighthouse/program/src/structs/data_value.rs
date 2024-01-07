use std::cell::Ref;

use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use solana_program::pubkey::Pubkey;

use crate::error::LighthouseError;

use super::operator::{EvaluationResult, Operator};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataType {
    Bool,
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
    pub fn format(&self) -> String {
        match self {
            DataValue::Bool(value) => value.to_string(),
            DataValue::U8(value) => value.to_string(),
            DataValue::I8(value) => value.to_string(),
            DataValue::U16(value) => value.to_string(),
            DataValue::I16(value) => value.to_string(),
            DataValue::U32(value) => value.to_string(),
            DataValue::I32(value) => value.to_string(),
            DataValue::U64(value) => value.to_string(),
            DataValue::I64(value) => value.to_string(),
            DataValue::U128(value) => value.to_string(),
            DataValue::I128(value) => value.to_string(),
            DataValue::Bytes(value) => {
                let value_str = value
                    .iter()
                    .take(15)
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<String>>()
                    .join("");
                format!("0x{}", value_str)
            }
            DataValue::Pubkey(value) => value.to_string(),
        }
    }
}

impl DataValue {
    pub fn get_data_type(&self) -> DataType {
        match self {
            DataValue::Bool(_) => DataType::Bool,
            DataValue::U8(_) => DataType::U8,
            DataValue::I8(_) => DataType::I8,
            DataValue::U16(_) => DataType::U16,
            DataValue::I16(_) => DataType::I16,
            DataValue::U32(_) => DataType::U32,
            DataValue::I32(_) => DataType::I32,
            DataValue::U64(_) => DataType::U64,
            DataValue::I64(_) => DataType::I64,
            DataValue::U128(_) => DataType::U128,
            DataValue::I128(_) => DataType::I128,
            DataValue::Bytes(_) => DataType::Bytes,
            DataValue::Pubkey(_) => DataType::Pubkey,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            DataValue::Bool(_) => 1,
            DataValue::U8(_) => 1,
            DataValue::I8(_) => 1,
            DataValue::U16(_) => 2,
            DataValue::I16(_) => 2,
            DataValue::U32(_) => 4,
            DataValue::I32(_) => 4,
            DataValue::U64(_) => 8,
            DataValue::I64(_) => 8,
            DataValue::U128(_) => 16,
            DataValue::I128(_) => 16,
            DataValue::Bytes(value) => value.len(),
            DataValue::Pubkey(_) => 32,
        }
    }

    pub fn serialize(self) -> Vec<u8> {
        match self {
            DataValue::Bool(value) => vec![value as u8],
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
            DataType::Bool => {
                let len = bytes.len();
                if len != 1 {
                    panic!("Invalid bool length: {}", len);
                } else {
                    DataValue::Bool(bytes[0] != 0)
                }
            }
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

    pub fn evaluate_from_data_slice(
        &self,
        data: Ref<'_, &mut [u8]>,
        offset: usize,
        operator: &Operator,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>, LighthouseError> {
        let slice = &data[offset..(offset + self.size())];
        let value = DataValue::deserialize(self.get_data_type(), slice);

        match self {
            DataValue::Bool(expected_value) => {
                let value = match value {
                    DataValue::Bool(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::U8(expected_value) => {
                let value = match value {
                    DataValue::U8(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::I8(expected_value) => {
                let value = match value {
                    DataValue::I8(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::U16(expected_value) => {
                let value = match value {
                    DataValue::U16(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::I16(expected_value) => {
                let value = match value {
                    DataValue::I16(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::U32(expected_value) => {
                let value = match value {
                    DataValue::U32(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::I32(expected_value) => {
                let value = match value {
                    DataValue::I32(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::U64(expected_value) => {
                let value = match value {
                    DataValue::U64(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::I64(expected_value) => {
                let value = match value {
                    DataValue::I64(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::U128(expected_value) => {
                let value = match value {
                    DataValue::U128(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::I128(expected_value) => {
                let value = match value {
                    DataValue::I128(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::Bytes(expected_value) => {
                match operator {
                    Operator::Equal => {}
                    Operator::NotEqual => {}
                    _ => return Err(LighthouseError::DataValueMismatch),
                }

                let value = match value {
                    DataValue::Bytes(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValue::Pubkey(expected_value) => {
                match operator {
                    Operator::Equal => {}
                    Operator::NotEqual => {}
                    _ => return Err(LighthouseError::UnsupportedOperator),
                }

                let value = match value {
                    DataValue::Pubkey(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
        }
    }
}
