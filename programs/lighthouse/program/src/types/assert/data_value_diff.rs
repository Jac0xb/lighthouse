use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError};

use crate::{
    constants::CANNOT_BORROW_DATA_TARGET_ERROR_MSG,
    err,
    error::LighthouseError,
    types::{Assert, DataValue, EvaluationResult, IntegerOperator, Operator},
    utils::{try_from_slice, Result},
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
        let left_offset = self.offset_left as usize;
        let right_offset = self.offset_right as usize;
        let assertion = &self.assertion;

        let (left_account, right_account) = accounts;

        let left_account_data = left_account.try_borrow_data().map_err(|e| {
            msg!("{}: {}", CANNOT_BORROW_DATA_TARGET_ERROR_MSG, e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;
        let right_account_data = right_account.try_borrow_data().map_err(|e| {
            msg!("{}: {}", CANNOT_BORROW_DATA_TARGET_ERROR_MSG, e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        match assertion {
            DataValueDiffAssertion::U8(expected_diff_value, operator) => {
                let left_value = try_from_slice::<u8>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<u8>(&right_account_data, right_offset)?;

                let diff_value = left_value as i16 - right_value as i16;

                Ok(operator.evaluate(&diff_value, expected_diff_value, include_output))
            }
            DataValueDiffAssertion::I8(expected_value, operator) => {
                let left_value = try_from_slice::<i8>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<i8>(&right_account_data, right_offset)?;

                let diff_value = left_value as i16 - right_value as i16;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::U16(expected_value, operator) => {
                let left_value = try_from_slice::<u16>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<u16>(&right_account_data, right_offset)?;

                let diff_value = left_value as i32 - right_value as i32;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::I16(expected_value, operator) => {
                let left_value = try_from_slice::<i16>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<i16>(&right_account_data, right_offset)?;

                let diff_value = left_value as i32 - right_value as i32;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::U32(expected_value, operator) => {
                let left_value = try_from_slice::<u32>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<u32>(&right_account_data, right_offset)?;

                let diff_value = left_value as i64 - right_value as i64;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::I32(expected_value, operator) => {
                let left_value = try_from_slice::<i32>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<i32>(&right_account_data, right_offset)?;

                let diff_value = left_value as i64 - right_value as i64;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::U64(expected_value, operator) => {
                let left_value = try_from_slice::<u64>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<u64>(&right_account_data, right_offset)?;

                let diff_value = left_value as i128 - right_value as i128;

                Ok(operator.evaluate(&diff_value, expected_value, include_output))
            }
            DataValueDiffAssertion::I64(expected_value, operator) => {
                let left_value = try_from_slice::<i64>(&left_account_data, left_offset)?;
                let right_value = try_from_slice::<i64>(&right_account_data, right_offset)?;

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

#[cfg(test)]
mod tests {
    mod evaluate_from_data_slice {
        use solana_sdk::{account_info::AccountInfo, msg, system_program};

        use crate::{
            test_utils::create_test_account,
            types::{AccountDataDiffAssertion, Assert, DataValueDiffAssertion, IntegerOperator},
        };

        #[test]
        fn evaluate_diff_u8() {
            let key = system_program::id();
            let (lamports_l, lamports_r) = (&mut 0, &mut 0);
            let left_data: &mut [u8] = &mut [0u8; 171];
            left_data.copy_from_slice(create_test_account().try_to_vec_override().as_ref());
            let left_account_info =
                AccountInfo::new(&key, false, false, lamports_l, left_data, &key, false, 0);

            let right_data: &mut [u8] = &mut [0u8; 1];
            let right_u8 = u8::MAX;
            right_data.copy_from_slice(right_u8.to_le_bytes().as_ref());
            let right_account_info =
                AccountInfo::new(&key, false, false, lamports_r, right_data, &key, false, 0);

            let assertion = AccountDataDiffAssertion {
                offset_left: 0,
                offset_right: 0,
                assertion: DataValueDiffAssertion::U8(
                    1i16 - (u8::MAX as i16),
                    IntegerOperator::Equal,
                ),
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    true,
                )
                .unwrap();

            assert!(result.passed);

            let reverse_assertion = AccountDataDiffAssertion {
                offset_left: 0,
                offset_right: 0,
                assertion: DataValueDiffAssertion::U8(
                    (u8::MAX as i16) - 1i16,
                    IntegerOperator::Equal,
                ),
            };

            let result = reverse_assertion
                .evaluate(&(right_account_info, left_account_info), true)
                .unwrap();

            assert!(result.passed);
        }

        #[test]
        fn evaluate_diff_i8() {
            let key = system_program::id();
            let (lamports_l, lamports_r) = (&mut 0, &mut 0);
            let left_data: &mut [u8] = &mut [0u8; 171];
            let test_account = create_test_account();
            left_data.copy_from_slice(create_test_account().try_to_vec_override().as_ref());
            let left_account_info =
                AccountInfo::new(&key, false, false, lamports_l, left_data, &key, false, 0);

            let right_data: &mut [u8] = &mut [0u8; 1];
            let right_i8 = i8::MIN;
            right_data.copy_from_slice(right_i8.to_le_bytes().as_ref());
            let right_account_info =
                AccountInfo::new(&key, false, false, lamports_r, right_data, &key, false, 0);

            let assertion = AccountDataDiffAssertion {
                offset_left: 1,
                offset_right: 0,
                assertion: DataValueDiffAssertion::I8(
                    (test_account.i8 as i16) - (i8::MIN as i16),
                    IntegerOperator::Equal,
                ),
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    true,
                )
                .unwrap();

            msg!("{:?}", result.output);

            assert!(result.passed);

            let reverse_assertion = AccountDataDiffAssertion {
                offset_left: 0,
                offset_right: 1,
                assertion: DataValueDiffAssertion::I8(
                    (i8::MIN as i16) - (test_account.i8 as i16),
                    IntegerOperator::Equal,
                ),
            };

            let result = reverse_assertion
                .evaluate(&(right_account_info, left_account_info), true)
                .unwrap();

            assert!(result.passed);
        }

        #[test]
        fn evaluate_diff_u16() {
            let key = system_program::id();
            let (lamports_l, lamports_r) = (&mut 0, &mut 0);
            let left_data: &mut [u8] = &mut [0u8; 171];
            let test_account = create_test_account();
            left_data.copy_from_slice(test_account.try_to_vec_override().as_ref());
            let left_account_info =
                AccountInfo::new(&key, false, false, lamports_l, left_data, &key, false, 0);

            let right_data: &mut [u8] = &mut [0u8; 2];
            let right_u16 = u16::MAX;
            right_data.copy_from_slice(right_u16.to_le_bytes().as_ref());
            let right_account_info =
                AccountInfo::new(&key, false, false, lamports_r, right_data, &key, false, 0);

            let assertion = AccountDataDiffAssertion {
                offset_left: 2,
                offset_right: 0,
                assertion: DataValueDiffAssertion::U16(
                    (test_account.u16 as i32) - (u16::MAX as i32),
                    IntegerOperator::Equal,
                ),
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    true,
                )
                .unwrap();

            msg!("{:?}", result.output);

            assert!(result.passed);

            let reverse_assertion = AccountDataDiffAssertion {
                offset_left: 0,
                offset_right: 2,
                assertion: DataValueDiffAssertion::U16(
                    (u16::MAX as i32) - (test_account.u16 as i32),
                    IntegerOperator::Equal,
                ),
            };

            let result = reverse_assertion
                .evaluate(&(right_account_info, left_account_info), true)
                .unwrap();

            assert!(result.passed);
        }

        #[test]
        fn evaluate_diff_i16() {
            let key = system_program::id();
            let (lamports_l, lamports_r) = (&mut 0, &mut 0);
            let left_data: &mut [u8] = &mut [0u8; 171];
            let test_account = create_test_account();
            left_data.copy_from_slice(test_account.try_to_vec_override().as_ref());
            let left_account_info =
                AccountInfo::new(&key, false, false, lamports_l, left_data, &key, false, 0);

            let right_data: &mut [u8] = &mut [0u8; 2];
            let right_i16 = i16::MIN;
            right_data.copy_from_slice(right_i16.to_le_bytes().as_ref());
            let right_account_info =
                AccountInfo::new(&key, false, false, lamports_r, right_data, &key, false, 0);

            let assertion = AccountDataDiffAssertion {
                offset_left: 4,
                offset_right: 0,
                assertion: DataValueDiffAssertion::I16(
                    (test_account.i16 as i32) - (i16::MIN as i32) - 10,
                    IntegerOperator::GreaterThan,
                ),
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    true,
                )
                .unwrap();

            msg!("{:?}", result.output);

            assert!(result.passed);

            let reverse_assertion = AccountDataDiffAssertion {
                offset_left: 0,
                offset_right: 4,
                assertion: DataValueDiffAssertion::I16(
                    (i16::MIN as i32) - (test_account.i16 as i32) + 10,
                    IntegerOperator::LessThan,
                ),
            };

            let result = reverse_assertion
                .evaluate(&(right_account_info, left_account_info), true)
                .unwrap();

            assert!(result.passed);
        }
    }
}
