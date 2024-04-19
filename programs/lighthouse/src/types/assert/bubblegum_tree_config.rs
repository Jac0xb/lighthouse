use super::{Assert, EquatableOperator, Evaluate, IntegerOperator, LogLevel};
use crate::{err, err_msg, error::lighthausError, utils::Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum BubblegumTreeConfigAssertion {
    TreeCreator {
        value: Pubkey,
        operator: EquatableOperator,
    },
    TreeDelegate {
        value: Pubkey,
        operator: EquatableOperator,
    },
    TotalMintCapacity {
        value: u64,
        operator: IntegerOperator,
    },
    NumMinted {
        value: u64,
        operator: IntegerOperator,
    },
    IsPublic {
        value: bool,
        operator: EquatableOperator,
    },
    IsDecompressible {
        value: u8,
        operator: EquatableOperator,
    },
}

impl<'info> Assert<&AccountInfo<'info>> for BubblegumTreeConfigAssertion {
    fn evaluate(&self, account: &AccountInfo<'info>, log_level: LogLevel) -> Result<()> {
        let data = account.try_borrow_data().map_err(|e| {
            err_msg!("Cannot borrow data for target account", e);
            err!(lighthausError::AccountBorrowFailed)
        })?;

        match self {
            BubblegumTreeConfigAssertion::TreeCreator { value, operator } => {
                let data_slice = data
                    .get(8..40)
                    .ok_or_else(|| lighthausError::oob_err(8..40))?;
                let actual_tree_creator = bytemuck::from_bytes::<Pubkey>(data_slice);

                Pubkey::evaluate(actual_tree_creator, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::TreeDelegate { value, operator } => {
                let data_slice = data
                    .get(40..72)
                    .ok_or_else(|| lighthausError::oob_err(40..72))?;
                let actual_tree_delegate = bytemuck::from_bytes::<Pubkey>(data_slice);

                Pubkey::evaluate(actual_tree_delegate, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::TotalMintCapacity { value, operator } => {
                let data_slice = data
                    .get(72..80)
                    .ok_or_else(|| lighthausError::oob_err(72..80))?;

                let actual_total_mint_capacity = u64::try_from_slice(data_slice).map_err(|e| {
                    err_msg!("Failed to deserialize mint from account data", e);
                    err!(lighthausError::FailedToDeserialize)
                })?;

                u64::evaluate(&actual_total_mint_capacity, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::NumMinted { value, operator } => {
                let data_slice = data
                    .get(80..88)
                    .ok_or_else(|| lighthausError::oob_err(80..88))?;

                let actual_num_minted = u64::try_from_slice(data_slice).map_err(|e| {
                    err_msg!("Failed to deserialize mint from account data", e);
                    err!(lighthausError::FailedToDeserialize)
                })?;

                u64::evaluate(&actual_num_minted, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::IsPublic { value, operator } => {
                let data_slice = data
                    .get(88..89)
                    .ok_or_else(|| lighthausError::oob_err(88..89))?;

                let actual_is_public = bool::try_from_slice(data_slice).map_err(|e| {
                    err_msg!("Failed to deserialize mint from account data", e);
                    err!(lighthausError::FailedToDeserialize)
                })?;

                bool::evaluate(&actual_is_public, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::IsDecompressible { value, operator } => {
                let actual_is_decompressible = data
                    .get(89)
                    .ok_or_else(|| lighthausError::oob_err(89..90))?;

                u8::evaluate(actual_is_decompressible, value, operator, log_level)
            }
        }
    }
}
