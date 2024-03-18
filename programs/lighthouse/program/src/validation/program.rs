use super::{AccountValidation, CheckedAccount, Id};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use std::{marker::PhantomData, ops::Deref};

pub(crate) struct Program<'a, 'info, T: Id> {
    pub(crate) info: &'a AccountInfo<'info>,
    _phantom: PhantomData<T>,
}

impl<'a, 'info, T: Id> CheckedAccount<'a, 'info> for Program<'a, 'info, T>
where
    'info: 'a,
{
    fn get_validations() -> Option<Vec<AccountValidation<'a>>> {
        Some(vec![AccountValidation::KeyEquals(T::id())])
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

pub const SPL_ACCOUNT_COMPRESSION_ID: Pubkey = Pubkey::new_from_array([
    9u8, 42u8, 19u8, 238u8, 149u8, 196u8, 28u8, 186u8, 8u8, 166u8, 127u8, 90u8, 198u8, 126u8,
    141u8, 247u8, 225u8, 218u8, 17u8, 98u8, 94u8, 29u8, 100u8, 19u8, 127u8, 143u8, 79u8, 35u8,
    131u8, 3u8, 127u8, 20u8,
]);

pub struct SplAccountCompressionProgram;
impl Id for SplAccountCompressionProgram {
    fn id() -> Pubkey {
        SPL_ACCOUNT_COMPRESSION_ID
    }
}

pub struct SystemProgram;

impl Id for SystemProgram {
    fn id() -> Pubkey {
        solana_program::system_program::ID
    }
}
