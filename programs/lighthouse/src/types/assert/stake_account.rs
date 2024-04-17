use super::{Assert, LogLevel};
use crate::types::assert::evaluate::{EquatableOperator, Evaluate, IntegerOperator};
use crate::utils::Result;
use crate::{err, err_msg, error::LighthouseError};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::{
    pubkey::Pubkey,
    stake::state::{Meta as StakeMeta, Stake as StakeInfo, StakeStateV2},
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum StakeStateType {
    Uninitialized = 0,
    Initialized = 1,
    Stake = 2,
    RewardsPool = 3,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum StakeAccountAssertion {
    State {
        value: StakeStateType,
        operator: EquatableOperator,
    },
    MetaAssertion(MetaAssertion),
    StakeAssertion(StakeAssertion),
    StakeFlags {
        value: u8,
        operator: IntegerOperator,
    },
}

impl<'a> Assert<&'a StakeStateV2> for StakeAccountAssertion {
    fn evaluate(&self, stake_account: &'a StakeStateV2, log_level: LogLevel) -> Result<()> {
        match self {
            StakeAccountAssertion::State {
                value: assertion_value,
                operator,
            } => {
                let actual_state = match stake_account {
                    StakeStateV2::Uninitialized => StakeStateType::Uninitialized,
                    StakeStateV2::Initialized(_) => StakeStateType::Initialized,
                    StakeStateV2::Stake(_, _, _) => StakeStateType::Stake,
                    StakeStateV2::RewardsPool => StakeStateType::RewardsPool,
                } as u8;

                let casted_assertion_value = *assertion_value as u8;
                u8::evaluate(&actual_state, &casted_assertion_value, operator, log_level)
            }
            StakeAccountAssertion::MetaAssertion(meta_assertion) => match stake_account {
                StakeStateV2::Initialized(meta) | StakeStateV2::Stake(meta, _, _) => {
                    meta_assertion.evaluate(meta, log_level)
                }
                _ => {
                    msg!("Stake account is not in a state that has meta field");
                    Err(LighthouseError::AssertionFailed.into())
                }
            },
            StakeAccountAssertion::StakeAssertion(stake_assertion) => match stake_account {
                StakeStateV2::Stake(_, stake, _) => stake_assertion.evaluate(stake, log_level),
                _ => {
                    msg!("Stake account is not in a state that has stake field");
                    Err(LighthouseError::AssertionFailed.into())
                }
            },
            StakeAccountAssertion::StakeFlags { value, operator } => {
                match stake_account {
                    StakeStateV2::Stake(_, _, actual_stake_flags) => {
                        // No way to access stake flags directly, serialize and use the raw bytes
                        let serialized_stake_flag =
                            actual_stake_flags.try_to_vec().map_err(|e| {
                                err_msg!("Failed to serialize stake flags", e);
                                err!(LighthouseError::FailedToSerialize)
                            })?;

                        let actual_stake_flag = serialized_stake_flag[0];

                        u8::evaluate(&actual_stake_flag, value, operator, log_level)
                    }
                    _ => {
                        msg!("Stake account is not in a state that has stake field");
                        Err(LighthouseError::AssertionFailed.into())
                    }
                }
            }
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum MetaAssertion {
    RentExemptReserve {
        value: u64,
        operator: IntegerOperator,
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
        operator: IntegerOperator,
    },
    LockupEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    LockupCustodian {
        value: Pubkey,
        operator: EquatableOperator,
    },
}

impl Assert<&StakeMeta> for MetaAssertion {
    fn evaluate(&self, meta: &StakeMeta, log_level: LogLevel) -> Result<()> {
        match self {
            MetaAssertion::RentExemptReserve {
                value: assertion_value,
                operator,
            } => u64::evaluate(
                &meta.rent_exempt_reserve,
                assertion_value,
                operator,
                log_level,
            ),
            MetaAssertion::AuthorizedStaker {
                value: assertion_value,
                operator,
            } => Pubkey::evaluate(
                &meta.authorized.staker,
                assertion_value,
                operator,
                log_level,
            ),
            MetaAssertion::AuthorizedWithdrawer {
                value: assertion_value,
                operator,
            } => Pubkey::evaluate(
                &meta.authorized.withdrawer,
                assertion_value,
                operator,
                log_level,
            ),
            MetaAssertion::LockupUnixTimestamp {
                value: assertion_value,
                operator,
            } => i64::evaluate(
                &meta.lockup.unix_timestamp,
                assertion_value,
                operator,
                log_level,
            ),
            MetaAssertion::LockupEpoch {
                value: assertion_value,
                operator,
            } => u64::evaluate(&meta.lockup.epoch, assertion_value, operator, log_level),
            MetaAssertion::LockupCustodian {
                value: assertion_value,
                operator,
            } => Pubkey::evaluate(&meta.lockup.custodian, assertion_value, operator, log_level),
        }
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
        operator: IntegerOperator,
    },
    DelegationActivationEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    DelegationDeactivationEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    CreditsObserved {
        value: u64,
        operator: IntegerOperator,
    },
}

impl Assert<&StakeInfo> for StakeAssertion {
    fn evaluate(&self, stake: &StakeInfo, log_level: LogLevel) -> Result<()> {
        match self {
            StakeAssertion::DelegationVoterPubkey {
                value: assertion_value,
                operator,
            } => Pubkey::evaluate(
                &stake.delegation.voter_pubkey,
                assertion_value,
                operator,
                log_level,
            ),
            StakeAssertion::DelegationStake {
                value: assertion_value,
                operator,
            } => u64::evaluate(
                &stake.delegation.stake,
                assertion_value,
                operator,
                log_level,
            ),
            StakeAssertion::DelegationActivationEpoch {
                value: assertion_value,
                operator,
            } => u64::evaluate(
                &stake.delegation.activation_epoch,
                assertion_value,
                operator,
                log_level,
            ),
            StakeAssertion::DelegationDeactivationEpoch {
                value: assertion_value,
                operator,
            } => u64::evaluate(
                &stake.delegation.deactivation_epoch,
                assertion_value,
                operator,
                log_level,
            ),
            StakeAssertion::CreditsObserved {
                value: assertion_value,
                operator,
            } => u64::evaluate(
                &stake.credits_observed,
                assertion_value,
                operator,
                log_level,
            ),
        }
    }
}
