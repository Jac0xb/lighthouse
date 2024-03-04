use std::{any::type_name, fmt::Debug, ops::Range};

use crate::{
    error::LighthouseError,
    types::{operator::EvaluationResult, Assert},
};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    log, msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

pub type Result<T> = std::result::Result<T, ProgramError>;

pub fn print_assertion_result<U: Debug, T: Assert<U> + Debug>(
    assertion: &T,
    assertion_index: usize,
    evaluation_result: &EvaluationResult,
) {
    msg!(
        // repeating zeros infront of assettion index
        "{} {} {:?} {}",
        format!("[{:0>2}]", assertion_index),
        if evaluation_result.passed {
            "[✓] PASSED"
        } else {
            "[✕] FAILED"
        },
        assertion,
        evaluation_result.output
    );
}

pub fn unpack_coption_key(src: &[u8]) -> Result<COption<Pubkey>> {
    let tag = &src[0..4];
    let body = &src[4..36];

    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(
            body.try_into().unwrap(),
        ))),
        _ => {
            msg!("Failed to deserialize COption<Pubkey> src: {:?}", src);
            Err(LighthouseError::FailedToDeserialize.into())
        }
    }
}

pub fn unpack_coption_u64(src: &[u8]) -> Result<COption<u64>> {
    let tag = &src[0..4];
    let body = &src[4..12];

    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(body.try_into().unwrap()))),
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

        // err!(LighthouseError::RangeOutOfBounds)

        ProgramError::Custom(0)
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

macro_rules! log_compute {
    () => {
        #[cfg(all(feature = "sol-log", feature = "log"))]
        ::solana_program::log::sol_log_compute_units();
    };
}
