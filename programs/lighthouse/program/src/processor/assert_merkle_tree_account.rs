use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::Result,
    validations::Program,
};
use solana_program::{account_info::AccountInfo, msg};

pub(crate) struct AssertMerkleTreeAccountContext<'a, 'info> {
    pub(crate) merkle_tree: AccountInfo<'info>,
    pub(crate) root: AccountInfo<'info>,
    pub(crate) spl_account_compression: Program<'a, 'info>,
    pub(crate) proof_path: &'a [AccountInfo<'info>],
}

impl<'a, 'info> AssertMerkleTreeAccountContext<'a, 'info> {
    pub(crate) fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self> {
        Ok(Self {
            merkle_tree: accounts.first().unwrap().to_owned(),
            root: accounts.get(1).unwrap().to_owned(),
            spl_account_compression: Program::new(
                accounts.get(2).unwrap(),
                &spl_account_compression::id(),
            )?,
            proof_path: accounts.get(3..).unwrap(),
        })
    }
}

pub(crate) fn assert_merkle_tree_account<
    'a,
    'info,
    T: Assert<AssertMerkleTreeAccountContext<'a, 'info>>,
>(
    context: &AssertMerkleTreeAccountContext<'a, 'info>,
    assertion: &T,
    log_level: &LogLevel,
) -> Result<()> {
    let result = assertion.evaluate(context, log_level)?;

    if !result.passed {
        msg!("Merkle tree assertion failed: {:?}", result.output);
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
