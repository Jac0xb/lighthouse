use super::{AccountInfoField, ClockField, DataValue};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum WriteType {
    AccountData { offset: u16, data_length: u16 },
    AccountInfoField(AccountInfoField),
    DataValue(DataValue),
    Clock(ClockField),
}

impl WriteType {
    pub fn data_length(&self) -> u64 {
        match self {
            WriteType::AccountData {
                offset: _,
                data_length,
            } => *data_length as u64,
            WriteType::AccountInfoField(field) => match field {
                AccountInfoField::Key => 32,
                AccountInfoField::Lamports => 8,
                AccountInfoField::Owner => 32,
                AccountInfoField::RentEpoch => 8,
                AccountInfoField::DataLength => 8,
                AccountInfoField::Executable => 1,
            },
            WriteType::DataValue(memory_value) => match memory_value {
                DataValue::Bool(_) | DataValue::U8(_) | DataValue::I8(_) => 1,
                DataValue::U16(_) | DataValue::I16(_) => 2,
                DataValue::U32(_) | DataValue::I32(_) => 4,
                DataValue::U64(_) | DataValue::I64(_) => 8,
                DataValue::U128(_) | DataValue::I128(_) => 16,
                DataValue::Bytes(bytes) => bytes.len() as u64,
                DataValue::Pubkey(_) => 32,
            },
            WriteType::Clock(field) => match field {
                ClockField::Slot => 8,
                ClockField::EpochStartTimestamp => 8,
                ClockField::Epoch => 8,
                ClockField::LeaderScheduleEpoch => 8,
                ClockField::UnixTimestamp => 8,
            },
        }
    }
}
