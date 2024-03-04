use std::fmt::Debug;

use crate::{
    error::LighthouseError,
    types::{Assert, EvaluationResult, LogLevel},
    utils::Result,
    validations::Program,
};
use anchor_lang::{context::CpiContext, ToAccountInfo};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, msg};

pub(crate) struct AssertMerkleLeafContext<'a, 'info> {
    pub(crate) merkle_tree: AccountInfo<'info>,
    pub(crate) root: AccountInfo<'info>,
    pub(crate) spl_account_compression: Program<'a, 'info>,
    pub(crate) proof_path: &'a [AccountInfo<'info>],
}

impl<'a, 'info> AssertMerkleLeafContext<'a, 'info> {
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

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct AssertMerkleLeafParameters {
    pub leaf_index: u32,
    pub leaf_hash: [u8; 32],
}

pub(crate) fn assert_merkle_leaf<'a, 'info, T: Assert<()> + Debug>(
    context: &AssertMerkleLeafContext<'a, 'info>,
    parameters: &AssertMerkleLeafParameters,
    assertion: &T,
    log_level: &LogLevel,
) -> Result<()> {
    let accounts = spl_account_compression::cpi::accounts::VerifyLeaf {
        merkle_tree: context.merkle_tree.clone(),
    };

    let cpi_context = CpiContext::new(context.spl_account_compression.to_account_info(), accounts)
        .with_remaining_accounts(context.proof_path.to_vec());

    let result = spl_account_compression::cpi::verify_leaf(
        cpi_context,
        context.root.key.to_bytes(),
        parameters.leaf_hash,
        parameters.leaf_index,
    );

    if let Err(e) = result {
        msg!("Merkle leaf assertion failed: {:?}", e);
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}

impl Assert<()> for () {
    fn evaluate(&self, _context: &(), log_level: &LogLevel) -> Result<Box<EvaluationResult>> {
        Ok(Box::new(EvaluationResult {
            passed: true,
            output: "No assertion to evaluate".to_string(),
        }))
    }
}
