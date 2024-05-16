use super::{Assert, EquatableOperator, Evaluate, IntegerOperator, LogLevel};
use crate::{
    err, err_msg,
    error::LighthouseError,
    utils::{try_from_slice, try_from_slice_pubkey, Result},
};
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
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        match self {
            BubblegumTreeConfigAssertion::TreeCreator { value, operator } => {
                let actual_tree_creator = try_from_slice_pubkey(&data, 8)?;
                Pubkey::evaluate(actual_tree_creator, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::TreeDelegate { value, operator } => {
                let actual_tree_delegate = try_from_slice_pubkey(&data, 40)?;
                Pubkey::evaluate(actual_tree_delegate, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::TotalMintCapacity { value, operator } => {
                let actual_total_mint_capacity = try_from_slice(&data, 72, None)?;
                u64::evaluate(&actual_total_mint_capacity, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::NumMinted { value, operator } => {
                let actual_num_minted = try_from_slice(&data, 80, None)?;
                u64::evaluate(&actual_num_minted, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::IsPublic { value, operator } => {
                let actual_is_public = try_from_slice(&data, 88, None)?;
                bool::evaluate(&actual_is_public, value, operator, log_level)
            }
            BubblegumTreeConfigAssertion::IsDecompressible { value, operator } => {
                let actual_is_decompressible: u8 = try_from_slice(&data, 89, None)?;
                u8::evaluate(&actual_is_decompressible, value, operator, log_level)
            }
        }
    }
}
