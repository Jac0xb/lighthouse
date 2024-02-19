use crate::{
    error::LighthouseError,
    types::{Assert, ComparableOperator, DataValue, EquatableOperator, EvaluationResult, Operator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, msg, pubkey::Pubkey};
use std::cell::Ref;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountDataAssertion {
    pub offset: u16,
    pub assertion: DataValueAssertion,
}

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
}

impl Assert<AccountInfo<'_>> for AccountDataAssertion {
    fn format(&self) -> String {
        format!("AccountData[{}|{:?}]", self.offset, self.assertion)
    }

    fn evaluate(
        &self,
        account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        let offset = self.offset as usize;
        let assertion = &self.assertion;

        let data = account.try_borrow_data()?;
        let slice = data
            .get(offset..(offset + assertion.size()))
            .ok_or(LighthouseError::OutOfRange)?;

        let value = DataValueAssertion::deserialize(assertion, slice)?;

        match assertion {
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

// #[cfg(test)]
// mod tests {
//     mod evaluate_from_data_slice {
//         use blackhat::processor::TestAccountV1;
//         use std::{cell::RefCell, rc::Rc};

//         use crate::{
//             error::LighthouseError,
//             types::{DataValue, Operator},
//         };

//         #[test]
//         fn evaluate_bool() {
//             let data_src: &mut [u8] = &mut [0u8; 128];

//             data_src[16] = 1;

//             let data_ref: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_src));
//             let data_value_true = DataValue::Bool(true);

//             let operator_and_expected_result: Vec<(dyn Operator, Option<bool>)> = vec![
//                 (Operator::Equal, Some(true)),
//                 (Operator::NotEqual, Some(false)),
//                 (Operator::GreaterThan, None),
//                 (Operator::GreaterThanOrEqual, None),
//                 (Operator::LessThan, None),
//                 (Operator::LessThanOrEqual, None),
//                 (Operator::Exists, None),
//                 (Operator::DoesNotExist, None),
//             ];

//             for (operator, expected_result) in operator_and_expected_result {
//                 let result = data_value_true.evaluate_from_data_slice(
//                     data_ref.borrow(),
//                     16,
//                     &operator,
//                     true,
//                 );

//                 if let Some(expected_result) = expected_result {
//                     let result = result.unwrap();
//                     assert_eq!(result.passed, expected_result, "{:?}", result.output);
//                 } else {
//                     let error = result.err().unwrap();
//                     assert_eq!(error, LighthouseError::UnsupportedOperator.into())
//                 }
//             }

//             let data_value_false = DataValue::Bool(false);

//             let operator_and_expected_result: Vec<(Operator, Option<bool>)> = vec![
//                 (Operator::Equal, Some(true)),
//                 (Operator::NotEqual, Some(false)),
//                 (Operator::GreaterThan, None),
//                 (Operator::GreaterThanOrEqual, None),
//                 (Operator::LessThan, None),
//                 (Operator::LessThanOrEqual, None),
//                 (Operator::Exists, None),
//                 (Operator::DoesNotExist, None),
//             ];

//             for (operator, expected_result) in operator_and_expected_result {
//                 let result = data_value_false.evaluate_from_data_slice(
//                     data_ref.borrow(),
//                     17,
//                     &operator,
//                     true,
//                 );

//                 if let Some(expected_result) = expected_result {
//                     let result = result.unwrap();
//                     assert_eq!(
//                         result.passed, expected_result,
//                         "Output: {:?}",
//                         result.output
//                     );
//                 } else {
//                     let error = result.err().unwrap();
//                     assert_eq!(error, LighthouseError::UnsupportedOperator.into())
//                 }
//             }
//         }

//         #[test]
//         fn evaluate() {
//             let data_src: &mut [u8] = &mut [0u8; 139];

//             data_src.copy_from_slice(create_test_account().try_to_vec_override().as_ref());

//             let data_ref: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_src));

//             let operator_and_expected_result: Vec<(DataValue, Operator, Option<bool>)> = vec![
//                 (DataValue::U8(1), Operator::Equal, Some(true)),
//                 (DataValue::U8(0), Operator::NotEqual, Some(true)),
//                 (DataValue::U8(5), Operator::GreaterThan, Some(false)),
//                 (DataValue::U8(1), Operator::GreaterThanOrEqual, Some(true)),
//                 (DataValue::U8(0), Operator::LessThan, Some(false)),
//                 (DataValue::U8(5), Operator::LessThanOrEqual, Some(true)),
//                 (DataValue::U8(1), Operator::Exists, None),
//                 (DataValue::U8(1), Operator::DoesNotExist, None),
//                 (DataValue::I8(-1), Operator::Equal, Some(true)),
//                 (DataValue::I8(0), Operator::NotEqual, Some(true)),
//                 (DataValue::I8(-5), Operator::GreaterThan, Some(true)),
//                 (DataValue::I8(-1), Operator::GreaterThanOrEqual, Some(true)),
//                 (DataValue::I8(0), Operator::LessThan, Some(true)),
//                 (DataValue::I8(-5), Operator::LessThanOrEqual, Some(false)),
//                 (DataValue::I8(-1), Operator::Exists, None),
//                 (DataValue::I8(-1), Operator::DoesNotExist, None),
//                 (DataValue::U16(256), Operator::Equal, Some(true)),
//                 (DataValue::U16(0), Operator::NotEqual, Some(true)),
//                 (
//                     DataValue::U16(u8::MAX as u16 * 2),
//                     Operator::GreaterThan,
//                     Some(false),
//                 ),
//                 (
//                     DataValue::U16(256),
//                     Operator::GreaterThanOrEqual,
//                     Some(true),
//                 ),
//                 (DataValue::U16(0), Operator::LessThan, Some(false)),
//                 (DataValue::U16(260), Operator::LessThanOrEqual, Some(true)),
//                 (DataValue::U16(256), Operator::Exists, None),
//                 (DataValue::U16(256), Operator::DoesNotExist, None),
//             ];

//             for (data_value, operator, expected_result) in operator_and_expected_result {
//                 let offset: usize = match &data_value {
//                     DataValue::U8(_) => 0,
//                     DataValue::I8(_) => 1,
//                     DataValue::U16(_) => 2,
//                     DataValue::I16(_) => 4,
//                     _ => panic!("Unsupported data value"),
//                 };

//                 println!("{:?} {:?} {:?}", data_value, operator, expected_result);

//                 let result =
//                     data_value.evaluate_from_data_slice(data_ref.borrow(), offset, &operator, true);

//                 if let Some(expected_result) = expected_result {
//                     let result = result.unwrap();
//                     assert_eq!(result.passed, expected_result, "{:?}", result.output);
//                 } else {
//                     let error = result.err().unwrap();
//                     assert_eq!(error, LighthouseError::UnsupportedOperator.into())
//                 }
//             }
//         }

//         pub fn create_test_account() -> TestAccountV1 {
//             TestAccountV1 {
//                 u8: 1,
//                 i8: -1,
//                 u16: (u8::MAX as u16) + 1,
//                 i16: (i8::MIN as i16) - 1,
//                 u32: (u16::MAX as u32) + 1,
//                 i32: (i16::MIN as i32) - 1,
//                 u64: (u32::MAX as u64) + 1,
//                 i64: (i32::MIN as i64) - 1,
//                 u128: (u64::MAX as u128) + 1,
//                 i128: (i64::MIN as i128) - 1,
//                 bytes: [u8::MAX; 32],
//                 true_: true,
//                 false_: false,
//                 option_u8: Some(u8::MAX),
//                 option_u8_none: None,
//                 option_u16: Some(u16::MAX),
//                 option_u16_none: None,
//                 vec: vec![u8::MAX; 32],
//             }
//         }
//     }
// }
