use super::{Assert, EquatableOperator, IntegerOperator, LogLevel};
use crate::{
    error::LighthouseError,
    types::assert::evaluate::Evaluate,
    utils::{checked_get_slice, try_from_slice, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
use lighthouse_common::CompactU64;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum AccountDeltaAssertion {
    AccountInfo {
        a_offset: CompactU64,
        assertion: AccountInfoDeltaAssertion,
    },
    Data {
        a_offset: CompactU64,
        b_offset: CompactU64,
        assertion: DataValueDeltaAssertion,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
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
    Bytes {
        length: u16,
        operator: EquatableOperator,
    },
}

impl<'a, 'info> Assert<(&'a AccountInfo<'info>, &'a AccountInfo<'info>)> for AccountDeltaAssertion {
    fn evaluate(
        &self,
        accounts: (&'a AccountInfo<'info>, &'a AccountInfo<'info>),
        log_level: LogLevel,
    ) -> Result<()> {
        match self {
            AccountDeltaAssertion::Data {
                a_offset,
                b_offset,
                assertion,
            } => {
                let a_offset = **a_offset as usize;
                let b_offset = **b_offset as usize;

                let (a_account, b_account) = accounts;

                let a_account_data = a_account
                    .try_borrow_data()
                    .map_err(LighthouseError::failed_borrow_err)?;
                let b_account_data = b_account
                    .try_borrow_data()
                    .map_err(LighthouseError::failed_borrow_err)?;

                if a_account_data.is_empty() || b_account_data.is_empty() {
                    return Err(LighthouseError::AccountNotInitialized.into());
                }

                match assertion {
                    DataValueDeltaAssertion::U8 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<u8>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<u8>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i16 - a_value as i16;

                        i16::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::I8 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<i8>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<i8>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i16 - a_value as i16;

                        i16::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::U16 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<u16>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<u16>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i32 - a_value as i32;

                        i32::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::I16 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<i16>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<i16>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i32 - a_value as i32;

                        i32::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::U32 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<u32>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<u32>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i64 - a_value as i64;

                        i64::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::I32 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<i32>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<i32>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i64 - a_value as i64;

                        i64::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::U64 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<u64>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<u64>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i128 - a_value as i128;

                        i128::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::I64 {
                        value: assertion_value,
                        operator,
                    } => {
                        let a_value = try_from_slice::<i64>(&a_account_data, a_offset)?;
                        let b_value = try_from_slice::<i64>(&b_account_data, b_offset)?;
                        let diff_value = b_value as i128 - a_value as i128;

                        i128::evaluate(&diff_value, assertion_value, operator, log_level)
                    }
                    DataValueDeltaAssertion::Bytes { operator, length } => {
                        let a_value =
                            checked_get_slice(&a_account_data, a_offset, *length as usize)?;

                        let b_value =
                            checked_get_slice(&b_account_data, b_offset, *length as usize)?;

                        <[u8]>::evaluate(a_value, b_value, operator, log_level)
                    }
                }
            }
            AccountDeltaAssertion::AccountInfo {
                a_offset,
                assertion,
            } => {
                let (a_account, b_account) = accounts;

                if a_account.data_is_empty() {
                    return Err(LighthouseError::AccountNotInitialized.into());
                }

                let a_account_data = a_account
                    .try_borrow_data()
                    .map_err(LighthouseError::failed_borrow_err)?;

                let a_offset = **a_offset as usize;

                match assertion {
                    AccountInfoDeltaAssertion::Lamports { value, operator } => {
                        let a_lamports = try_from_slice::<u64>(&a_account_data, a_offset)?;
                        let b_lamports = b_account.lamports();
                        let diff_value = b_lamports as i128 - a_lamports as i128;

                        i128::evaluate(&diff_value, value, operator, log_level)
                    }
                    AccountInfoDeltaAssertion::DataLength { value, operator } => {
                        let a_data_len = try_from_slice::<u64>(&a_account_data, a_offset)?;
                        let b_data_len = b_account.data_len() as i128;
                        let diff_value = b_data_len - a_data_len as i128;

                        i128::evaluate(&diff_value, value, operator, log_level)
                    }
                    AccountInfoDeltaAssertion::Owner { operator } => {
                        let a_owner = try_from_slice::<Pubkey>(&a_account_data, a_offset)?;

                        Pubkey::evaluate(&a_owner, b_account.owner, operator, log_level)
                    }
                    AccountInfoDeltaAssertion::RentEpoch { value, operator } => {
                        let a_rent_epoch = try_from_slice::<u64>(&a_account_data, a_offset)?;
                        let b_rent_epoch = b_account.rent_epoch;
                        let diff_value = b_rent_epoch as i128 - a_rent_epoch as i128;

                        i128::evaluate(&diff_value, value, operator, log_level)
                    }
                }
            }
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum AccountInfoDeltaAssertion {
    Lamports {
        value: i128,
        operator: IntegerOperator,
    },
    DataLength {
        value: i128,
        operator: IntegerOperator,
    },
    Owner {
        operator: EquatableOperator,
    },
    RentEpoch {
        value: i128,
        operator: IntegerOperator,
    },
}

///
///
///     Tests for `AccountDeltaAssertion`.
///
///

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::{assert_passed, create_test_account},
        types::assert::{
            AccountDeltaAssertion, AccountInfoDeltaAssertion, Assert, DataValueDeltaAssertion,
            EquatableOperator, IntegerOperator, LogLevel,
        },
    };
    use borsh::BorshSerialize;
    use lighthouse_common::CompactU64;
    use solana_sdk::{
        account_info::AccountInfo, signature::Keypair, signer::EncodableKeypair, system_program,
    };

    #[test]
    fn evaluate_diff_u8() {
        let key = system_program::id();
        let (lamports_b, lamports_a) = (&mut 0, &mut 0);
        let a_data: &mut [u8] = &mut [0u8; 171];
        a_data.copy_from_slice(create_test_account().try_to_vec().unwrap().as_ref());
        let a_account_info =
            AccountInfo::new(&key, false, false, lamports_b, a_data, &key, false, 0);

        let b_data: &mut [u8] = &mut [0u8; 1];
        let b_u8 = u8::MAX;
        b_data.copy_from_slice(b_u8.to_le_bytes().as_ref());
        let b_account_info =
            AccountInfo::new(&key, false, false, lamports_a, b_data, &key, false, 0);

        let assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(0),
            b_offset: CompactU64(0),
            assertion: DataValueDeltaAssertion::U8 {
                value: (u8::MAX as i16) - 1i16,
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(
            (&a_account_info, &b_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);

        let reverse_assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(0),
            b_offset: CompactU64(0),
            assertion: DataValueDeltaAssertion::U8 {
                value: 1i16 - (u8::MAX as i16),
                operator: IntegerOperator::Equal,
            },
        };

        let result = reverse_assertion.evaluate(
            (&b_account_info, &a_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);
    }

    #[test]
    fn evaluate_diff_i8() {
        let key = system_program::id();
        let (lamports_b, lamports_a) = (&mut 0, &mut 0);
        let a_data: &mut [u8] = &mut [0u8; 171];
        let test_account = create_test_account();
        a_data.copy_from_slice(create_test_account().try_to_vec().unwrap().as_ref());
        let a_account_info =
            AccountInfo::new(&key, false, false, lamports_b, a_data, &key, false, 0);

        let b_data: &mut [u8] = &mut [0u8; 1];
        let b_i8 = i8::MIN;
        b_data.copy_from_slice(b_i8.to_le_bytes().as_ref());
        let b_account_info =
            AccountInfo::new(&key, false, false, lamports_a, b_data, &key, false, 0);

        let assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(1),
            b_offset: CompactU64(0),
            assertion: DataValueDeltaAssertion::I8 {
                value: (i8::MIN as i16) - (test_account.i8 as i16),
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(
            (&a_account_info.clone(), &b_account_info.clone()),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);

        let reverse_assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(0),
            b_offset: CompactU64(1),
            assertion: DataValueDeltaAssertion::I8 {
                value: (test_account.i8 as i16) - (i8::MIN as i16),
                operator: IntegerOperator::Equal,
            },
        };

        let result = reverse_assertion.evaluate(
            (&b_account_info, &a_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);
    }

    #[test]
    fn evaluate_diff_u16() {
        let key = system_program::id();
        let (lamports_b, lamports_a) = (&mut 0, &mut 0);
        let a_data: &mut [u8] = &mut [0u8; 171];
        let test_account = create_test_account();
        a_data.copy_from_slice(test_account.try_to_vec().unwrap().as_ref());
        let a_account_info =
            AccountInfo::new(&key, false, false, lamports_b, a_data, &key, false, 0);

        let b_data: &mut [u8] = &mut [0u8; 2];
        let b_u16 = u16::MAX;
        b_data.copy_from_slice(b_u16.to_le_bytes().as_ref());
        let b_account_info =
            AccountInfo::new(&key, false, false, lamports_a, b_data, &key, false, 0);

        let assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(2),
            b_offset: CompactU64(0),
            assertion: DataValueDeltaAssertion::U16 {
                value: (u16::MAX as i32) - (test_account.u16 as i32),
                operator: IntegerOperator::Equal,
            },
        };

        let result = assertion.evaluate(
            (&a_account_info.clone(), &b_account_info.clone()),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);

        let reverse_assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(0),
            b_offset: CompactU64(2),
            assertion: DataValueDeltaAssertion::U16 {
                value: (test_account.u16 as i32) - (u16::MAX as i32),
                operator: IntegerOperator::Equal,
            },
        };

        let result = reverse_assertion.evaluate(
            (&b_account_info, &a_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);
    }

    #[test]
    fn evaluate_diff_i16() {
        let key = system_program::id();
        let (lamports_b, lamports_a) = (&mut 0, &mut 0);
        let a_data: &mut [u8] = &mut [0u8; 171];
        let test_account = create_test_account();
        a_data.copy_from_slice(test_account.try_to_vec().unwrap().as_ref());
        let a_account_info =
            AccountInfo::new(&key, false, false, lamports_b, a_data, &key, false, 0);

        let b_data: &mut [u8] = &mut [0u8; 2];
        let b_i16 = i16::MIN;
        b_data.copy_from_slice(b_i16.to_le_bytes().as_ref());
        let b_account_info =
            AccountInfo::new(&key, false, false, lamports_a, b_data, &key, false, 0);

        let assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(4),
            b_offset: CompactU64(0),
            assertion: DataValueDeltaAssertion::I16 {
                value: (i16::MIN as i32) - (test_account.i16 as i32) - 10,
                operator: IntegerOperator::GreaterThan,
            },
        };

        let result = assertion.evaluate(
            (&a_account_info.clone(), &b_account_info.clone()),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);

        let reverse_assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(0),
            b_offset: CompactU64(4),
            assertion: DataValueDeltaAssertion::I16 {
                value: (test_account.i16 as i32) - (i16::MIN as i32) + 10,
                operator: IntegerOperator::LessThan,
            },
        };

        let result = reverse_assertion.evaluate(
            (&b_account_info, &a_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);
    }

    #[test]
    fn evaluate_diff_bytes() {
        let key = system_program::id();
        let (lamports_b, lamports_a) = (&mut 0, &mut 0);
        let a_data: &mut [u8] = &mut [0u8; 32];

        let keypair = Keypair::new().encodable_pubkey().to_bytes();
        a_data.copy_from_slice(keypair.as_ref());
        let a_account_info =
            AccountInfo::new(&key, false, false, lamports_b, a_data, &key, false, 0);

        let b_data: &mut [u8] = &mut [0u8; 36];
        b_data[4..].copy_from_slice(keypair.as_ref());
        let b_account_info =
            AccountInfo::new(&key, false, false, lamports_a, b_data, &key, false, 0);

        let assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(0),
            b_offset: CompactU64(4),
            assertion: DataValueDeltaAssertion::Bytes {
                operator: EquatableOperator::Equal,
                length: 32,
            },
        };

        let result = assertion.evaluate(
            (&a_account_info.clone(), &b_account_info.clone()),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);

        let reverse_assertion = AccountDeltaAssertion::Data {
            a_offset: CompactU64(4),
            b_offset: CompactU64(0),
            assertion: DataValueDeltaAssertion::Bytes {
                operator: EquatableOperator::Equal,
                length: 32,
            },
        };

        let result = reverse_assertion.evaluate(
            (&b_account_info, &a_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);
    }

    #[test]
    fn evaluate_lamport_delta() {
        let key = system_program::id();
        let (lamports_b, lamports_a) = (&mut 0, &mut 0);
        let a_data: &mut [u8] = &mut [0u8; 128];
        let a_account_info =
            AccountInfo::new(&key, false, false, lamports_b, a_data, &key, false, 0);

        let b_data: &mut [u8] = &mut [0u8; 128];
        let b_account_info =
            AccountInfo::new(&key, false, false, lamports_a, b_data, &key, false, 0);

        // Positive Value (0 to 100 = 100)

        let mut mut_ref = b_account_info.try_borrow_mut_lamports().unwrap();
        **mut_ref = 100;
        drop(mut_ref);

        let assertion = AccountDeltaAssertion::AccountInfo {
            a_offset: CompactU64(0),
            assertion: AccountInfoDeltaAssertion::Lamports {
                value: 100,
                operator: IntegerOperator::Equal,
            },
        };

        assertion
            .evaluate(
                (&a_account_info, &b_account_info),
                LogLevel::PlaintextMessage,
            )
            .unwrap();

        // Negative Value (100 to 0 = -100)

        let mut mut_ref = b_account_info.try_borrow_mut_lamports().unwrap();
        **mut_ref = 0;
        drop(mut_ref);

        let mut mut_ref = a_account_info.try_borrow_mut_data().unwrap();
        mut_ref[0..8].copy_from_slice(100u64.to_le_bytes().as_ref());
        drop(mut_ref);

        let reverse_assertion = AccountDeltaAssertion::AccountInfo {
            a_offset: CompactU64(0),
            assertion: AccountInfoDeltaAssertion::Lamports {
                value: -100,
                operator: IntegerOperator::Equal,
            },
        };

        reverse_assertion
            .evaluate(
                (&a_account_info, &b_account_info),
                LogLevel::PlaintextMessage,
            )
            .unwrap();

        // Negative Value (100 to 0 <= -50)

        let mut mut_ref = b_account_info.try_borrow_mut_lamports().unwrap();
        **mut_ref = 0;
        drop(mut_ref);

        let mut mut_ref = a_account_info.try_borrow_mut_data().unwrap();
        mut_ref[0..8].copy_from_slice(100u64.to_le_bytes().as_ref());
        drop(mut_ref);

        let reverse_assertion = AccountDeltaAssertion::AccountInfo {
            a_offset: CompactU64(0),
            assertion: AccountInfoDeltaAssertion::Lamports {
                value: -150,
                operator: IntegerOperator::GreaterThan,
            },
        };

        let result = reverse_assertion.evaluate(
            (&a_account_info, &b_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);
    }

    #[test]
    fn evaluate_owner_delta() {
        let key = system_program::id();
        let owner = Keypair::new().encodable_pubkey();

        let (lamports_b, lamports_a) = (&mut 0, &mut 0);
        let a_data: &mut [u8] = &mut [0u8; 128];
        let a_account_info =
            AccountInfo::new(&key, false, false, lamports_b, a_data, &key, false, 0);

        // Store owner in data
        a_account_info.try_borrow_mut_data().unwrap()[8..40].copy_from_slice(&owner.to_bytes());

        let b_data: &mut [u8] = &mut [0u8; 128];
        let b_account_info =
            AccountInfo::new(&key, false, false, lamports_a, b_data, &owner, false, 0);

        let assertion = AccountDeltaAssertion::AccountInfo {
            a_offset: CompactU64(8),
            assertion: AccountInfoDeltaAssertion::Owner {
                operator: EquatableOperator::Equal,
            },
        };

        let result = assertion.evaluate(
            (&a_account_info, &b_account_info),
            LogLevel::PlaintextMessage,
        );

        assert_passed(result);
    }
}
