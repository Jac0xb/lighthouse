use super::{AccountValidation, CheckedAccount, Id};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use std::{marker::PhantomData, ops::Deref};

pub struct Program<'a, 'info, T: Id> {
    info: &'a AccountInfo<'info>,
    _phantom: PhantomData<T>,
}

impl<'a, 'info, T: Id> CheckedAccount<'a, 'info> for Program<'a, 'info, T>
where
    'info: 'a,
{
    fn get_validations() -> Vec<AccountValidation<'a>> {
        vec![AccountValidation::KeyEquals(T::id())]
    }

    fn info(&self) -> &'a AccountInfo<'info> {
        self.info
    }

    fn new(account: &'a AccountInfo<'info>) -> Self {
        Self {
            info: account,
            _phantom: PhantomData,
        }
    }
}

impl<'a, 'info, T: Id> AsRef<AccountInfo<'info>> for Program<'a, 'info, T> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

impl<'a, 'info, T: Id> Deref for Program<'a, 'info, T> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}

pub struct LighthouseProgram;
impl Id for LighthouseProgram {
    fn id() -> Pubkey {
        crate::ID
    }
}

pub struct SplAccountCompressionProgram;
impl Id for SplAccountCompressionProgram {
    fn id() -> Pubkey {
        spl_account_compression::ID
    }
}

pub struct SystemProgram;

impl Id for SystemProgram {
    fn id() -> Pubkey {
        solana_program::system_program::ID
    }
}
