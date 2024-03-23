use super::{Program, Signer, SystemProgram};
use crate::{
    error::LighthouseError,
    utils::{create_account, keys_equal, Result},
};
use solana_program::{
    account_info::AccountInfo, msg, program::invoke, pubkey::Pubkey, rent::Rent,
    system_instruction, system_program, sysvar::Sysvar,
};

type ValidationFn = Box<dyn Fn(&AccountInfo) -> Result<()> + 'static>;

#[allow(dead_code)]
pub(crate) enum AccountValidation<'a> {
    IsWritable,
    IsSigner,
    IsNotOwned,
    IsProgramOwned(Pubkey),
    IsProgramDerivedAddress {
        seeds: &'a Vec<Vec<u8>>,
        program_id: &'a Pubkey,
        find_bump: bool,
    },
    KeyEquals(Pubkey),
    CustomValidation(ValidationFn),
}

#[allow(dead_code)]
pub(crate) enum InitializeType<'b, 'a, 'info> {
    InitOrReallocIfNeeded {
        space: u64,
        payer: &'b Signer<'a, 'info>,
        program_owner: &'b Pubkey,
        system_program: &'b Program<'a, 'info, SystemProgram>,
        seeds: &'b Vec<Vec<u8>>,
    },
    Init {
        space: u64,
        payer: &'b Signer<'a, 'info>,
        program_owner: &'b Pubkey,
        system_program: &'b Program<'a, 'info, SystemProgram>,
        seeds: &'b Vec<Vec<u8>>,
    },
    InitIfNeeded {
        space: u64,
        payer: &'b Signer<'a, 'info>,
        program_owner: &'b Pubkey,
        system_program: &'b Program<'a, 'info, SystemProgram>,
        seeds: &'b Vec<Vec<u8>>,
    },
    Realloc {
        space: u64,
        system_program: &'b Program<'a, 'info, SystemProgram>,
        payer: &'b Signer<'a, 'info>,
    },
}

