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

pub const MPL_BUBBLEGUM_ID: Pubkey = Pubkey::new_from_array([
    152, 139, 128, 235, 121, 53, 40, 105, 178, 36, 116, 95, 89, 221, 191, 138, 38, 88, 202, 19,
    220, 104, 129, 33, 38, 53, 28, 174, 7, 193, 165, 165,
]);

pub const SPL_ACCOUNT_COMPRESSION_ID: Pubkey = Pubkey::new_from_array([
    9, 42, 19, 238, 149, 196, 28, 186, 8, 166, 127, 90, 198, 126, 141, 247, 225, 218, 17, 98, 94,
    29, 100, 19, 127, 143, 79, 35, 131, 3, 127, 20,
]);

pub const SPL_NOOP_ID: Pubkey = Pubkey::new_from_array([
    11, 188, 15, 192, 187, 71, 202, 47, 116, 196, 17, 46, 148, 171, 19, 207, 163, 198, 52, 229,
    220, 23, 234, 203, 3, 205, 26, 35, 205, 126, 120, 124,
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
