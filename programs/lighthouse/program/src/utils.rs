use anchor_lang::prelude::{borsh::BorshDeserialize, *};

use crate::{
    error,
    structs::{BorshField, Operator},
};

pub fn process_value<T: Ord + BorshDeserialize + ToString>(
    data: &[u8],
    offset: u32,
    size: usize,
    expected_value: &T,
    borsh_field: &BorshField,
    operator: &Operator,
) -> Result<(String, String, bool)> {
    let slice = &data[offset as usize..(offset as usize + size)];
    let value = T::try_from_slice(slice).map_err(|_| error::ProgramError::BorshValueMismatch)?;

    borsh_field.is_supported_operator(operator);
    let assertion_result = operator.is_true(&value, expected_value);

    Ok((
        value.to_string(),
        expected_value.to_string(),
        assertion_result,
    ))
}
