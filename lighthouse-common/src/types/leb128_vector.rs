use borsh::maybestd::io::Read;
use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt::Debug;
use std::io::Write;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LEB128Vec<T: BorshSerialize + BorshDeserialize>(Vec<T>);

/// Deferences the inner `Vec` type.
impl<T> Deref for LEB128Vec<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Deferences the inner `Vec` type as mutable.
impl<T> DerefMut for LEB128Vec<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Debug` implementation for `LEB128Vec`.
///
/// This implementation simply forwards to the inner `Vec` type.
impl<T> Debug for LEB128Vec<T>
where
    T: BorshSerialize + BorshDeserialize + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl<T> BorshDeserialize for LEB128Vec<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn deserialize_reader<R: Read>(reader: &mut R) -> borsh::maybestd::io::Result<Self> {
        let mut items: Vec<T> = Vec::new();

        let len = leb128::read::unsigned(reader).map_err(|e| {
            borsh::maybestd::io::Error::new(borsh::maybestd::io::ErrorKind::InvalidData, e)
        })? as usize;

        for _ in 0..len {
            items.push(T::deserialize_reader(reader)?);
        }

        Ok(Self(items))
    }
}

impl<T> BorshSerialize for LEB128Vec<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> borsh::maybestd::io::Result<()> {
        leb128::write::unsigned(writer, self.0.len() as u64)?;

        for item in self.0.iter() {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

impl<T: BorshSerialize + BorshDeserialize> From<Vec<T>> for LEB128Vec<T> {
    fn from(vec: Vec<T>) -> Self {
        LEB128Vec(vec)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use super::*;

    #[test]
    fn deserialize_data() {
        let mut data: Vec<u8> = vec![];

        leb128::write::unsigned(&mut data, 10).unwrap();
        for i in 1..=10 {
            data.push(i);
        }

        let mut reader = &data[..];
        let leb128_vec: LEB128Vec<u8> = BorshDeserialize::deserialize(&mut reader).unwrap();

        assert_eq!(leb128_vec, LEB128Vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    fn serialize_data() {
        let leb128_vec: LEB128Vec<u8> = LEB128Vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut data: Vec<u8> = vec![];
        leb128_vec.serialize(&mut data).unwrap();

        assert_eq!(data, (vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    fn large_serialize_data() {
        let leb128_vec: LEB128Vec<u16> = LEB128Vec((0..1000).collect());

        let mut data: Vec<u8> = vec![];
        leb128_vec.serialize(&mut data).unwrap();

        // LEB128 encoding of 1000 is 0b11101000 0b00000111
        let mut expected_data: Vec<u8> = vec![0b11101000, 0b00000111];

        for i in 0..1000 {
            (i as u16).serialize(&mut expected_data).unwrap();
        }

        assert_eq!(data, expected_data);
    }

    #[test]
    fn large_deserialize_data() {
        let mut data: Vec<u8> = vec![];

        // LEB128 encoding of 1000 is 0b11101000 0b00000111
        data.push(0b11101000);
        data.push(0b00000111);

        for i in 0..1000 {
            (i as u16).serialize(&mut data).unwrap();
        }

        let mut reader = &data[..];

        let leb128_vec: LEB128Vec<u16> = BorshDeserialize::deserialize(&mut reader).unwrap();

        assert_eq!(leb128_vec, LEB128Vec((0..1000).collect()));
    }

    #[test]
    fn empty_serialize_data() {
        let leb128_vec: LEB128Vec<u8> = LEB128Vec(vec![]);

        let mut data: Vec<u8> = vec![];
        leb128_vec.serialize(&mut data).unwrap();

        assert_eq!(data, (vec![0]));
    }

    #[test]
    fn incomplete_data() {
        let mut data: Vec<u8> = vec![];
        leb128::write::unsigned(&mut data, 10).unwrap();

        data.push(1);
        data.push(2);

        let mut reader = &data[..];
        let leb128_vec: Result<LEB128Vec<u8>, Error> = BorshDeserialize::deserialize(&mut reader);

        assert!(leb128_vec.is_err());
    }

    #[test]
    fn empty_encoding() {
        let data: Vec<u8> = vec![];
        let mut reader = &data[..];
        let leb128_vec: Result<LEB128Vec<u8>, Error> = BorshDeserialize::deserialize(&mut reader);

        assert!(leb128_vec.is_err());
    }

    #[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
    struct TestStruct {
        a: u8,
        b: u16,
        c: Vec<String>,
    }

    #[test]
    fn struct_serialize() {
        let leb128_vec: LEB128Vec<TestStruct> = LEB128Vec(vec![
            TestStruct {
                a: 1,
                b: 2,
                c: vec!["hello".to_string()],
            },
            TestStruct {
                a: 3,
                b: 4,
                c: vec!["world".to_string()],
            },
        ]);

        let mut data: Vec<u8> = vec![];
        leb128_vec.serialize(&mut data).unwrap();

        let mut expected_data: Vec<u8> = vec![];
        leb128::write::unsigned(&mut expected_data, 2).unwrap();

        TestStruct {
            a: 1,
            b: 2,
            c: vec!["hello".to_string()],
        }
        .serialize(&mut expected_data)
        .unwrap();

        TestStruct {
            a: 3,
            b: 4,
            c: vec!["world".to_string()],
        }
        .serialize(&mut expected_data)
        .unwrap();

        assert_eq!(data, expected_data);
    }

    #[test]
    fn struct_deserialize() {
        let mut data: Vec<u8> = vec![];
        leb128::write::unsigned(&mut data, 2).unwrap();

        TestStruct {
            a: 1,
            b: 2,
            c: vec!["hello".to_string()],
        }
        .serialize(&mut data)
        .unwrap();

        TestStruct {
            a: 3,
            b: 4,
            c: vec!["world".to_string()],
        }
        .serialize(&mut data)
        .unwrap();

        let mut reader = &data[..];
        let leb128_vec: LEB128Vec<TestStruct> = BorshDeserialize::deserialize(&mut reader).unwrap();

        assert_eq!(
            leb128_vec,
            LEB128Vec(vec![
                TestStruct {
                    a: 1,
                    b: 2,
                    c: vec!["hello".to_string()],
                },
                TestStruct {
                    a: 3,
                    b: 4,
                    c: vec!["world".to_string()],
                },
            ])
        );
    }
}
