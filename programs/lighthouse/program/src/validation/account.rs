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
        space: usize,
        payer: &'b Signer<'a, 'info>,
        program_owner: &'b Pubkey,
        system_program: &'b Program<'a, 'info, SystemProgram>,
        seeds: &'b Vec<Vec<u8>>,
    },
    Init {
        space: usize,
        payer: &'b Signer<'a, 'info>,
        program_owner: &'b Pubkey,
        system_program: &'b Program<'a, 'info, SystemProgram>,
        seeds: &'b Vec<Vec<u8>>,
    },
    InitIfNeeded {
        space: usize,
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
                system_program.info(),
                program_owner,
                &Rent::get()?,
                space,
                seeds.to_vec(),
            )?;

            Ok(true)
        } else {
            Ok(false)
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
        Self: std::marker::Sized,
    {
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
                    space as u64,
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
                    space as u64,
                    program_owner,
                    system_program,
                    seeds,
                )?;

                if !was_inited {
                    Self::realloc(account_info, payer, system_program, space as u64)?;
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
                    system_program.info(),
                    program_owner,
                    &Rent::get()?,
                    space as u64,
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

        Self::check_conditions(account_info, validations)?;
        Self::check_conditions(account_info, Self::get_validations().as_ref())?;

        Ok(Self::new(account_info))
    }

    fn new_checked(
        account_info: &'a AccountInfo<'info>,
        validations: Option<&Vec<AccountValidation>>,
    ) -> Result<Self>
    where
        Self: std::marker::Sized,
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
    use super::{AccountValidation, CheckedAccount, InitializeType};
    use crate::{
        error::LighthouseError,
        utils::keys_equal,
        validation::{DerivedAddress, Memory, MemorySeeds, Program, Signer},
        Result,
    };
    use solana_sdk::{
        account_info::AccountInfo,
        msg,
        program_memory::sol_memset,
        program_stubs::{set_syscall_stubs, SyscallStubs},
        pubkey::Pubkey,
        rent::{
            Rent, DEFAULT_BURN_PERCENT, DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR,
        },
        signature::Keypair,
        signer::EncodableKeypair,
        system_instruction::{SystemInstruction, MAX_PERMITTED_DATA_LENGTH},
        system_program,
        sysvar::Sysvar,
    };
    use std::slice::from_raw_parts_mut;

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

        let account_info = AccountInfo::new(&key, false, false, lamports, data, &owner, false, 0);

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
                    if account.rent_epoch == 0 {
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
        let data: &mut [u8] = &mut vec![0u8; 8];

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

    struct MockSyscallStubs {}
    impl SyscallStubs for MockSyscallStubs {
        fn sol_invoke_signed(
            &self,
            _instruction: &solana_sdk::instruction::Instruction,
            _account_infos: &[AccountInfo],
            _signers_seeds: &[&[&[u8]]],
        ) -> solana_sdk::entrypoint::ProgramResult {
            let create_ix: SystemInstruction =
                bincode::deserialize(_instruction.data.as_ref()).unwrap();

            match create_ix {
                SystemInstruction::CreateAccount {
                    lamports,
                    space,
                    owner,
                } => {
                    let payer = _account_infos[0].clone();
                    let new_account = _account_infos[1].clone();

                    if !payer.is_signer {
                        msg!("payer is not signer");
                        return Err(0.into());
                    }

                    if !payer.is_writable {
                        msg!("payer is not writable");
                        return Err(1.into());
                    }

                    if !new_account.is_signer {
                        msg!("new account is not signer");
                        return Err(3.into());
                    }

                    if !new_account.is_writable {
                        msg!("new account is not writable");
                        return Err(4.into());
                    }

                    if new_account.executable {
                        msg!("new account is executable");
                        return Err(5.into());
                    }

                    if !new_account.data_is_empty() {
                        msg!("new account is not empty");
                        return Err(6.into());
                    }

                    if !system_program::check_id(new_account.owner) {
                        msg!("new account is not owned by system program");
                        return Err(7.into());
                    }

                    if space > MAX_PERMITTED_DATA_LENGTH {
                        msg!("requested space is too large");
                        return Err(8.into());
                    }

                    msg!("lamports {}", lamports);

                    // transfer lamports from payer to new account
                    **payer.try_borrow_mut_lamports().unwrap() -= lamports;
                    **new_account.try_borrow_mut_lamports().unwrap() += lamports;

                    let mut data = new_account.try_borrow_mut_data().unwrap();
                    unsafe {
                        new_account.assign(&owner);

                        let data_ptr = data.as_mut_ptr();
                        *(data_ptr.offset(-8) as *mut u64) = space;
                        *data = from_raw_parts_mut(data_ptr, space as usize)
                    }

                    sol_memset(&mut data, 0, space as usize);
                }
                SystemInstruction::Assign { owner } => {
                    let account = _account_infos[0].clone();

                    if !account.is_writable {
                        msg!("account is not writable");
                        return Err(9.into());
                    }
                    if account.executable {
                        msg!("account is executable");
                        return Err(10.into());
                    }

                    let data = account.try_borrow_data().unwrap();

                    if !data.iter().all(|&x| x == 0) {
                        msg!("account is not empty");
                        return Err(11.into());
                    }

                    // don't touch the account if the owner does not change
                    if keys_equal(account.owner, &owner) {
                        return Ok(());
                    }

                    account.assign(&owner);
                }
                SystemInstruction::Allocate { space } => {
                    let account = _account_infos[0].clone();

                    if !account.is_writable {
                        msg!("account is not writable");
                        return Err(11.into());
                    }
                    if account.executable {
                        msg!("account is executable");
                        return Err(12.into());
                    }

                    if !account.is_signer {
                        msg!("account is not signer");
                        return Err(13.into());
                    }

                    let mut data = account.try_borrow_mut_data().unwrap();

                    if !data.iter().all(|&x| x == 0) {
                        msg!("account is not empty");
                        return Err(11.into());
                    }

                    unsafe {
                        let data_ptr = data.as_mut_ptr();
                        *(data_ptr.offset(-8) as *mut u64) = space;
                        *data = from_raw_parts_mut(data_ptr, space as usize)
                    }

                    sol_memset(&mut data, 0, space as usize);

                    drop(data)
                }
                SystemInstruction::Transfer { lamports } => {
                    let payer = _account_infos[0].clone();
                    let dest = _account_infos[1].clone();

                    if !payer.is_signer {
                        msg!("payer is not signer");
                        return Err(14.into());
                    }

                    if !payer.is_writable {
                        msg!("payer is not writable");
                        return Err(15.into());
                    }

                    if !dest.is_writable {
                        msg!("dest is not writable");
                        return Err(16.into());
                    }

                    if dest.executable {
                        msg!("dest is executable");
                        return Err(17.into());
                    }

                    let mut src_ref = payer.try_borrow_mut_lamports().unwrap();
                    let mut dest_ref = dest.try_borrow_mut_lamports().unwrap();

                    **src_ref -= lamports;
                    **dest_ref += lamports;

                    drop(src_ref);
                    drop(dest_ref);
                }
                _ => panic!("unexpected instruction"),
            }

            msg!("create_ix: {:?}", create_ix);

            Ok(())
        }

        fn sol_get_rent_sysvar(&self, var_addr: *mut u8) -> u64 {
            unsafe {
                *(var_addr as *mut _ as *mut Rent) = Rent {
                    lamports_per_byte_year: DEFAULT_LAMPORTS_PER_BYTE_YEAR,
                    burn_percent: DEFAULT_BURN_PERCENT,
                    exemption_threshold: DEFAULT_EXEMPTION_THRESHOLD,
                };
            }
            solana_program::entrypoint::SUCCESS
        }
    }

    // Where crashing starts

    #[test]
    fn new_init_checked__init() {
        set_syscall_stubs(Box::new(MockSyscallStubs {}));

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 16];
        let sysprog_account = AccountInfo::new(
            &system_program::ID,
            false,
            false,
            lamports,
            data,
            &system_program::ID,
            true,
            0,
        );
        let sys_program = Program::new(&sysprog_account);

        let lamports = &mut 100_000_000;
        let data: &mut [u8] = &mut vec![0u8; 16];
        let key = Keypair::new().encodable_pubkey();
        let signer_account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            data,
            &system_program::ID,
            false,
            0,
        );
        let signer = Signer::new(&signer_account_info);

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 2048];
        let key = Keypair::new().encodable_pubkey();

        let owner = system_program::id();
        let account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            &mut data[0..0],
            &owner,
            false,
            0,
        );

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::Init {
                payer: &signer,
                space: 120,
                program_owner: &crate::ID,
                system_program: &sys_program,
                seeds: &vec![],
            },
            None,
        )
        .unwrap();

        // Test branching logic where account is already partially funded but not initialized

        assert_eq!(
            account_info.lamports(),
            Rent::get().unwrap().minimum_balance(120)
        );
        assert_eq!(account_info.owner, &crate::ID);
        assert_eq!(account_info.data_len(), 120);

        let lamports = &mut 10_000;
        let data: &mut [u8] = &mut vec![0u8; 2048];
        let key = Keypair::new().encodable_pubkey();

        let owner = system_program::id();
        let account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            &mut data[0..0],
            &owner,
            false,
            0,
        );

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::Init {
                payer: &signer,
                space: 512,
                program_owner: &crate::ID,
                system_program: &sys_program,
                seeds: &vec![],
            },
            None,
        )
        .unwrap();

        assert_eq!(
            account_info.lamports(),
            Rent::get().unwrap().minimum_balance(512)
        );
        assert_eq!(account_info.owner, &crate::ID);
        assert_eq!(account_info.data_len(), 512);
    }

    #[test]
    fn new_init_checked__init_if_needed() {
        set_syscall_stubs(Box::new(MockSyscallStubs {}));

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 8];
        let sysprog_account = AccountInfo::new(
            &system_program::ID,
            false,
            false,
            lamports,
            data,
            &system_program::ID,
            true,
            0,
        );
        let sys_program = Program::new(&sysprog_account);

        let lamports = &mut 100_000_000;
        let data: &mut [u8] = &mut vec![0u8; 8];
        let key = Keypair::new().encodable_pubkey();
        let signer_account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            data,
            &system_program::ID,
            false,
            0,
        );
        let signer = Signer::new(&signer_account_info);

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 2048];
        let key = Keypair::new().encodable_pubkey();

        let owner = system_program::id();
        let account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            &mut data[0..0],
            &owner,
            false,
            0,
        );

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::InitIfNeeded {
                payer: &signer,
                space: 120,
                program_owner: &crate::ID,
                system_program: &sys_program,
                seeds: &vec![],
            },
            None,
        )
        .unwrap();

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::InitIfNeeded {
                payer: &signer,
                space: 120,
                program_owner: &crate::ID,
                system_program: &sys_program,
                seeds: &vec![],
            },
            None,
        )
        .unwrap();
    }

    #[test]
    fn new_init_checked__init_or_realloc_if_needed() {
        set_syscall_stubs(Box::new(MockSyscallStubs {}));

        if std::mem::size_of::<usize>() == 8 {
            println!("Running in a 64-bit environment");
        } else if std::mem::size_of::<usize>() == 4 {
            println!("Running in a 32-bit environment");
        } else {
            println!("Unexpected usize size");
        }

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 8];
        let sysprog_account = AccountInfo::new(
            &system_program::ID,
            false,
            false,
            lamports,
            data,
            &system_program::ID,
            true,
            0,
        );
        let sys_program = Program::new(&sysprog_account);

        let lamports = &mut 100_000_000;
        let data: &mut [u8] = &mut vec![0u8; 8];
        let key = Keypair::new().encodable_pubkey();
        let signer_account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            data,
            &system_program::ID,
            false,
            0,
        );
        let signer = Signer::new(&signer_account_info);

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 2048];
        let key = Keypair::new().encodable_pubkey();

        let owner = system_program::id();
        let account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            &mut data[0..0],
            &owner,
            false,
            0,
        );

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::InitOrReallocIfNeeded {
                payer: &signer,
                space: 120,
                program_owner: &crate::ID,
                system_program: &sys_program,
                seeds: &vec![],
            },
            None,
        )
        .unwrap();

        assert_eq!(
            account_info.lamports(),
            Rent::get().unwrap().minimum_balance(120)
        );
        assert_eq!(account_info.owner, &crate::ID);
        assert_eq!(account_info.data_len(), 120);

        let mut data = account_info.try_borrow_mut_data().unwrap();
        sol_memset(&mut data, 255, 120);

        drop(data);

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::InitOrReallocIfNeeded {
                payer: &signer,
                space: 256,
                program_owner: &crate::ID,
                system_program: &sys_program,
                seeds: &vec![],
            },
            None,
        )
        .unwrap();

        assert_eq!(
            account_info.lamports(),
            Rent::get().unwrap().minimum_balance(256)
        );
        assert_eq!(account_info.owner, &crate::ID);
        assert_eq!(account_info.data_len(), 256);

        let data = account_info.try_borrow_data().unwrap();

        for i in 0..120 {
            assert_eq!(data[i], 255);
        }

        for i in 120..256 {
            assert_eq!(data[i], 0);
        }
    }

    #[test]
    fn new_init_checked__realloc() {
        set_syscall_stubs(Box::new(MockSyscallStubs {}));

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 8];
        let sysprog_account = AccountInfo::new(
            &system_program::ID,
            false,
            false,
            lamports,
            data,
            &system_program::ID,
            true,
            0,
        );
        let sys_program = Program::new(&sysprog_account);

        let lamports = &mut 100_000_000;
        let data: &mut [u8] = &mut vec![0u8; 8];
        let key = Keypair::new().encodable_pubkey();
        let signer_account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            data,
            &system_program::ID,
            false,
            0,
        );
        let signer = Signer::new(&signer_account_info);

        let lamports = &mut 0;
        let data: &mut [u8] = &mut vec![0u8; 2048];
        let key = Keypair::new().encodable_pubkey();

        let owner = system_program::id();
        let account_info = AccountInfo::new(
            &key,
            true,
            true,
            lamports,
            &mut data[0..0],
            &owner,
            false,
            0,
        );

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::Init {
                payer: &signer,
                space: 256,
                program_owner: &crate::ID,
                system_program: &sys_program,
                seeds: &vec![],
            },
            None,
        )
        .unwrap();

        assert_eq!(
            account_info.lamports(),
            Rent::get().unwrap().minimum_balance(256)
        );
        assert_eq!(account_info.owner, &crate::ID);
        assert_eq!(account_info.data_len(), 256);

        TestAccount::new_init_checked(
            &account_info,
            InitializeType::Realloc {
                payer: &signer,
                space: 120,
                system_program: &sys_program,
            },
            None,
        )
        .unwrap();

        assert_eq!(
            account_info.lamports(),
            Rent::get().unwrap().minimum_balance(256)
        );
        assert_eq!(account_info.owner, &crate::ID);
        assert_eq!(account_info.data_len(), 120);
    }
}
