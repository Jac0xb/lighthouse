use super::{Assert, EvaluationResult, LogLevel};
use crate::{processor::AssertMerkleTreeAccountContext, utils::Result, validation::CheckedAccount};
use anchor_lang::context::CpiContext;
use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt::Debug;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MerkleTreeAssertion {
    VerifyLeaf {
        leaf_index: u32,
        leaf_hash: [u8; 32],
    },
}

impl<'a, 'info> Assert<&AssertMerkleTreeAccountContext<'a, 'info>> for MerkleTreeAssertion {
    fn evaluate(
        &self,
        context: &AssertMerkleTreeAccountContext<'a, 'info>,
        _log_level: LogLevel,
    ) -> Result<Box<EvaluationResult>> {
        let accounts = spl_account_compression::cpi::accounts::VerifyLeaf {
            merkle_tree: context.merkle_tree.clone(),
        };

        match self {
            &MerkleTreeAssertion::VerifyLeaf {
                leaf_index,
                leaf_hash,
            } => {
                let cpi_context =
                    CpiContext::new(context.spl_account_compression.info_as_owned(), accounts)
                        .with_remaining_accounts(context.proof_path.to_vec());

                let result = spl_account_compression::cpi::verify_leaf(
                    cpi_context,
                    context.root.key.to_bytes(),
                    leaf_hash,
                    leaf_index,
                );

                // CPI failing, fails everything so this won't ever happen
                if let Err(e) = result {
                    Ok(Box::new(EvaluationResult {
                        passed: false,
                        output: Some(format!("VerifyLeaf CPI failed: {:?}", e)),
                    }))
                } else {
                    Ok(Box::new(EvaluationResult {
                        passed: true,
                        output: Some("Merkle tree leaf verified".to_string()),
                    }))
                }
            }
        }
    }
}
