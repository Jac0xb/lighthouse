use super::{Assert, LogLevel};
use crate::{
    err, err_msg,
    error::LighthouseError,
    types::operator::{EvaluationResult, IntegerOperator, Operator},
    utils::{try_from_slice, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountDataDeltaAssertion {
    pub offset_left: u16,
    pub offset_right: u16,
    pub assertion: DataValueDeltaAssertion,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataValueDeltaAssertion {
    U8 {
        value: i16,
        operator: IntegerOperator,
    },
    I8 {
        value: i16,
        operator: IntegerOperator,
    },
    U16 {
        value: i32,
        operator: IntegerOperator,
    },
    I16 {
        value: i32,
        operator: IntegerOperator,
    },
    U32 {
        value: i64,
        operator: IntegerOperator,
    },
    I32 {
        value: i64,
        operator: IntegerOperator,
    },
    U64 {
        value: i128,
        operator: IntegerOperator,
    },
    I64 {
        value: i128,
        operator: IntegerOperator,
    },
}

impl Assert<(AccountInfo<'_>, AccountInfo<'_>)> for AccountDataDeltaAssertion {
    fn evaluate(
        &self,
        accounts: &(AccountInfo, AccountInfo),
        log_level: &LogLevel,
    ) -> Result<Box<EvaluationResult>> {
        let left_offset = self.offset_left as usize;
        let right_offset = self.offset_right as usize;
        let assertion = &self.assertion;

        let (left_account, right_account) = accounts;

        let left_account_data = left_account.try_borrow_data().map_err(|e| {
            err_msg!("Cannot borrow data for left target account", e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;
        let right_account_data = right_account.try_borrow_data().map_err(|e| {
            err_msg!("Cannot borrow data for right target account", e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        match assertion {
            DataValueDeltaAssertion::U8 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<u8>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<u8>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i16 - right_value as i16;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
            }
            DataValueDeltaAssertion::I8 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<i8>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<i8>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i16 - right_value as i16;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
            }
            DataValueDeltaAssertion::U16 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<u16>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<u16>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i32 - right_value as i32;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
            }
            DataValueDeltaAssertion::I16 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<i16>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<i16>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i32 - right_value as i32;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
            }
            DataValueDeltaAssertion::U32 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<u32>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<u32>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i64 - right_value as i64;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
            }
            DataValueDeltaAssertion::I32 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<i32>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<i32>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i64 - right_value as i64;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
            }
            DataValueDeltaAssertion::U64 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<u64>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<u64>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i128 - right_value as i128;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
            }
            DataValueDeltaAssertion::I64 {
                value: assertion_value,
                operator,
            } => {
                let left_value = try_from_slice::<i64>(&left_account_data, left_offset, None)?;
                let right_value = try_from_slice::<i64>(&right_account_data, right_offset, None)?;

                let diff_value = left_value as i128 - right_value as i128;

                Ok(operator.evaluate(&diff_value, assertion_value, log_level))
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
            types::assert::{
                account_data_delta::{AccountDataDeltaAssertion, DataValueDeltaAssertion},
                Assert, LogLevel,
            },
            types::operator::IntegerOperator,
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

            let assertion = AccountDataDeltaAssertion {
                offset_left: 0,
                offset_right: 0,
                assertion: DataValueDeltaAssertion::U8 {
                    value: 1i16 - (u8::MAX as i16),
                    operator: IntegerOperator::Equal,
                },
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    &LogLevel::PlaintextMsgLog,
                )
                .unwrap();

            assert!(result.passed);

            let reverse_assertion = AccountDataDeltaAssertion {
                offset_left: 0,
                offset_right: 0,
                assertion: DataValueDeltaAssertion::U8 {
                    value: (u8::MAX as i16) - 1i16,
                    operator: IntegerOperator::Equal,
                },
            };

            let result = reverse_assertion
                .evaluate(
                    &(right_account_info, left_account_info),
                    &LogLevel::PlaintextMsgLog,
                )
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

            let assertion = AccountDataDeltaAssertion {
                offset_left: 1,
                offset_right: 0,
                assertion: DataValueDeltaAssertion::I8 {
                    value: (test_account.i8 as i16) - (i8::MIN as i16),
                    operator: IntegerOperator::Equal,
                },
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    &LogLevel::PlaintextMsgLog,
                )
                .unwrap();

            msg!("{:?}", result.output);

            assert!(result.passed);

            let reverse_assertion = AccountDataDeltaAssertion {
                offset_left: 0,
                offset_right: 1,
                assertion: DataValueDeltaAssertion::I8 {
                    value: (i8::MIN as i16) - (test_account.i8 as i16),
                    operator: IntegerOperator::Equal,
                },
            };

            let result = reverse_assertion
                .evaluate(
                    &(right_account_info, left_account_info),
                    &LogLevel::PlaintextMsgLog,
                )
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

            let assertion = AccountDataDeltaAssertion {
                offset_left: 2,
                offset_right: 0,
                assertion: DataValueDeltaAssertion::U16 {
                    value: (test_account.u16 as i32) - (u16::MAX as i32),
                    operator: IntegerOperator::Equal,
                },
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    &LogLevel::PlaintextMsgLog,
                )
                .unwrap();

            msg!("{:?}", result.output);

            assert!(result.passed);

            let reverse_assertion = AccountDataDeltaAssertion {
                offset_left: 0,
                offset_right: 2,
                assertion: DataValueDeltaAssertion::U16 {
                    value: (u16::MAX as i32) - (test_account.u16 as i32),
                    operator: IntegerOperator::Equal,
                },
            };

            let result = reverse_assertion
                .evaluate(
                    &(right_account_info, left_account_info),
                    &LogLevel::PlaintextMsgLog,
                )
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

            let assertion = AccountDataDeltaAssertion {
                offset_left: 4,
                offset_right: 0,
                assertion: DataValueDeltaAssertion::I16 {
                    value: (test_account.i16 as i32) - (i16::MIN as i32) - 10,
                    operator: IntegerOperator::GreaterThan,
                },
            };

            let result = assertion
                .evaluate(
                    &(left_account_info.clone(), right_account_info.clone()),
                    &LogLevel::PlaintextMsgLog,
                )
                .unwrap();

            msg!("{:?}", result.output);

            assert!(result.passed);

            let reverse_assertion = AccountDataDeltaAssertion {
                offset_left: 0,
                offset_right: 4,
                assertion: DataValueDeltaAssertion::I16 {
                    value: (i16::MIN as i32) - (test_account.i16 as i32) + 10,
                    operator: IntegerOperator::LessThan,
                },
            };

            let result = reverse_assertion
                .evaluate(
                    &(right_account_info, left_account_info),
                    &LogLevel::PlaintextMsgLog,
                )
                .unwrap();

            assert!(result.passed);
        }
    }
}
