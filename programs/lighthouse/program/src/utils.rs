use std::{any::type_name, ops::Range};

use crate::error::LighthouseError;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_memory::sol_memcmp,
    pubkey::Pubkey,
    pubkey::PUBKEY_BYTES,
    rent::Rent,
    system_instruction, system_program,
};

pub type Result<T> = std::result::Result<T, ProgramError>;

pub fn unpack_coption_key(src: &[u8]) -> Result<Option<Pubkey>> {
    let tag = &src[0..4];
    let body = &src[4..36];

    match *tag {
        [0, 0, 0, 0] => Ok(Option::None),
        [1, 0, 0, 0] => Ok(Option::Some(Pubkey::new_from_array(
            body.try_into().unwrap(),
        ))),
        _ => {
            msg!("Failed to deserialize COption<Pubkey> src: {:?}", src);
            Err(LighthouseError::FailedToDeserialize.into())
        }
    }
}

pub fn unpack_coption_u64(src: &[u8]) -> Result<Option<u64>> {
    let tag = &src[0..4];
    let body = &src[4..12];

    match *tag {
        [0, 0, 0, 0] => Ok(Option::None),
        [1, 0, 0, 0] => Ok(Option::Some(u64::from_le_bytes(body.try_into().unwrap()))),
        _ => {
            msg!("Failed to deserialize COption<u64> src: {:?}", src);
            Err(LighthouseError::FailedToDeserialize.into())
        }
    }
}

pub fn try_from_slice<T: BorshDeserialize + Sized>(
    data: &[u8],
    offset: usize,
    length: Option<usize>,
) -> Result<T> {
    let data_length = length.unwrap_or(std::mem::size_of::<T>());
    let start = offset;
    let end = offset + data_length;

    let slice = data.get(start..end).ok_or_else(|| {
        msg!(
            "Failed to deserialized {} range {:?} was out of bounds",
            type_name::<T>(),
            start..end
        );

        LighthouseError::RangeOutOfBounds
    })?;

    Ok(T::try_from_slice(slice)?)
}

pub fn create_account<'a, 'info>(
    payer: &'a AccountInfo<'info>,
    new_account: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    program_owner: &Pubkey,
    rent: &Rent,
    space: u64,
    seeds: Vec<Vec<u8>>,
) -> ProgramResult {
    let current_lamports = **new_account.try_borrow_lamports()?;
    if current_lamports == 0 {
        // If there are no lamports in the new account, we create it with the create_account instruction
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                new_account.key,
                rent.minimum_balance(space as usize),
                space,
                program_owner,
            ),
            &[payer.clone(), new_account.clone(), system_program.clone()],
            &[seeds
                .iter()
                .map(|seed| seed.as_slice())
                .collect::<Vec<&[u8]>>()
                .as_slice()],
        )
    } else {
        // Fund the account for rent exemption.
        let required_lamports = rent
            .minimum_balance(space as usize)
            .max(1)
            .saturating_sub(current_lamports);
        if required_lamports > 0 {
            invoke(
                &system_instruction::transfer(payer.key, new_account.key, required_lamports),
                &[payer.clone(), new_account.clone(), system_program.clone()],
            )?;
        }
        // Allocate space.
        invoke_signed(
            &system_instruction::allocate(new_account.key, space),
            &[new_account.clone(), system_program.clone()],
            &[seeds
                .iter()
                .map(|seed| seed.as_slice())
                .collect::<Vec<&[u8]>>()
                .as_slice()],
        )?;
        // Assign to the specified program
        invoke_signed(
            &system_instruction::assign(new_account.key, program_owner),
            &[new_account.clone(), system_program.clone()],
            &[seeds
                .iter()
                .map(|seed| seed.as_slice())
                .collect::<Vec<&[u8]>>()
                .as_slice()],
        )
    }
}

pub fn out_of_bounds_err(r: Range<usize>) -> ProgramError {
    msg!("Failed to access account data range {:?}: out of bounds", r);
    LighthouseError::RangeOutOfBounds.into()
}

pub fn close<'info>(info: AccountInfo<'info>, sol_destination: AccountInfo<'info>) -> Result<()> {
    // Transfer tokens from the account to the sol_destination.
    let dest_starting_lamports = sol_destination.lamports();
    **sol_destination.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(info.lamports()).unwrap();
    **info.lamports.borrow_mut() = 0;

    info.assign(&system_program::ID);
    info.realloc(0, false).map_err(Into::into)
}

pub fn is_closed(info: &AccountInfo) -> bool {
    keys_equal(info.owner, &system_program::id()) && info.data_is_empty()
}

pub fn keys_equal(key_a: &Pubkey, key_b: &Pubkey) -> bool {
    sol_memcmp(key_a.as_ref(), key_b.as_ref(), PUBKEY_BYTES) == 0
}

pub fn contains_key(key: &Pubkey, keys: &[&Pubkey]) -> bool {
    keys.iter().any(|k| keys_equal(k, key))
}
