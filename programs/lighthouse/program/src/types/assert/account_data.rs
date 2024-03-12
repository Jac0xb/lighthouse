use super::{Assert, LogLevel};
use crate::{
    err, err_msg,
    error::LighthouseError,
    types::assert::operator::{ByteSliceOperator, EquatableOperator, IntegerOperator, Operator},
    utils::{try_from_slice, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, msg, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AccountDataAssertion {
    pub offset: u16,
    pub assertion: DataValueAssertion,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum DataValueAssertion {
    Bool {
        value: bool,
        operator: EquatableOperator,
    },
    U8 {
        value: u8,
        operator: IntegerOperator,
    },
    I8 {
        value: i8,
        operator: IntegerOperator,
    },
    U16 {
        value: u16,
        operator: IntegerOperator,
    },
    I16 {
        value: i16,
        operator: IntegerOperator,
    },
    U32 {
        value: u32,
        operator: IntegerOperator,
    },
    I32 {
        value: i32,
        operator: IntegerOperator,
    },
    U64 {
        value: u64,
        operator: IntegerOperator,
    },
    I64 {
        value: i64,
        operator: IntegerOperator,
    },
    U128 {
        value: u128,
        operator: IntegerOperator,
    },
    I128 {
        value: i128,
        operator: IntegerOperator,
    },
    Bytes {
        value: Vec<u8>,
        operator: ByteSliceOperator,
    },
    Pubkey {
        value: Pubkey,
        operator: EquatableOperator,
    },
}

impl Assert<&AccountInfo<'_>> for AccountDataAssertion {
    fn evaluate(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        let offset = self.offset as usize;
        let assertion = &self.assertion;

        let data = account.try_borrow_data().map_err(|e| {
            err_msg!("Cannot borrow data for target account", e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        match assertion {
            DataValueAssertion::Bool {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<bool>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::U8 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u8>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::I8 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i8>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::U16 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u16>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::I16 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i16>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::U32 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u32>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::I32 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i32>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::U64 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u64>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::I64 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i64>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::U128 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u128>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::I128 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i128>(&data, offset, None)?;
                operator.evaluate(&actual_value, assertion_value, log_level)
            }
            DataValueAssertion::Bytes {
                value: assertion_value,
                operator,
            } => {
                let actual_value = data
                    .get(offset..offset + assertion_value.len())
                    .ok_or_else(|| {
                        msg!("Data range out of bounds");
                        err!(LighthouseError::RangeOutOfBounds)
                    })?;

                operator.evaluate(actual_value, assertion_value.as_slice(), log_level)
            }
            DataValueAssertion::Pubkey {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<Pubkey>(&data, offset, None)?;

                operator.evaluate(&actual_value, assertion_value, log_level)
            }
        }
    }
}

///
///
///
///
///

#[cfg(test)]
mod tests {
    use solana_sdk::{
        account_info::AccountInfo, signature::Keypair, signer::EncodableKeypair, system_program,
    };

    use crate::{
        test_utils::{assert_failed, assert_passed, create_test_account},
        types::assert::{
            operator::{EquatableOperator, IntegerOperator},
            AccountDataAssertion, Assert, LogLevel,
        },
    };

    use super::DataValueAssertion;

    #[test]
    fn evaluate() {
        let key = system_program::id();
        let lamports = &mut 0;
        let test_account = create_test_account();
        let data: &mut [u8] = &mut [0u8; 171];
        data.copy_from_slice(test_account.try_to_vec_override().as_ref());
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        // Test all operators
        let assertions = vec![
            (
                DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                },
                true,
            ),
            (
                DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::NotEqual,
                },
                false,
            ),
            (
                DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::GreaterThan,
                },
                true,
            ),
            (
                DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
                true,
            ),
            (
                DataValueAssertion::U8 {
                    value: 2,
                    operator: IntegerOperator::LessThan,
                },
                true,
            ),
            (
                DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::LessThanOrEqual,
                },
                true,
            ),
        ];

        for (assertion, should_pass) in assertions {
            let assertion = AccountDataAssertion {
                offset: 0,
                assertion,
            };

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            if should_pass {
                assert_passed(result);
            } else {
                assert_failed(result);
            }
        }

        let assertions = vec![
            (
                DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::Equal,
                },
                true,
            ),
            (
                DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::NotEqual,
                },
                false,
            ),
            (
                DataValueAssertion::I8 {
                    value: -2,
                    operator: IntegerOperator::GreaterThan,
                },
                true,
            ),
            (
                DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
                true,
            ),
            (
                DataValueAssertion::I8 {
                    value: 0,
                    operator: IntegerOperator::LessThan,
                },
                true,
            ),
            (
                DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::LessThanOrEqual,
                },
                true,
            ),
        ];

        for (assertion, should_pass) in assertions {
            let assertion = AccountDataAssertion {
                offset: 1,
                assertion,
            };

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            if should_pass {
                assert_passed(result);
            } else {
                assert_failed(result);
            }
        }

        let assertions = vec![
            (
                DataValueAssertion::Pubkey {
                    value: test_account.pubkey,
                    operator: EquatableOperator::Equal,
                },
                true,
            ),
            (
                DataValueAssertion::Pubkey {
                    value: test_account.pubkey,
                    operator: EquatableOperator::NotEqual,
                },
                false,
            ),
            (
                DataValueAssertion::Pubkey {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                },
                false,
            ),
            (
                DataValueAssertion::Pubkey {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::NotEqual,
                },
                true,
            ),
        ];

        for (assertion, should_pass) in assertions {
            let assertion = AccountDataAssertion {
                offset: 103,
                assertion,
            };

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            if should_pass {
                assert_passed(result);
            } else {
                assert_failed(result);
            }
        }
    }
}
