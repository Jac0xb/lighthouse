use super::{Assert, LogLevel};
use crate::{
    processor::AssertMerkleTreeAccountContext,
    utils::{anchor_discriminator, Result},
    validation::CheckedAccount,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};
use std::fmt::Debug;

#[repr(u64)]
pub enum SplAccountCompressionInstruction {
    VerifyLeaf = anchor_discriminator("global:verify_leaf"),
}

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
        ctx: &AssertMerkleTreeAccountContext<'a, 'info>,
        _log_level: LogLevel,
    ) -> Result<()> {
        match self {
            &MerkleTreeAssertion::VerifyLeaf {
                leaf_index,
                leaf_hash,
            } => {
                let mut data = vec![0; 76];
                data[..8].copy_from_slice(
                    &(SplAccountCompressionInstruction::VerifyLeaf as u64).to_le_bytes(),
                );
                data[8..40].copy_from_slice(&ctx.root.key.to_bytes());
                data[40..72].copy_from_slice(&leaf_hash);
                data[72..76].copy_from_slice(&leaf_index.to_le_bytes());

                let mut accounts = vec![ctx.merkle_tree.clone()];
                accounts.extend_from_slice(ctx.proof_path);

                let ix = Instruction {
                    program_id: ctx.spl_account_compression.key(),
                    accounts: accounts
                        .iter()
                        .map(|account| AccountMeta::new_readonly(*account.key, false))
                        .collect(),
                    data,
                };

                invoke(&ix, &accounts)?;

                Ok(())
            }
        }
    }
}
