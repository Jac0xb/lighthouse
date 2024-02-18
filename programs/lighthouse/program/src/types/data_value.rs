use super::{
    operator::{EvaluationResult, Operator},
    ComparableOperator, EquatableOperator,
};
use crate::{error::LighthouseError, utils::Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{msg, pubkey::Pubkey};
use std::cell::Ref;

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
