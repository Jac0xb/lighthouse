use crate::{
    types::assert::{Assert, LogLevel},
    utils::Result,
    validation::{CheckedAccount, Program, SplAccountCompressionProgram},
};
use solana_program::{account_info::next_account_info, account_info::AccountInfo};
use std::slice::Iter;

pub(crate) struct AssertMerkleTreeAccountContext<'a, 'info> {
    pub(crate) merkle_tree: &'a AccountInfo<'info>,
    pub(crate) root: &'a AccountInfo<'info>,
    pub(crate) spl_account_compression: Program<'a, 'info, SplAccountCompressionProgram>,
    pub(crate) proof_path: &'a [AccountInfo<'info>],
}

impl<'a, 'info> AssertMerkleTreeAccountContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            merkle_tree: next_account_info(account_iter)?,
            root: next_account_info(account_iter)?,
            spl_account_compression: Program::new_checked(next_account_info(account_iter)?, None)?,
            proof_path: account_iter.as_slice(),
        })
    }
}

pub(crate) fn assert_merkle_tree_account<
    'a,
    'info,
    T: Assert<&'a AssertMerkleTreeAccountContext<'a, 'info>>,
>(
    ctx: &'a AssertMerkleTreeAccountContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    assertion.evaluate(ctx, log_level)
}
