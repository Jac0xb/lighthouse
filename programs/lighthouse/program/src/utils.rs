use crate::{
    error::LighthouseError,
    types::{operator::EvaluationResult, Assert},
};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

pub type Result<T> = std::result::Result<T, ProgramError>;

pub fn print_assertion_result<U, T: Assert<U>>(
    assertion: &T,
    assertion_index: usize,
    evaluation_result: &EvaluationResult,
) {
    msg!(
        // repeating zeros infront of assettion index
        "{} {} {} {}",
        format!("[{:0>2}]", assertion_index),
        if evaluation_result.passed {
            "[✓] PASSED"
        } else {
            "[✕] FAILED"
        },
        assertion.format(),
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
        _ => Err(LighthouseError::AccountNotInitialized.into()),
    }
}

pub fn unpack_coption_u64(src: &[u8]) -> Result<COption<u64>> {
    let tag = &src[0..4];
    let body = &src[4..12];

    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(body.try_into().unwrap()))),
        _ => Err(LighthouseError::AccountNotInitialized.into()),
    }
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

// if #if_needed {
//     #owner_optional_check
//     if space != actual_field.data_len() {
//         return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSpace).with_account_name(#name_str).with_values((space, actual_field.data_len())));
//     }

//     if actual_owner != #owner {
//         return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintOwner).with_account_name(#name_str).with_pubkeys((*actual_owner, *#owner)));
//     }

//     {
//         let required_lamports = __anchor_rent.minimum_balance(space);
//         if pa.to_account_info().lamports() < required_lamports {
//             return Err(anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt).with_account_name(#name_str));
//         }
//     }
// }

// fn generate_create_account(
//     field: &Ident,
//     space: proc_macro2::TokenStream,
//     owner: proc_macro2::TokenStream,
//     payer: proc_macro2::TokenStream,
//     seeds_with_nonce: proc_macro2::TokenStream,
// ) -> proc_macro2::TokenStream {
//     // Field, payer, and system program are already validated to not be an Option at this point
//     quote! {
//         // If the account being initialized already has lamports, then
//         // return them all back to the payer so that the account has
//         // zero lamports when the system program's create instruction
//         // is eventually called.
//         let __current_lamports = #field.lamports();
//         if __current_lamports == 0 {
//             // Create the token account with right amount of lamports and space, and the correct owner.
//             let space = #space;
//             let lamports = __anchor_rent.minimum_balance(space);
//             let cpi_accounts = anchor_lang::system_program::CreateAccount {
//                 from: #payer.to_account_info(),
//                 to: #field.to_account_info()
//             };
//             let cpi_context = anchor_lang::context::CpiContext::new(system_program.to_account_info(), cpi_accounts);
//             anchor_lang::system_program::create_account(cpi_context.with_signer(&[#seeds_with_nonce]), lamports, space as u64, #owner)?;
//         } else {
//             require_keys_neq!(#payer.key(), #field.key(), anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount);
//             // Fund the account for rent exemption.
//             let required_lamports = __anchor_rent
//                 .minimum_balance(#space)
//                 .max(1)
//                 .saturating_sub(__current_lamports);
//             if required_lamports > 0 {
//                 let cpi_accounts = anchor_lang::system_program::Transfer {
//                     from: #payer.to_account_info(),
//                     to: #field.to_account_info(),
//                 };
//                 let cpi_context = anchor_lang::context::CpiContext::new(system_program.to_account_info(), cpi_accounts);
//                 anchor_lang::system_program::transfer(cpi_context, required_lamports)?;
//             }
//             // Allocate space.
//             let cpi_accounts = anchor_lang::system_program::Allocate {
//                 account_to_allocate: #field.to_account_info()
//             };
//             let cpi_context = anchor_lang::context::CpiContext::new(system_program.to_account_info(), cpi_accounts);
//             anchor_lang::system_program::allocate(cpi_context.with_signer(&[#seeds_with_nonce]), #space as u64)?;
//             // Assign to the spl token program.
//             let cpi_accounts = anchor_lang::system_program::Assign {
//                 account_to_assign: #field.to_account_info()
//             };
//             let cpi_context = anchor_lang::context::CpiContext::new(system_program.to_account_info(), cpi_accounts);
//             anchor_lang::system_program::assign(cpi_context.with_signer(&[#seeds_with_nonce]), #owner)?;
//         }
//     }
// }
