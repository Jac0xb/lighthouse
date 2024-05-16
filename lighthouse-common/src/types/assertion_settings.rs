use std::io::Error;

use crate::integer_operator::IntegerOperator;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DataValue {
    Pubkey = 0,
    Bool = 1,
    Number = 2,
    SignedNumber = 3,
    Bytes = 4,
}

impl TryFrom<u8> for DataValue {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Pubkey),
            1 => Ok(Self::Bool),
            2 => Ok(Self::Number),
            3 => Ok(Self::SignedNumber),
            4 => Ok(Self::Bytes),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid DataValue",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssertionSettings {
    pub is_big_endian: bool,
    pub operator: IntegerOperator,
    pub data_value: DataValue,
}

pub struct CompactAssertionSettings(pub u8);

impl CompactAssertionSettings {
    const BIG_ENDIAN: u8 = 0b1000_0000;

    pub fn compact(is_big_endian: bool, operator: u8, data_value: DataValue) -> u8 {
        // The bottom 4 bits are dedicated to the operator enum
        let mut flags = operator;

        // the next 3 bits are dedicated to the data value enum
        flags |= (data_value as u8) << 4;

        if is_big_endian {
            flags |= Self::BIG_ENDIAN;
        }

        flags
    }

    pub fn decompact(&self) -> AssertionSettings {
        let is_big_endian = self.0 & Self::BIG_ENDIAN != 0;
        let operator = self.0 & 0b0000_1111;
        let data_value = (self.0 & 0b0111_0000) >> 4;

        AssertionSettings {
            is_big_endian,
            operator: IntegerOperator::try_from(operator).unwrap(),
            data_value: DataValue::try_from(data_value).unwrap(),
        }
    }
}