pub(crate) trait CheckedAccount<'a, 'info: 'a> {
    fn new(account: &'a AccountInfo<'info>) -> Self;
    fn info(&self) -> &'a AccountInfo<'info>;
    fn get_validations() -> Option<Vec<AccountValidation<'a>>>;

    fn info_as_owned(&self) -> AccountInfo<'info> {
        self.info().clone()
    }

    fn key(&self) -> Pubkey {
        *self.info().key
    }

    fn init_if_needed(
        account_info: &'a AccountInfo<'info>,
        payer: &'_ Signer<'a, 'info>,
        space: u64,
        program_owner: &'_ Pubkey,
        system_program: &'_ Program<'a, 'info, SystemProgram>,
        seeds: &'_ [Vec<u8>],
    ) -> Result<bool> {
        if keys_equal(account_info.owner, &system_program.key()) {
            create_account(
                payer.as_ref(),
                account_info,
                system_program.info,
                program_owner,
                &Rent::get()?,
                space,
                seeds.to_vec(),
            )?;

            Ok(true)
        } else if keys_equal(account_info.owner, program_owner) {
            Ok(false)
        } else {
            msg!("Unexpected program owner");
            Err(LighthouseError::AccountValidationFailed.into())
        }
    }

    fn realloc(
        account_info: &'_ AccountInfo<'info>,
        payer: &'_ Signer<'a, 'info>,
        system_program: &'_ Program<'a, 'info, SystemProgram>,
        space: u64,
    ) -> Result<()> {
        let current_lamports = **account_info.try_borrow_lamports()?;
        let required_lamports = Rent::get()?
            .minimum_balance(space as usize)
            .max(1)
            .saturating_sub(current_lamports);
        if required_lamports > 0 {
            invoke(
                &system_instruction::transfer(payer.key, account_info.key, required_lamports),
                &[
                    payer.info_as_owned(),
                    account_info.clone(),
                    system_program.info_as_owned(),
                ],
            )?;
        }

        account_info.realloc(space as usize, false)
    }

    fn new_init_checked(
        account_info: &'a AccountInfo<'info>,
        init_type: InitializeType<'_, 'a, 'info>,
        validations: Option<&Vec<AccountValidation>>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::check_conditions(account_info, validations)?;
        Self::check_conditions(account_info, Self::get_validations().as_ref())?;

        match init_type {
            InitializeType::InitIfNeeded {
                payer,
                space,
                program_owner,
                system_program,
                seeds,
            } => {
                Self::init_if_needed(
                    account_info,
                    payer,
                    space,
                    program_owner,
                    system_program,
                    seeds,
                )?;
            }
            InitializeType::InitOrReallocIfNeeded {
                payer,
                space,
                program_owner,
                system_program,
                seeds,
            } => {
                let was_inited = Self::init_if_needed(
                    account_info,
                    payer,
                    space,
                    program_owner,
                    system_program,
                    seeds,
                )?;

                if !was_inited {
                    Self::realloc(account_info, payer, system_program, space)?;
                }
            }
            InitializeType::Init {
                payer,
                space,
                program_owner,
                system_program,
                seeds,
            } => {
                create_account(
                    payer.as_ref(),
                    account_info,
                    system_program.info,
                    program_owner,
                    &Rent::get()?,
                    space,
                    seeds.to_vec(),
                )?;
            }
            InitializeType::Realloc {
                system_program,
                payer,
                space,
            } => {
                Self::realloc(account_info, payer, system_program, space)?;
            }
        }

        Ok(Self::new(account_info))
    }

    fn new_checked(
        account_info: &'a AccountInfo<'info>,
        validations: Option<&Vec<AccountValidation>>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::check_conditions(account_info, validations)?;
        Self::check_conditions(account_info, Self::get_validations().as_ref())?;
        Ok(Self::new(account_info))
    }

    fn check_conditions(
        account: &AccountInfo<'info>,
        validations: Option<&Vec<AccountValidation>>,
    ) -> Result<()> {
        if let Some(validations) = validations {
            for validation in validations {
                match validation {
                    AccountValidation::IsWritable => {
                        if account.is_writable {
                        } else {
                            msg!("account is writable condition failed: {:?}", account.key);
                            return Err(LighthouseError::AccountValidationFailed.into());
                        }
                    }
                    AccountValidation::IsSigner => {
                        if account.is_signer {
                        } else {
                            msg!("account is signer condition failed: {:?}", account.key);
                            return Err(LighthouseError::AccountValidationFailed.into());
                        }
                    }
                    AccountValidation::IsProgramOwned(owner) => {
                        if account.lamports() != 0 && keys_equal(account.owner, owner) {
                        } else {
                            msg!("account inited condition failed: {:?}", account.key);
                            return Err(LighthouseError::AccountValidationFailed.into());
                        }
                    }
                    AccountValidation::IsNotOwned => {
                        if account.lamports() == 0 && keys_equal(account.owner, &system_program::ID)
                        {
                        } else {
                            msg!("account not inited condition failed: {:?}", account.key);
                            return Err(LighthouseError::AccountValidationFailed.into());
                        }
                    }
                    AccountValidation::IsProgramDerivedAddress {
                        seeds,
                        program_id,
                        find_bump,
                    } => {
                        if *find_bump {
                            let seeds =
                                seeds.iter().map(|seed| seed.as_slice()).collect::<Vec<_>>();

                            let (generated_pda, _) =
                                Pubkey::find_program_address(seeds.as_slice(), program_id);

                            if !keys_equal(account.key, &generated_pda) {
                                msg!(
                                "program derived address condition failed: expected {:?} actual {:?}",
                                account.key,
                                generated_pda
                            );

                                return Err(LighthouseError::AccountValidationFailed.into());
                            }
                        } else {
                            let derived_address = Pubkey::create_program_address(
                                seeds
                                    .iter()
                                    .map(|seed| seed.as_slice())
                                    .collect::<Vec<&[u8]>>()
                                    .as_slice(),
                                program_id,
                            )
                            .map_err(|err| {
                                msg!("failed to create program address: {:?} {:?}", seeds, err);
                                LighthouseError::AccountValidationFailed
                            })?;

                            if !keys_equal(account.key, &derived_address) {
                                msg!(
                                "program derived address condition failed: expected {:?} actual {:?}",
                                account.key,
                                derived_address
                            );
                                return Err(LighthouseError::AccountValidationFailed.into());
                            }
                        }
                    }
                    AccountValidation::KeyEquals(key) => {
                        if keys_equal(account.key, key) {
                        } else {
                            msg!(
                                "account key condition failed: expected {:?} actual {:?}",
                                account.key,
                                key
                            );
                            return Err(LighthouseError::AccountValidationFailed.into());
                        }
                    }
                    AccountValidation::CustomValidation(condition) => {
                        condition(account)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[allow(non_snake_case)]
#[allow(clippy::useless_vec)]
#[cfg(test)]
mod tests {
    use super::{AccountValidation, CheckedAccount};
    use crate::{
        error::LighthouseError,
        validation::{DerivedAddress, Memory, MemorySeeds},
        Result,
    };
    use solana_sdk::{
        account_info::AccountInfo, pubkey::Pubkey, signature::Keypair, signer::EncodableKeypair,
        system_program,
    };

    struct TestAccount<'a, 'info> {
        account: &'a AccountInfo<'info>,
    }

    impl<'a, 'info> CheckedAccount<'a, 'info> for TestAccount<'a, 'info> {
        fn new(account: &'a AccountInfo<'info>) -> Self {
            Self { account }
        }

        fn get_validations() -> Option<Vec<AccountValidation<'a>>> {
            None
        }

        fn info(&self) -> &'a AccountInfo<'info> {
            self.account
        }
    }

    #[test]
    fn new_checked() {
        let min_rent = 80_000;
        let key = system_program::id();
        let owner = Keypair::new().encodable_pubkey();
        let lamports = &mut min_rent.clone();
        let data: &mut [u8] = &mut vec![0u8; 8];

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &owner, false, 100);

        TestAccount::new_checked(
            &account_info,
            Some(&vec![
                AccountValidation::KeyEquals(key),
                AccountValidation::IsProgramOwned(owner),
            ]),
        )
        .unwrap();

        let lamports = &mut 0;
        let account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            data,
            &system_program::ID,
            true,
            0,
        );

        TestAccount::new_checked(
            &account_info,
            Some(&vec![
                AccountValidation::IsSigner,
                AccountValidation::IsWritable,
                AccountValidation::IsNotOwned,
                AccountValidation::CustomValidation(Box::new(|account| -> Result<()> {
                    if account.rent_epoch == 100 {
                        Ok(())
                    } else {
                        Err(LighthouseError::AccountValidationFailed.into())
                    }
                })),
            ]),
        )
        .unwrap();
    }

    #[allow(clippy::useless_vec)]
    #[allow(clippy::type_complexity)]
    fn find_memory_pda(
        payer: &Pubkey,
        memory_id: u8,
    ) -> Result<(Pubkey, u8, Vec<Vec<u8>>, Vec<Vec<u8>>)> {
        let seeds = Memory::get_seeds(MemorySeeds {
            payer,
            memory_id,
            bump: None,
        });

        let (key, bump) = Pubkey::find_program_address(
            seeds
                .iter()
                .map(|seed| seed.as_slice())
                .collect::<Vec<_>>()
                .as_slice(),
            &crate::id(),
        );

        let seeds_without_bump = seeds.clone();

        let seeds = seeds
            .iter()
            .chain(vec![vec![bump]].iter())
            .map(|seed| seed.to_vec())
            .collect::<Vec<_>>();

        Ok((key, bump, seeds_without_bump, seeds))
    }

    #[test]
    fn new_checked_pda() {
        let payer = Keypair::new().encodable_pubkey();
        let owner = Keypair::new().encodable_pubkey();

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 0];

        let (key, _bump, seeds_without_bump, seeds) = find_memory_pda(&payer, 0).unwrap();

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &owner, false, 0);

        TestAccount::new_checked(
            &account_info,
            Some(&vec![AccountValidation::IsProgramDerivedAddress {
                seeds: &seeds_without_bump,
                program_id: &crate::id(),
                find_bump: true,
            }]),
        )
        .unwrap();

        TestAccount::new_checked(
            &account_info,
            Some(&vec![AccountValidation::IsProgramDerivedAddress {
                seeds: &seeds,
                program_id: &crate::id(),
                find_bump: false,
            }]),
        )
        .unwrap();

        let fake_payer = Keypair::new().encodable_pubkey();
        let (_key, _bump, seeds_without_bump, seeds) = find_memory_pda(&fake_payer, 0).unwrap();
        let result = TestAccount::new_checked(
            &account_info,
            Some(&vec![AccountValidation::IsProgramDerivedAddress {
                seeds: &seeds_without_bump,
                program_id: &crate::id(),
                find_bump: true,
            }]),
        );

        if let Err(err) = result {
            assert_eq!(err, LighthouseError::AccountValidationFailed.into());
        } else {
            panic!("expected error");
        }

        let result = TestAccount::new_checked(
            &account_info,
            Some(&vec![AccountValidation::IsProgramDerivedAddress {
                seeds: &seeds,
                program_id: &crate::id(),
                find_bump: false,
            }]),
        );

        if let Err(err) = result {
            assert_eq!(err, LighthouseError::AccountValidationFailed.into());
        } else {
            panic!("expected error");
        }
    }
}
