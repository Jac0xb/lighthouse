use crate::{
    constants::{CANNOT_BORROW_DATA_TARGET_ERROR_MSG, FAILED_DESERIALIZE_STAKE_ACCOUNT_ERROR_MSG},
    err, err_msg,
    error::LighthouseError,
    types::{Assert, EvaluationResult, Operator},
};
use crate::{
    types::{ComparableOperator, EquatableOperator, IntegerOperator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    stake::state::{Meta, Stake, StakeStateV2},
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
#[repr(u8)]
pub enum StakeAccountState {
    Uninitialized = 0,
    Initialized = 1,
    Stake = 2,
    RewardsPool = 3,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum StakeAccountAssertion {
    State {
        value: u8,
        operator: ComparableOperator,
    },
    MetaAssertion(MetaAssertion),
    StakeAssertion(StakeAssertion),
    StakeFlags {
        value: u8,
        operator: IntegerOperator,
    },
}

impl Assert<AccountInfo<'_>> for StakeAccountAssertion {
    fn format(&self) -> String {
        format!("StakeAccountAssertion[{:?}]", self)
    }

    fn evaluate(
        &self,
        account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        if account.data_is_empty() {
            return Err(LighthouseError::AccountNotInitialized.into());
        }

        if ![solana_program::stake::program::id()].contains(account.owner) {
            return Err(LighthouseError::AccountOwnerMismatch.into());
        }

        // TODO: Logic to assert on if account is a mint account
        let data = account.try_borrow_data().map_err(|e| {
            msg!("{}: {}", CANNOT_BORROW_DATA_TARGET_ERROR_MSG, e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        let result = match self {
            StakeAccountAssertion::State { value, operator } => {
                let actual_state: u8 = data[0];

                if actual_state > 4 {
                    msg!(
                        "{} enum out of bounds",
                        FAILED_DESERIALIZE_STAKE_ACCOUNT_ERROR_MSG
                    );
                    return Err(LighthouseError::FailedToDeserialize.into());
                }

                operator.evaluate(&actual_state, value, include_output)
            }
            StakeAccountAssertion::MetaAssertion(meta_assertion) => {
                let stake_account = StakeStateV2::deserialize(&mut data.as_ref()).map_err(|e| {
                    err_msg!("Failed to deserialize stake account state", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?;

                match stake_account {
                    StakeStateV2::Initialized(meta) | StakeStateV2::Stake(meta, _, _) => {
                        meta_assertion.evaluate(&meta, include_output)?
                    }
                    _ => Box::new(EvaluationResult {
                        passed: false,
                        output: "Stake account is not in a state that has meta field".to_string(),
                    }),
                }
            }
            StakeAccountAssertion::StakeAssertion(stake_assertion) => {
                let stake_account = StakeStateV2::deserialize(&mut data.as_ref()).map_err(|e| {
                    msg!("Failed to deserialize stake account state {}", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?;

                match stake_account {
                    StakeStateV2::Stake(_, stake, _) => {
                        stake_assertion.evaluate(&stake, include_output)?
                    }
                    _ => Box::new(EvaluationResult {
                        passed: false,
                        output: "Stake account is not in a state that has stake field".to_string(),
                    }),
                }
            }
            StakeAccountAssertion::StakeFlags { value, operator } => {
                let stake_account = StakeStateV2::deserialize(&mut data.as_ref()).map_err(|e| {
                    msg!("Failed to deserialize stake account state {}", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?;

                match stake_account {
                    StakeStateV2::Stake(_, _, actual_stake_flags) => {
                        // No way to access stake flags directly, serialize and use the raw bytes
                        let serialized_stake_flag = actual_stake_flags.try_to_vec()?;
                        let actual_stake_flag = serialized_stake_flag[0];

                        operator.evaluate(&actual_stake_flag, value, include_output)
                    }
                    _ => Box::new(EvaluationResult {
                        passed: false,
                        output: "Stake account is not in a state that has stake field".to_string(),
                    }),
                }
            }
        };

        Ok(result)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum MetaAssertion {
    RentExemptReserve {
        value: u64,
        operator: ComparableOperator,
    },
    AuthorizedStaker {
        value: Pubkey,
        operator: EquatableOperator,
    },
    AuthorizedWithdrawer {
        value: Pubkey,
        operator: EquatableOperator,
    },
    LockupUnixTimestamp {
        value: i64,
        operator: ComparableOperator,
    },
    LockupEpoch {
        value: u64,
        operator: ComparableOperator,
    },
    LockupCustodian {
        value: Pubkey,
        operator: EquatableOperator,
    },
}

impl Assert<Meta> for MetaAssertion {
    fn format(&self) -> String {
        format!("MetaAssertion[{:?}]", self)
    }

    fn evaluate(&self, meta: &Meta, include_output: bool) -> Result<Box<EvaluationResult>> {
        let result = match self {
            MetaAssertion::RentExemptReserve {
                value: assertion_value,
                operator,
            } => operator.evaluate(&meta.rent_exempt_reserve, assertion_value, include_output),
            MetaAssertion::AuthorizedStaker {
                value: assertion_value,
                operator,
            } => operator.evaluate(&meta.authorized.staker, assertion_value, include_output),
            MetaAssertion::AuthorizedWithdrawer {
                value: assertion_value,
                operator,
            } => operator.evaluate(&meta.authorized.withdrawer, assertion_value, include_output),
            MetaAssertion::LockupUnixTimestamp {
                value: assertion_value,
                operator,
            } => operator.evaluate(&meta.lockup.unix_timestamp, assertion_value, include_output),
            MetaAssertion::LockupEpoch {
                value: assertion_value,
                operator,
            } => operator.evaluate(&meta.lockup.epoch, assertion_value, include_output),
            MetaAssertion::LockupCustodian {
                value: assertion_value,
                operator,
            } => operator.evaluate(&meta.lockup.custodian, assertion_value, include_output),
        };

        Ok(result)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum StakeAssertion {
    DelegationVoterPubkey {
        value: Pubkey,
        operator: EquatableOperator,
    },
    DelegationStake {
        value: u64,
        operator: ComparableOperator,
    },
    DelegationActivationEpoch {
        value: u64,
        operator: ComparableOperator,
    },
    DelegationDeactivationEpoch {
        value: u64,
        operator: ComparableOperator,
    },
    CreditsObserved {
        value: u64,
        operator: ComparableOperator,
    },
}

impl Assert<Stake> for StakeAssertion {
    fn format(&self) -> String {
        format!("StakeAssertion[{:?}]", self)
    }

    fn evaluate(&self, stake: &Stake, include_output: bool) -> Result<Box<EvaluationResult>> {
        let result = match self {
            StakeAssertion::DelegationVoterPubkey {
                value: assertion_value,
                operator,
            } => operator.evaluate(
                &stake.delegation.voter_pubkey,
                assertion_value,
                include_output,
            ),
            StakeAssertion::DelegationStake {
                value: assertion_value,
                operator,
            } => operator.evaluate(&stake.delegation.stake, assertion_value, include_output),
            StakeAssertion::DelegationActivationEpoch {
                value: assertion_value,
                operator,
            } => operator.evaluate(
                &stake.delegation.activation_epoch,
                assertion_value,
                include_output,
            ),
            StakeAssertion::DelegationDeactivationEpoch {
                value: assertion_value,
                operator,
            } => operator.evaluate(
                &stake.delegation.deactivation_epoch,
                assertion_value,
                include_output,
            ),
            StakeAssertion::CreditsObserved {
                value: assertion_value,
                operator,
            } => operator.evaluate(&stake.credits_observed, assertion_value, include_output),
        };

        Ok(result)
    }
}
