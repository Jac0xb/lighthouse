use crate::{
    error::LighthouseError,
    types::{Assert, DataValue, EquatableOperator, EvaluationResult, IntegerOperator, Operator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, msg, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountDataAssertion {
    pub offset: u16,
    pub assertion: DataValueAssertion,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum DataValueAssertion {
    Bool(bool, EquatableOperator),
    U8(u8, IntegerOperator),
    I8(i8, IntegerOperator),
    U16(u16, IntegerOperator),
    I16(i16, IntegerOperator),
    U32(u32, IntegerOperator),
    I32(i32, IntegerOperator),
    U64(u64, IntegerOperator),
    I64(i64, IntegerOperator),
    U128(u128, IntegerOperator),
    I128(i128, IntegerOperator),
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

#[cfg(test)]
mod tests {
    use solana_sdk::{
        account_info::AccountInfo, signature::Keypair, signer::EncodableKeypair, system_program,
    };

    use crate::{
        test_utils::create_test_account,
        types::{
            AccountDataAssertion, Assert, DataValueAssertion, EquatableOperator, IntegerOperator,
        },
    };

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
            (DataValueAssertion::U8(1, IntegerOperator::Equal), true),
            (DataValueAssertion::U8(1, IntegerOperator::NotEqual), false),
            (
                DataValueAssertion::U8(0, IntegerOperator::GreaterThan),
                true,
            ),
            (
                DataValueAssertion::U8(1, IntegerOperator::GreaterThanOrEqual),
                true,
            ),
            (DataValueAssertion::U8(2, IntegerOperator::LessThan), true),
            (
                DataValueAssertion::U8(1, IntegerOperator::LessThanOrEqual),
                true,
            ),
        ];

        for (assertion, should_pass) in assertions {
            let assertion = AccountDataAssertion {
                offset: 0,
                assertion,
            };

            let result = assertion.evaluate(&account_info, true).unwrap();

            assert_eq!(
                result.passed, should_pass,
                "{:?} {:?}",
                assertion, result.output
            );
        }

        let assertions = vec![
            (DataValueAssertion::I8(-1, IntegerOperator::Equal), true),
            (DataValueAssertion::I8(-1, IntegerOperator::NotEqual), false),
            (
                DataValueAssertion::I8(-2, IntegerOperator::GreaterThan),
                true,
            ),
            (
                DataValueAssertion::I8(-1, IntegerOperator::GreaterThanOrEqual),
                true,
            ),
            (DataValueAssertion::I8(0, IntegerOperator::LessThan), true),
            (
                DataValueAssertion::I8(-1, IntegerOperator::LessThanOrEqual),
                true,
            ),
        ];

        for (assertion, should_pass) in assertions {
            let assertion = AccountDataAssertion {
                offset: 1,
                assertion,
            };

            let result = assertion.evaluate(&account_info, true).unwrap();

            assert_eq!(
                result.passed, should_pass,
                "{:?} {:?}",
                assertion, result.output
            );
        }

        let assertions = vec![
            (
                DataValueAssertion::Pubkey(test_account.pubkey, EquatableOperator::Equal),
                true,
            ),
            (
                DataValueAssertion::Pubkey(test_account.pubkey, EquatableOperator::NotEqual),
                false,
            ),
            (
                DataValueAssertion::Pubkey(
                    Keypair::new().encodable_pubkey(),
                    EquatableOperator::Equal,
                ),
                false,
            ),
            (
                DataValueAssertion::Pubkey(
                    Keypair::new().encodable_pubkey(),
                    EquatableOperator::NotEqual,
                ),
                true,
            ),
        ];

        for (assertion, should_pass) in assertions {
            let assertion = AccountDataAssertion {
                offset: 103,
                assertion,
            };

            let result = assertion.evaluate(&account_info, true).unwrap();

            assert_eq!(
                result.passed, should_pass,
                "{:?} {:?}",
                assertion, result.output
            );
        }
    }
}
