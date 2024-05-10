use borsh::maybestd::io::Read;
use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt::Debug;
use std::io::Write;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompactU64(pub u64);

impl BorshSerialize for CompactU64 {
    fn serialize<W: Write>(&self, writer: &mut W) -> borsh::maybestd::io::Result<()> {
        leb128::write::unsigned(writer, self.0)?;

        Ok(())
    }
}

impl BorshDeserialize for CompactU64 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> borsh::maybestd::io::Result<Self> {
        let value = leb128::read::unsigned(reader).map_err(|e| {
            borsh::maybestd::io::Error::new(borsh::maybestd::io::ErrorKind::InvalidData, e)
        })?;

        Ok(Self(value))
    }
}

impl Deref for CompactU64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompactU64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_from_unsigned {
    ($($t:ty),*) => {
        $(
            impl From<$t> for CompactU64 {
                fn from(value: $t) -> Self {
                    CompactU64(value as u64)
                }
            }
        )*
    };
}

impl_from_unsigned!(u8, u16, u32, u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_data() {
        let mut data: Vec<u8> = vec![];

        leb128::write::unsigned(&mut data, 10).unwrap();

        let mut reader = &data[..];
        let leb128_vec: CompactU64 = BorshDeserialize::deserialize(&mut reader).unwrap();

        assert_eq!(leb128_vec, CompactU64(10));
    }

    #[test]
    fn serialize_data() {
        let leb128_vec: CompactU64 = CompactU64(10);

        let mut data: Vec<u8> = vec![];

        leb128_vec.serialize(&mut data).unwrap();

        assert_eq!(data, (vec![10]));
    }

    #[test]
    fn large_serialize_data() {
        let leb128_vec: CompactU64 = CompactU64(1000);

        let mut data: Vec<u8> = vec![];
        leb128_vec.serialize(&mut data).unwrap();

        // LEB128 encoding of 1000 is 0b11101000 0b00000111
        let expected_data: Vec<u8> = vec![0b11101000, 0b00000111];

        assert_eq!(data, expected_data);
    }

    #[test]
    fn max_u64_serialize_data() {
        let leb128_vec: CompactU64 = CompactU64(u64::MAX);

        let mut data: Vec<u8> = vec![];
        leb128_vec.serialize(&mut data).unwrap();

        assert_eq!(
            data,
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }

    #[test]
    fn max_u64_deserialize() {
        let data: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];

        let mut reader = &data[..];

        let leb128_vec: CompactU64 = BorshDeserialize::deserialize(&mut reader).unwrap();

        assert_eq!(leb128_vec, CompactU64(u64::MAX));
    }
}
