use std::{collections::HashMap, slice::Iter};

use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use num_traits::PrimInt;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
    rent::Rent,
    system_program,
    sysvar::Sysvar,
};

use crate::{
    utils::{create_account, Result},
    validations::{to_checked_account, AccountValidation, MemoryAccount, Program, Signer},
};

enum ArithmeticOverflowBehavior {
    Checked,
    Saturating,
    Wrapping,
}

#[derive(Clone)]
#[repr(u8)]
pub enum RegistrySize {
    U8 = 0b_0000_0000,
    U16 = 0b_0000_0001,
    U32 = 0b_0000_0010,
    U64 = 0b_0000_0011,
    U128 = 0b_0000_0100,
    CheckedU8 = 0b_1000_0000,
    CheckedU16 = 0b_1000_0001,
    CheckedU32 = 0b_1000_0010,
    CheckedU64 = 0b_1000_0011,
    CheckedU128 = 0b_1000_0100,
    SaturatingU8 = 0b_0100_0000,
    SaturatingU16 = 0b_0100_0001,
    SaturatingU32 = 0b_0100_0010,
    SaturatingU64 = 0b_0100_0011,
    SaturatingU128 = 0b_0100_0100,
    WrappingU8 = 0b_1100_0000,
    WrappingU16 = 0b_1100_0001,
    WrappingU32 = 0b_1100_0010,
    WrappingU64 = 0b_1100_0011,
    WrappingU128 = 0b_1100_0100,
}

impl RegistrySize {
    fn is_checked(&self) -> bool {
        (self.clone() as u8) & 0b1000_0000 == 0b1000_0000
    }

    fn is_saturating(&self) -> bool {
        (self.clone() as u8) & 0b0100_0000 == 0b0100_0000
    }

    fn is_wrapping(&self) -> bool {
        (self.clone() as u8) & 0b1100_0000 == 0b1100_0000
    }

    fn strip_flags(&self) -> RegistrySize {
        RegistrySize::from_u8((self.clone() as u8) & 0b0011_1111)
    }

    fn from_u8(value: u8) -> RegistrySize {
        match value {
            0b_0000_0000 => RegistrySize::U8,
            0b_0000_0001 => RegistrySize::U16,
            0b_0000_0010 => RegistrySize::U32,
            0b_0000_0011 => RegistrySize::U64,
            0b_0000_0100 => RegistrySize::U128,
            0b_1000_0000 => RegistrySize::CheckedU8,
            0b_1000_0001 => RegistrySize::CheckedU16,
            0b_1000_0010 => RegistrySize::CheckedU32,
            0b_1000_0011 => RegistrySize::CheckedU64,
            0b_1000_0100 => RegistrySize::CheckedU128,
            0b_0100_0000 => RegistrySize::SaturatingU8,
            0b_0100_0001 => RegistrySize::SaturatingU16,
            0b_0100_0010 => RegistrySize::SaturatingU32,
            0b_0100_0011 => RegistrySize::SaturatingU64,
            0b_0100_0100 => RegistrySize::SaturatingU128,
            0b_1100_0000 => RegistrySize::WrappingU8,
            0b_1100_0001 => RegistrySize::WrappingU16,
            0b_1100_0010 => RegistrySize::WrappingU32,
            0b_1100_0011 => RegistrySize::WrappingU64,
            0b_1100_0100 => RegistrySize::WrappingU128,
            _ => panic!("Invalid registry type"),
        }
    }
}

pub enum Expression {}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Not,
    Shl(u8),
    Shr(u8),
}

#[repr(transparent)]
#[derive(Zeroable, Pod, Clone, Copy)]
pub struct NumericalRegistry<T: Pod + PrimInt> {
    pub registers: [T; 16],
}

impl<T: Pod + PrimInt> NumericalRegistry<T> {
    pub fn set(&mut self, index: u8, value: T) {
        self.registers[index as usize] = value;
    }
}

// #[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
// pub struct CreateMemoryAccountParameters {
//     pub memory_index: u8,
//     pub memory_account_size: u64,
// }

