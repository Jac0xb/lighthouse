use crate::{
    error::LighthouseError,
    types::{ComparableOperator, DataValue, EquatableOperator, EvaluationResult, Operator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{msg, pubkey::Pubkey};
use std::cell::Ref;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataValueAssertion {
    Bool(bool, EquatableOperator),
    U8(u8, ComparableOperator),
    I8(i8, ComparableOperator),
    U16(u16, ComparableOperator),
    I16(i16, ComparableOperator),
    U32(u32, ComparableOperator),
    I32(i32, ComparableOperator),
    U64(u64, ComparableOperator),
    I64(i64, ComparableOperator),
    U128(u128, ComparableOperator),
    I128(i128, ComparableOperator),
    Bytes(Vec<u8>, EquatableOperator),
    Pubkey(Pubkey, EquatableOperator),
}

impl DataValueAssertion {
    pub fn size(&self) -> usize {
        match self {
            DataValueAssertion::Bool(_, _) => 1,
            DataValueAssertion::U8(_, _) => 1,
            DataValueAssertion::I8(_, _) => 1,
            DataValueAssertion::U16(_, _) => 2,
            DataValueAssertion::I16(_, _) => 2,
            DataValueAssertion::U32(_, _) => 4,
            DataValueAssertion::I32(_, _) => 4,
            DataValueAssertion::U64(_, _) => 8,
            DataValueAssertion::I64(_, _) => 8,
            DataValueAssertion::U128(_, _) => 16,
            DataValueAssertion::I128(_, _) => 16,
            DataValueAssertion::Bytes(value, _) => value.len(),
            DataValueAssertion::Pubkey(_, _) => 32,
        }
    }

    pub fn deserialize(&self, bytes: &[u8]) -> Result<DataValue> {
        match self {
            DataValueAssertion::Bool(_, _) => {
                let len = bytes.len();
                if len != 1 {
                    msg!("Boolean data value must be 1 byte long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::Bool(bytes[0] != 0))
                }
            }
            DataValueAssertion::U8(_, _) => {
                let len = bytes.len();
                if len != 1 {
                    msg!("U8 data value must be 1 byte long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::U8(u8::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::I8(_, _) => {
                let len = bytes.len();
                if len != 1 {
                    msg!("I8 data value must be 1 byte long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::I8(i8::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::U16(_, _) => {
                let len = bytes.len();
                if len != 2 {
                    msg!("U16 data value must be 2 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::U16(u16::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::I16(_, _) => {
                let len = bytes.len();
                if len != 2 {
                    msg!("I16 data value must be 2 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::I16(i16::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::U32(_, _) => {
                let len = bytes.len();
                if len != 4 {
                    msg!("U32 data value must be 4 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::U32(u32::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::I32(_, _) => {
                let len = bytes.len();
                if len != 4 {
                    msg!("I32 data value must be 4 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::I32(i32::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::U64(_, _) => {
                let len = bytes.len();
                if len != 8 {
                    msg!("U64 data value must be 8 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::U64(u64::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::I64(_, _) => {
                let len = bytes.len();
                if len != 8 {
                    msg!("I64 data value must be 8 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::I64(i64::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::U128(_, _) => {
                let len = bytes.len();
                if len != 16 {
                    msg!("U128 data value must be 16 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::U128(u128::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::I128(_, _) => {
                let len = bytes.len();
                if len != 16 {
                    msg!("I128 data value must be 16 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::I128(i128::from_le_bytes(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
            DataValueAssertion::Bytes(_, _) => Ok(DataValue::Bytes(bytes.to_vec())),
            DataValueAssertion::Pubkey(_, _) => {
                let len = bytes.len();
                if len != 32 {
                    msg!("Pubkey data value must be 32 bytes long");
                    Err(LighthouseError::InvalidDataLength.into())
                } else {
                    Ok(DataValue::Pubkey(Pubkey::new_from_array(
                        bytes
                            .try_into()
                            .map_err(|_| LighthouseError::InvalidDataLength)?,
                    )))
                }
            }
        }
    }

    pub fn evaluate_from_data_slice(
        &self,
        data: Ref<'_, &mut [u8]>,
        offset: usize,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        let slice = data
            .get(offset..(offset + self.size()))
            .ok_or(LighthouseError::OutOfRange)?;

        let value = DataValueAssertion::deserialize(self, slice)?;

        match self {
            DataValueAssertion::Bool(expected_value, operator) => {
                let value = match value {
                    DataValue::Bool(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::U8(expected_value, operator) => {
                let value = match value {
                    DataValue::U8(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::I8(expected_value, operator) => {
                let value = match value {
                    DataValue::I8(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::U16(expected_value, operator) => {
                let value = match value {
                    DataValue::U16(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::I16(expected_value, operator) => {
                let value = match value {
                    DataValue::I16(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::U32(expected_value, operator) => {
                let value = match value {
                    DataValue::U32(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::I32(expected_value, operator) => {
                let value = match value {
                    DataValue::I32(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::U64(expected_value, operator) => {
                let value = match value {
                    DataValue::U64(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::I64(expected_value, operator) => {
                let value = match value {
                    DataValue::I64(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::U128(expected_value, operator) => {
                let value = match value {
                    DataValue::U128(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::I128(expected_value, operator) => {
                let value = match value {
                    DataValue::I128(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::Bytes(expected_value, operator) => {
                let value = match value {
                    DataValue::Bytes(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
            DataValueAssertion::Pubkey(expected_value, operator) => {
                let value = match value {
                    DataValue::Pubkey(value) => value,
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                Ok(operator.evaluate(&value, expected_value, include_output))
            }
        }
    }
}
