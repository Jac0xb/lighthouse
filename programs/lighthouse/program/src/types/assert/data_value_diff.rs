use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, msg};

use crate::{
    error::LighthouseError,
    types::{Assert, DataValue, EvaluationResult, IntegerOperator, Operator},
    utils::Result,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountDataDiffAssertion {
    pub offset_left: u16,
    pub offset_right: u16,
    pub assertion: DataValueDiffAssertion,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataValueDiffAssertion {
    U8(i16, IntegerOperator),
    I8(i16, IntegerOperator),
    U16(i32, IntegerOperator),
    I16(i32, IntegerOperator),
    U32(i64, IntegerOperator),
    I32(i64, IntegerOperator),
    U64(i128, IntegerOperator),
    I64(i128, IntegerOperator),
}

impl DataValueDiffAssertion {
    pub fn size(&self) -> usize {
        match self {
            DataValueDiffAssertion::U8(_, _) => 1,
            DataValueDiffAssertion::I8(_, _) => 1,
            DataValueDiffAssertion::U16(_, _) => 2,
            DataValueDiffAssertion::I16(_, _) => 2,
            DataValueDiffAssertion::U32(_, _) => 4,
            DataValueDiffAssertion::I32(_, _) => 4,
            DataValueDiffAssertion::U64(_, _) => 8,
            DataValueDiffAssertion::I64(_, _) => 8,
        }
    }

    pub fn deserialize(&self, bytes: &[u8]) -> Result<DataValue> {
        match self {
            DataValueDiffAssertion::U8(_, _) => {
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
            DataValueDiffAssertion::I8(_, _) => {
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
            DataValueDiffAssertion::U16(_, _) => {
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
            DataValueDiffAssertion::I16(_, _) => {
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
            DataValueDiffAssertion::U32(_, _) => {
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
            DataValueDiffAssertion::I32(_, _) => {
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
            DataValueDiffAssertion::U64(_, _) => {
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
            DataValueDiffAssertion::I64(_, _) => {
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
        }
    }
}

impl Assert<(AccountInfo<'_>, AccountInfo<'_>)> for AccountDataDiffAssertion {
    fn format(&self) -> String {
        format!(
            "AccountData[{}, {}, {}]",
            self.offset_left,
            self.offset_right,
            self.assertion.format()
        )
    }

    fn evaluate(
        &self,
        accounts: &(AccountInfo, AccountInfo),
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        let right_offset = self.offset_right as usize;
        let assertion = &self.assertion;

        let (left_account, right_account) = accounts;

        let left_offset = self.offset_left as usize;
        let left_data = left_account.try_borrow_data()?;
        let left_slice = left_data
            .get(left_offset..(left_offset + assertion.size()))
            .ok_or(LighthouseError::OutOfRange)?;
        let left_value = DataValueDiffAssertion::deserialize(assertion, left_slice)?;

        let right_data = right_account.try_borrow_data()?;
        let right_slice = right_data
            .get(right_offset..(right_offset + assertion.size()))
            .ok_or(LighthouseError::OutOfRange)?;
        let right_value = DataValueDiffAssertion::deserialize(assertion, right_slice)?;

        match assertion {
            DataValueDiffAssertion::U8(expected_diff_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::U8(left_value), DataValue::U8(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i16 - right_value as i16;

                Ok(operator.evaluate(&diff_value, expected_diff_value, include_output))
            }
            DataValueDiffAssertion::I8(expected_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::I8(left_value), DataValue::I8(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i16 - right_value as i16;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::U16(expected_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::U16(left_value), DataValue::U16(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i32 - right_value as i32;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::I16(expected_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::I16(left_value), DataValue::I16(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i32 - right_value as i32;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::U32(expected_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::U32(left_value), DataValue::U32(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i64 - right_value as i64;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::I32(expected_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::I32(left_value), DataValue::I32(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i64 - right_value as i64;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::U64(expected_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::U64(left_value), DataValue::U64(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i128 - right_value as i128;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::I64(expected_value, operator) => {
                let (left_value, right_value) = match (left_value, right_value) {
                    (DataValue::I64(left_value), DataValue::I64(right_value)) => {
                        (left_value, right_value)
                    }
                    _ => return Err(LighthouseError::DataValueMismatch.into()),
                };

                let diff_value = left_value as i128 - right_value as i128;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
        }
    }
}

impl DataValueDiffAssertion {
    pub fn format(&self) -> String {
        match self {
            DataValueDiffAssertion::U8(value, operator) => format!("U8[{}, {:?}]", value, operator),
            DataValueDiffAssertion::I8(value, operator) => format!("I8[{}, {:?}]", value, operator),
            DataValueDiffAssertion::U16(value, operator) => {
                format!("U16[{}, {:?}]", value, operator)
            }
            DataValueDiffAssertion::I16(value, operator) => {
                format!("I16[{}, {:?}]", value, operator)
            }
            DataValueDiffAssertion::U32(value, operator) => {
                format!("U32[{}, {:?}]", value, operator)
            }
            DataValueDiffAssertion::I32(value, operator) => {
                format!("I32[{}, {:?}]", value, operator)
            }
            DataValueDiffAssertion::U64(value, operator) => {
                format!("U64[{}, {:?}]", value, operator)
            }
            DataValueDiffAssertion::I64(value, operator) => {
                format!("I64[{}, {:?}]", value, operator)
            }
        }
    }
}
