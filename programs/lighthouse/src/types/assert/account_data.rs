use super::{evaluate_bytes, Assert, LogLevel};
use crate::{err, err_msg, error::LighthouseError, types::CompactBytes, utils::Result};
use borsh::{BorshDeserialize, BorshSerialize};
use lighthouse_common::{assertion_settings::CompactAssertionSettings, CompactU64};
use solana_program::{account_info::AccountInfo, msg};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AccountDataAssertion {
    pub offset: CompactU64,
    pub bytes: CompactBytes,
    pub compact_assertion_settings: u8,
}

impl Assert<&AccountInfo<'_>> for AccountDataAssertion {
    fn evaluate(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        let offset = *self.offset as usize;
        let length = self.bytes.len();

        let data = account.try_borrow_data().map_err(|e| {
            err_msg!("Cannot borrow data for target account", e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        if data.is_empty() {
            return Err(LighthouseError::AccountNotInitialized.into());
        }

        let data_slice = data.get(offset..offset + length).ok_or_else(|| {
            msg!(
                "Failed to deserialize data range {:?} was out of bounds",
                offset..offset + length
            );

            LighthouseError::RangeOutOfBounds
        })?;

        let settings = CompactAssertionSettings(self.compact_assertion_settings);
        let assertion_settings = settings.decompact();

        // if is_big_endian {
        //     panic!("Big endian is not supported yet")
        // }

        // msg!(
        //     "Actual: {:?} Operator: {:?} Expected: {:?} | Signed: {:?} BE: {:?}",
        //     data_slice,
        //     operator,
        //     self.bytes,
        //     is_signed,
        //     is_big_endian
        // );

        // let ordering: Ordering = if is_signed {
        //     compare_signed_bytes(data_slice, &self.bytes)
        // } else {
        //     compare_unsigned_bytes(data_slice, &self.bytes)
        // };

        evaluate_bytes(data_slice, &self.bytes, &assertion_settings, log_level)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::LighthouseError,
        test_utils::{assert_failed, assert_passed, create_test_account},
        types::{
            assert::{evaluate::EquatableOperator, AccountDataAssertion, Assert, LogLevel},
            CompactBytes,
        },
    };
    use borsh::BorshSerialize;
    use lighthouse_common::{
        assertion_settings::{CompactAssertionSettings, DataValue},
        integer_operator::IntegerOperator,
    };
    use solana_sdk::{
        account_info::AccountInfo, msg, pubkey::Pubkey, signature::Keypair,
        signer::EncodableKeypair, system_program,
    };

    pub struct AccountDataAssertionSimple {
        pub offset: u64,
        pub assertion: DataValueAssertion,
    }

    impl AccountDataAssertionSimple {
        // Deref into AccountDataAssertion
        pub fn convert_to_raw(&self) -> AccountDataAssertion {
            self.assertion
                .convert_to_account_data_assertion(self.offset)
        }
    }

    impl From<AccountDataAssertionSimple> for AccountDataAssertion {
        fn from(value: AccountDataAssertionSimple) -> Self {
            value.convert_to_raw()
        }
    }

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
            value: CompactBytes,
            operator: EquatableOperator,
        },
        Pubkey {
            value: Pubkey,
            operator: EquatableOperator,
        },
    }

    impl DataValueAssertion {
        pub fn convert_to_account_data_assertion(&self, offset: u64) -> AccountDataAssertion {
            let offset = offset.into();

            match self {
                DataValueAssertion::Bool { value, operator } => AccountDataAssertion {
                    bytes: vec![*value as u8].into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Bool,
                    ),
                },
                DataValueAssertion::U8 { value, operator } => AccountDataAssertion {
                    bytes: vec![*value].into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Number,
                    ),
                },
                DataValueAssertion::I8 { value, operator } => AccountDataAssertion {
                    bytes: vec![*value as u8].into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::SignedNumber,
                    ),
                },
                DataValueAssertion::U16 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Number,
                    ),
                },
                DataValueAssertion::I16 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::SignedNumber,
                    ),
                },
                DataValueAssertion::U32 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Number,
                    ),
                },
                DataValueAssertion::I32 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::SignedNumber,
                    ),
                },
                DataValueAssertion::U64 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Number,
                    ),
                },
                DataValueAssertion::I64 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::SignedNumber,
                    ),
                },
                DataValueAssertion::U128 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Number,
                    ),
                },
                DataValueAssertion::I128 { value, operator } => AccountDataAssertion {
                    bytes: value.to_le_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::SignedNumber,
                    ),
                },
                DataValueAssertion::Bytes { value, operator } => AccountDataAssertion {
                    bytes: value.clone(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Bytes,
                    ),
                },
                DataValueAssertion::Pubkey { value, operator } => AccountDataAssertion {
                    bytes: value.to_bytes().to_vec().into(),
                    offset,
                    compact_assertion_settings: CompactAssertionSettings::compact(
                        false,
                        *operator as u8,
                        DataValue::Pubkey,
                    ),
                },
            }
        }
    }

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
            let assertion: AccountDataAssertion = AccountDataAssertionSimple {
                offset: 0u8.into(),
                assertion,
            }
            .into();

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            msg!("bytes, {:?}", assertion);

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
            // let assertion = AccountDataAssertion {
            //     offset: 1u8.into(),
            //     assertion,
            // };

            let assertion = assertion.convert_to_account_data_assertion(1);

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
            // let assertion = AccountDataAssertion {
            //     offset: 103u8.into(),
            //     assertion,
            // };

            let assertion = assertion.convert_to_account_data_assertion(103);

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

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::U8 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, LighthouseError::AccountNotInitialized.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i8

        let data: &mut [u8] = &mut [0u8; 0];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::I8 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();
        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, LighthouseError::AccountNotInitialized.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u16

        let data: &mut [u8] = &mut [0u8; 1];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::U16 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i16

        let data: &mut [u8] = &mut [0u8; 1];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::I16 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u32

        let data: &mut [u8] = &mut [0u8; 2];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::U32 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i32

        let data: &mut [u8] = &mut [0u8; 2];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::I32 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u64

        let data: &mut [u8] = &mut [0u8; 4];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::U64 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i64

        let data: &mut [u8] = &mut [0u8; 4];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::I64 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on u128

        let data: &mut [u8] = &mut [0u8; 8];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::U128 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i128

        let data: &mut [u8] = &mut [0u8; 8];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::I128 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on pubkey

        let data: &mut [u8] = &mut [0u8; 24];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::Pubkey {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on bool

        let data: &mut [u8] = &mut [0u8; 0];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::Bool {
                value: true,
                operator: EquatableOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::AccountNotInitialized.into());
        } else {
            panic!("Expected error");
        }

        // Fail on bytes

        let data: &mut [u8] = &mut [0u8; 32];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::Bytes {
                value: vec![u8::MAX; 33].into(),
                operator: EquatableOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
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
        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 7u8.into(),
            assertion: DataValueAssertion::U16 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();
        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);
        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on i128
        let data: &mut [u8] = &mut [0u8; 16];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);
        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 9u8.into(),
            assertion: DataValueAssertion::I128 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
        } else {
            panic!("Expected error");
        }

        // Fail on bytes
        let data: &mut [u8] = &mut [0u8; 32];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);
        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 17u8.into(),
            assertion: DataValueAssertion::Bytes {
                value: vec![u8::MAX; 16].into(),
                operator: EquatableOperator::Equal,
            },
        }
        .into();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::RangeOutOfBounds.into());
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

        let assertion: AccountDataAssertion = AccountDataAssertionSimple {
            offset: 0u8.into(),
            assertion: DataValueAssertion::U8 {
                value: 1,
                operator: IntegerOperator::Equal,
            },
        }
        .into();

        let data = account_info.try_borrow_mut_data().unwrap();

        let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

        if let Err(e) = result {
            assert_eq!(e, LighthouseError::AccountBorrowFailed.into());
        } else {
            panic!("Expected error");
        }

        drop(data);
    }

    #[test]
    fn fuzz_range_test() {
        let key = system_program::id();
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [0u8; 64];
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);

        //
        // Test ranges [-10, 10] for i8 where actual value = 0
        //

        for i in -10..=10 {
            let actual_value = account_info.try_borrow_data().unwrap()[0];
            let operator = match (actual_value as i8).cmp(&i) {
                std::cmp::Ordering::Equal => IntegerOperator::Equal,
                std::cmp::Ordering::Less => IntegerOperator::LessThan,
                std::cmp::Ordering::Greater => IntegerOperator::GreaterThan,
            };

            let assertion: AccountDataAssertion = AccountDataAssertionSimple {
                offset: 0u8.into(),
                assertion: DataValueAssertion::I8 { value: i, operator },
            }
            .into();

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
        }

        //
        // Test ranges [-i16:min, i16:max] for i16
        //

        let actual_value: i16 = i8::MAX as i16 + 1;
        let mut data_mut_ref = account_info.try_borrow_mut_data().unwrap();
        data_mut_ref[0..2].copy_from_slice(&actual_value.to_le_bytes());

        drop(data_mut_ref);

        let range = i16::MIN as i32..=i16::MAX as i32;
        let step = (i16::MAX as i32 - i16::MIN as i32) / 99;

        for i in (range).step_by(step as usize).map(|x| x as i16) {
            let operator = match actual_value.cmp(&i) {
                std::cmp::Ordering::Equal => IntegerOperator::Equal,
                std::cmp::Ordering::Less => IntegerOperator::LessThan,
                std::cmp::Ordering::Greater => IntegerOperator::GreaterThan,
            };

            let assertion: AccountDataAssertion = AccountDataAssertionSimple {
                offset: 0u8.into(),
                assertion: DataValueAssertion::I16 { value: i, operator },
            }
            .into();

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
        }

        //
        // Test ranges [-i32:min, i32:max] for i32
        //

        let actual_value: i32 = i16::MAX as i32 + 1;
        let mut data_mut_ref = account_info.try_borrow_mut_data().unwrap();
        data_mut_ref[0..4].copy_from_slice(&actual_value.to_le_bytes());

        drop(data_mut_ref);

        let range = i32::MIN..=i32::MAX;
        let step = (i32::MAX as i64 - i32::MIN as i64) / 99;

        for i in (range).step_by(step as usize) {
            let operator = match actual_value.cmp(&i) {
                std::cmp::Ordering::Equal => IntegerOperator::Equal,
                std::cmp::Ordering::Less => IntegerOperator::LessThan,
                std::cmp::Ordering::Greater => IntegerOperator::GreaterThan,
            };

            let assertion: AccountDataAssertion = AccountDataAssertionSimple {
                offset: 0u8.into(),
                assertion: DataValueAssertion::I32 { value: i, operator },
            }
            .into();

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
        }

        //
        // Test ranges [-i64:min, i64:max] for i64
        //

        let actual_value: i64 = i32::MIN as i64 - 1;
        let mut data_mut_ref = account_info.try_borrow_mut_data().unwrap();
        data_mut_ref[0..8].copy_from_slice(&actual_value.to_le_bytes());

        drop(data_mut_ref);

        let range = i64::MIN..=i64::MAX;
        let step = (i64::MAX as i128 - i64::MIN as i128) / 99;

        for i in (range).step_by(step as usize) {
            let operator = match actual_value.cmp(&i) {
                std::cmp::Ordering::Equal => IntegerOperator::Equal,
                std::cmp::Ordering::Less => IntegerOperator::LessThan,
                std::cmp::Ordering::Greater => IntegerOperator::GreaterThan,
            };

            let assertion: AccountDataAssertion = AccountDataAssertionSimple {
                offset: 0u8.into(),
                assertion: DataValueAssertion::I64 { value: i, operator },
            }
            .into();

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
        }

        //
        // Test ranges [0, u8::MAX] for u8
        //

        let actual_value: u8 = 0;
        let mut data_mut_ref = account_info.try_borrow_mut_data().unwrap();
        data_mut_ref[0] = actual_value;

        drop(data_mut_ref);

        let range = 0..=u8::MAX;
        let step = u8::MAX as u16 / 99;

        for i in (range).step_by(step as usize) {
            let operator = match actual_value.cmp(&i) {
                std::cmp::Ordering::Equal => IntegerOperator::Equal,
                std::cmp::Ordering::Less => IntegerOperator::LessThan,
                std::cmp::Ordering::Greater => IntegerOperator::GreaterThan,
            };

            let assertion: AccountDataAssertion = AccountDataAssertionSimple {
                offset: 0u8.into(),
                assertion: DataValueAssertion::U8 { value: i, operator },
            }
            .into();

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
        }

        //
        // Test ranges [0, u64::MAX] for u64

        let actual_value: u64 = 0;
        let mut data_mut_ref = account_info.try_borrow_mut_data().unwrap();
        data_mut_ref[0..8].copy_from_slice(&actual_value.to_le_bytes());

        drop(data_mut_ref);

        let range = 0..=u64::MAX;
        let step = u64::MAX as u128 / 99;

        for i in (range).step_by(step as usize) {
            let operator = match actual_value.cmp(&i) {
                std::cmp::Ordering::Equal => IntegerOperator::Equal,
                std::cmp::Ordering::Less => IntegerOperator::LessThan,
                std::cmp::Ordering::Greater => IntegerOperator::GreaterThan,
            };

            let assertion: AccountDataAssertion = AccountDataAssertionSimple {
                offset: 0u8.into(),
                assertion: DataValueAssertion::U64 { value: i, operator },
            }
            .into();

            let result = assertion.evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
        }
    }
}
