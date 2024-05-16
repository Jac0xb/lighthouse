use lighthouse_common::{
    assertion_settings::{CompactAssertionSettings, DataValue},
    integer_operator::IntegerOperator,
};
use solana_program::pubkey::Pubkey;

use crate::{
    generated::types::{AccountDataAssertion, EquatableOperator},
    hooked::CompactBytes,
};

pub struct AccountDataAssertionWrapper {
    pub offset: u64,
    pub assertion: DataValueAssertion,
}

pub fn convert_to_account_data_assertion(
    assertion: &DataValueAssertion,
    offset: u64,
) -> AccountDataAssertion {
    let offset = offset.into();

    match assertion {
        DataValueAssertion::Bool { value, operator } => AccountDataAssertion {
            bytes: vec![*value as u8].into(),
            offset,
            compact_assertion_settings: CompactAssertionSettings::compact(
                false,
                operator.clone() as u8,
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
                operator.clone() as u8,
                DataValue::Bytes,
            ),
        },
        DataValueAssertion::Pubkey { value, operator } => AccountDataAssertion {
            bytes: value.to_bytes().to_vec().into(),
            offset,
            compact_assertion_settings: CompactAssertionSettings::compact(
                false,
                operator.clone() as u8,
                DataValue::Pubkey,
            ),
        },
    }
}

impl AccountDataAssertionWrapper {
    // Deref into AccountDataAssertion
    pub fn convert_to_raw(&self) -> AccountDataAssertion {
        convert_to_account_data_assertion(&self.assertion, self.offset)
    }
}

impl From<AccountDataAssertionWrapper> for AccountDataAssertion {
    fn from(value: AccountDataAssertionWrapper) -> Self {
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

pub struct ComparsionBitflags(u8);
impl ComparsionBitflags {
    const BIG_ENDIAN: u8 = 0b1000_0000;
    const SIGNED: u8 = 0b0100_0000;

    pub fn build(is_big_endian: bool, is_signed: bool, operator: u8) -> u8 {
        let mut flags = operator;

        if is_big_endian {
            flags |= Self::BIG_ENDIAN;
        }

        if is_signed {
            flags |= Self::SIGNED;
        }

        flags
    }

    pub fn get_operator(&self) -> IntegerOperator {
        // the first 4 bytes are an enum that represents the operator
        let enum_val = self.0 & 0b0000_1111;

        match enum_val {
            0 => IntegerOperator::Equal,
            1 => IntegerOperator::NotEqual,
            2 => IntegerOperator::GreaterThan,
            3 => IntegerOperator::LessThan,
            4 => IntegerOperator::GreaterThanOrEqual,
            5 => IntegerOperator::LessThanOrEqual,
            6 => IntegerOperator::Contains,
            7 => IntegerOperator::DoesNotContain,
            _ => panic!("Invalid operator"),
        }
    }

    pub fn is_big_endian(&self) -> bool {
        self.0 & Self::BIG_ENDIAN != 0
    }

    pub fn is_signed(&self) -> bool {
        self.0 & Self::SIGNED != 0
    }
}
