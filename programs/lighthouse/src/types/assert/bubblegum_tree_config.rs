use super::{Assert, EquatableOperator, Evaluate, IntegerOperator, LogLevel};
use crate::generate_asserts_borsh;
use crate::{err, err_msg, error::LighthouseError, utils::Result};
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

        generate_asserts_borsh!(
            self,
            BubblegumTreeConfigAssertion,
            data,
            log_level,
            standard_cases: [
                (TreeCreator, (Pubkey), 8),
                (TreeDelegate, (Pubkey), 40),
                (TotalMintCapacity, u64, 72),
                (NumMinted, u64, 80),
                (IsPublic, bool, 88),
                (IsDecompressible, u8, 89)
            ],
            custom_cases: []
        )
    }
}
