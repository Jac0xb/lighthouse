use super::{AccountValidation, CheckedAccount};
use solana_program::account_info::AccountInfo;
use std::ops::Deref;

#[derive(Clone)]
pub(crate) struct Signer<'a, 'info> {
    pub(crate) info: &'a AccountInfo<'info>,
}

impl<'a, 'info> CheckedAccount<'a, 'info> for Signer<'a, 'info>
where
    'info: 'a,
{
    fn get_validations() -> Option<Vec<AccountValidation<'a>>> {
        Some(vec![AccountValidation::IsSigner])
    }

    fn info(&self) -> &'a AccountInfo<'info> {
        self.info
    }

    fn new(account: &'a AccountInfo<'info>) -> Self {
        Self { info: account }
    }
}

impl<'a, 'info> AsRef<AccountInfo<'info>> for Signer<'a, 'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

impl<'a, 'info> Deref for Signer<'a, 'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}