pub(crate) struct SetNumericalRegistryContext<'a, 'info> {
    #[allow(dead_code)]
    pub(crate) lighthouse_program: Program<'a, 'info>,
    pub(crate) payer: Signer<'a, 'info>,
    pub(crate) memory_account: MemoryAccount<'info>,
    pub(crate) system_program: Program<'a, 'info>,
}

impl<'a, 'info> SetNumericalRegistryContext<'a, 'info> {
    pub(crate) fn load(
        account_iter: &mut Iter<'a, AccountInfo<'info>>,
        // parameters: &CreateMemoryAccountParameters,
    ) -> Result<(Self, HashMap<Pubkey, u8>)> {
        let lighthouse_program = Program::new(next_account_info(account_iter)?, &crate::id())?;
        let payer = Signer::new(next_account_info(account_iter)?)?;

        let (memory_account, bump_map) = to_checked_account(
            next_account_info(account_iter)?.clone(),
            &vec![
                AccountValidation::IsEmpty,
                AccountValidation::IsWritable,
                AccountValidation::IsProgramDerivedAddress(
                    &[b"numreg", payer.key.as_ref(), &[0]],
                    crate::id(),
                    None,
                ),
            ],
        )?;

        Ok((
            Self {
                lighthouse_program,
                payer,
                memory_account,
                system_program: Program::new(
                    next_account_info(account_iter)?,
                    &solana_program::system_program::id(),
                )?,
            },
            bump_map,
        ))
    }
}

pub(crate) fn set_numerical_registry(
    context: SetNumericalRegistryContext,
    // parameters: ,
    bump_map: HashMap<Pubkey, u8>,
    size: RegistrySize,
    registry_index: u8,
) -> Result<()> {
    let SetNumericalRegistryContext {
        lighthouse_program: _,
        payer,
        memory_account,
        system_program,
    } = context;

    let overflow_behavior = if size.is_checked() {
        ArithmeticOverflowBehavior::Checked
    } else if size.is_saturating() {
        ArithmeticOverflowBehavior::Saturating
    } else if size.is_wrapping() {
        ArithmeticOverflowBehavior::Wrapping
    } else {
        ArithmeticOverflowBehavior::Checked
    };

    let size = size.strip_flags();

    // // TODO: better error handling
    let bump = *bump_map.get(memory_account.account_info.key).unwrap();

    if memory_account.account_info.owner == &system_program::ID {
        let account_size = match &size {
            RegistrySize::U8 => std::mem::size_of::<u8>() * 16,
            RegistrySize::U16 => std::mem::size_of::<u16>() * 16,
            RegistrySize::U32 => std::mem::size_of::<u32>() * 16,
            RegistrySize::U64 => std::mem::size_of::<u64>() * 16,
            RegistrySize::U128 => std::mem::size_of::<u128>() * 16,
            _ => panic!("Invalid registry type"),
        } as u64;

        create_account(
            payer.as_ref(),
            &memory_account.account_info,
            system_program.as_ref(),
            &crate::id(),
            &Rent::get().unwrap(),
            account_size,
            vec![
                b"memory".to_vec(),
                payer.key.try_to_vec().unwrap(),
                vec![size.clone() as u8, bump],
            ],
        )?;
    }

    let mut data = memory_account.account_info.try_borrow_mut_data()?;

    match size {
        RegistrySize::U8 => {
            let registry: &mut NumericalRegistry<u8> = bytemuck::from_bytes_mut(&mut data);
            registry.set(registry_index, 0);
        }
        RegistrySize::U16 => {
            let registry: &mut NumericalRegistry<u16> = bytemuck::from_bytes_mut(&mut data);
            registry.set(registry_index, 0);
        }
        RegistrySize::U32 => {
            let registry: &mut NumericalRegistry<u32> = bytemuck::from_bytes_mut(&mut data);
            registry.set(registry_index, 0);
        }
        RegistrySize::U64 => {
            let registry: &mut NumericalRegistry<u64> = bytemuck::from_bytes_mut(&mut data);
            registry.set(registry_index, 0);
        }
        RegistrySize::U128 => {
            let registry: &mut NumericalRegistry<u128> = bytemuck::from_bytes_mut(&mut data);
            registry.set(registry_index, 0);
        }
        _ => panic!("Invalid registry type"),
    }

    Ok(())
}
