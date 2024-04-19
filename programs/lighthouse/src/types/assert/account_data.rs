use super::{Assert, LogLevel};
use crate::{
    err, err_msg,
    error::lighthausError,
    types::assert::evaluate::{EquatableOperator, Evaluate, IntegerOperator},
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
        operator: EquatableOperator,
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
            err!(lighthausError::AccountBorrowFailed)
        })?;

        if data.is_empty() {
            return Err(lighthausError::AccountNotInitialized.into());
        }

        match assertion {
            DataValueAssertion::Bool {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<bool>(&data, offset, None)?;
                bool::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::U8 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u8>(&data, offset, None)?;
                u8::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::I8 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i8>(&data, offset, None)?;
                i8::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::U16 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u16>(&data, offset, None)?;
                u16::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::I16 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i16>(&data, offset, None)?;
                i16::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::U32 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u32>(&data, offset, None)?;
                u32::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::I32 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i32>(&data, offset, None)?;
                i32::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::U64 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u64>(&data, offset, None)?;
                u64::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::I64 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i64>(&data, offset, None)?;
                i64::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::U128 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<u128>(&data, offset, None)?;
                u128::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::I128 {
                value: assertion_value,
                operator,
            } => {
                let actual_value = try_from_slice::<i128>(&data, offset, None)?;
                i128::evaluate(&actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::Bytes {
                value: assertion_value,
                operator,
            } => {
                let actual_value = data
                    .get(offset..offset + assertion_value.len())
                    .ok_or_else(|| {
                        msg!("Data range out of bounds");
                        err!(lighthausError::RangeOutOfBounds)
                    })?;

                <[u8]>::evaluate(actual_value, assertion_value, operator, log_level)
            }
            DataValueAssertion::Pubkey {
                value: assertion_value,
                operator,
            } => {
                let data_slice = data.get(offset..offset + 32).ok_or_else(|| {
                    msg!(
                        "Failed to deserialized Pubkey range {:?} was out of bounds",
                        offset..offset + 32
                    );

                    lighthausError::RangeOutOfBounds
                })?;
                let actual_value = bytemuck::from_bytes::<Pubkey>(data_slice);

                Pubkey::evaluate(actual_value, assertion_value, operator, log_level)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataValueAssertion;
    use crate::{
        error::lighthausError,
        test_utils::{assert_failed, assert_passed, create_test_account},
        types::assert::{
            evaluate::{EquatableOperator, IntegerOperator},
            AccountDataAssertion, Assert, LogLevel,
        },
    };
    use borsh::BorshSerialize;
    use solana_sdk::{
        account_info::AccountInfo, signature::Keypair, signer::EncodableKeypair, system_program,
    };

    #[test]
    fn evaluate() {
        let key = system_program::id();
        let lamports = &mut 0;
        let test_account = create_test_account();
        let data: &mut [u8] = &mut [0u8; 171];
        data.copy_from_slice(test_account.try_to_vec().unwrap().as_ref());
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

    #[test]
    fn fail_try_from_slice() {
        let key = system_program::id();
        let lamports = &mut 0;

        // Fail on u8

        let data: &mut [u8] = &mut [0u8; 0];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::U8 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };
        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, lighthausError::AccountNotInitialized.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i8

        let data: &mut [u8] = &mut [0u8; 0];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::I8 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };
        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, lighthausError::AccountNotInitialized.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u16

        let data: &mut [u8] = &mut [0u8; 1];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::U16 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i16

        let data: &mut [u8] = &mut [0u8; 1];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::I16 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u32

        let data: &mut [u8] = &mut [0u8; 2];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::U32 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i32

        let data: &mut [u8] = &mut [0u8; 2];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::I32 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u64

        let data: &mut [u8] = &mut [0u8; 4];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::U64 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i64

        let data: &mut [u8] = &mut [0u8; 4];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::I64 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u128

        let data: &mut [u8] = &mut [0u8; 8];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::U128 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i128

        let data: &mut [u8] = &mut [0u8; 8];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::I128 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on pubkey

        let data: &mut [u8] = &mut [0u8; 24];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::Pubkey {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on bool

        let data: &mut [u8] = &mut [0u8; 0];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::Bool {
                value: true,
                operator: EquatableOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::AccountNotInitialized.into());
        } else {
            panic!("Expected error");
        }

        // Fail on bytes

        let data: &mut [u8] = &mut [0u8; 32];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::Bytes {
                value: vec![u8::MAX; 33],
                operator: EquatableOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    fn fail_try_from_slice_with_offset() {
        let key = system_program::id();
        let lamports = &mut 0;

        // Fail on u16
        let data: &mut [u8] = &mut [0u8; 8];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);
        let assertion = AccountDataAssertion {
            offset: 7,
            assertion: DataValueAssertion::U16 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };
        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i128
        let data: &mut [u8] = &mut [0u8; 16];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);
        let assertion = AccountDataAssertion {
            offset: 9,
            assertion: DataValueAssertion::I128 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on bytes
        let data: &mut [u8] = &mut [0u8; 32];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);
        let assertion = AccountDataAssertion {
            offset: 17,
            assertion: DataValueAssertion::Bytes {
                value: vec![u8::MAX; 16],
                operator: EquatableOperator::Equal,
            },
        };

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    fn fail_to_borrow_account() {
        let key = system_program::id();
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [0u8; 64];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion = AccountDataAssertion {
            offset: 0,
            assertion: DataValueAssertion::U8 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        };

        let data = account_info.try_borrow_mut_data().unwrap();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, lighthausError::AccountBorrowFailed.into());
        } else {
            panic!("Expected error");
        }

        drop(data);
    }
}
