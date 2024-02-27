use crate::{
    error::LighthouseError,
    types::{Assert, EvaluationResult, Operator},
};
use crate::{
    types::{ComparableOperator, EquatableOperator, IntegerOperator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
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
    State(u8, ComparableOperator),
    MetaAssertion(MetaAssertion),
    StakeAssertion(StakeAssertion),
    StakeFlags(u8, IntegerOperator),
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
            return Err(LighthouseError::OwnerMismatch.into());
        }

        // TODO: Logic to assert on if account is a mint account
        let data = account.try_borrow_data()?;
        let stake_account = StakeStateV2::deserialize(&mut data.as_ref())?;

        let result = match self {
            StakeAccountAssertion::State(state, operator) => {
                let state_enum: u8 = match stake_account {
                    StakeStateV2::Uninitialized => 0,
                    StakeStateV2::Initialized(_) => 1,
                    StakeStateV2::Stake(_, _, _) => 2,
                    StakeStateV2::RewardsPool => 3,
                };

                operator.evaluate(&state_enum, state, include_output)
            }
            StakeAccountAssertion::MetaAssertion(meta_assertion) => match stake_account {
                StakeStateV2::Initialized(meta) | StakeStateV2::Stake(meta, _, _) => {
                    meta_assertion.evaluate(&meta, include_output)?
                }
                _ => Box::new(EvaluationResult {
                    passed: false,
                    output: "Stake account is not in a state that has meta field".to_string(),
                }),
            },
            StakeAccountAssertion::StakeAssertion(stake_assertion) => match stake_account {
                StakeStateV2::Stake(_, stake, _) => {
                    stake_assertion.evaluate(&stake, include_output)?
                }
                _ => Box::new(EvaluationResult {
                    passed: false,
                    output: "Stake account is not in a state that has stake field".to_string(),
                }),
            },
            StakeAccountAssertion::StakeFlags(stake_flags, operator) => match stake_account {
                StakeStateV2::Stake(_, _, actual_stake_flags) => {
                    // No way to access stake flags directly
                    let serialized_stake_flag = actual_stake_flags.try_to_vec()?;
                    let actual_stake_flag = serialized_stake_flag.first().unwrap();

                    operator.evaluate(actual_stake_flag, stake_flags, include_output)
                }
                _ => Box::new(EvaluationResult {
                    passed: false,
                    output: "Stake account is not in a state that has stake field".to_string(),
                }),
            },
        };

        Ok(result)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum MetaAssertion {
    RentExemptReserve(u64, ComparableOperator),
    AuthorizedStaker(Pubkey, EquatableOperator),
    AuthorizedWithdrawer(Pubkey, EquatableOperator),
    LockupUnixTimestamp(i64, ComparableOperator),
    LockupEpoch(u64, ComparableOperator),
    LockupCustodian(Pubkey, EquatableOperator),
}

impl Assert<Meta> for MetaAssertion {
    fn format(&self) -> String {
        format!("MetaAssertion[{:?}]", self)
    }

    fn evaluate(&self, meta: &Meta, include_output: bool) -> Result<Box<EvaluationResult>> {
        let result = match self {
            MetaAssertion::RentExemptReserve(rent_exempt_reserve, operator) => operator.evaluate(
                &meta.rent_exempt_reserve,
                rent_exempt_reserve,
                include_output,
            ),
            MetaAssertion::AuthorizedStaker(authorized_staker, operator) => {
                operator.evaluate(&meta.authorized.staker, authorized_staker, include_output)
            }
            MetaAssertion::AuthorizedWithdrawer(authorized_withdrawer, operator) => operator
                .evaluate(
                    &meta.authorized.withdrawer,
                    authorized_withdrawer,
                    include_output,
                ),
            MetaAssertion::LockupUnixTimestamp(lockup_unix_timestamp, operator) => operator
                .evaluate(
                    &meta.lockup.unix_timestamp,
                    lockup_unix_timestamp,
                    include_output,
                ),
            MetaAssertion::LockupEpoch(lockup_epoch, operator) => {
                operator.evaluate(&meta.lockup.epoch, lockup_epoch, include_output)
            }
            MetaAssertion::LockupCustodian(lockup_custodian, operator) => {
                operator.evaluate(&meta.lockup.custodian, lockup_custodian, include_output)
            }
        };

        Ok(result)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum StakeAssertion {
    DelegationVoterPubkey(Pubkey, EquatableOperator),
    DelegationStake(u64, ComparableOperator),
    DelegationActivationEpoch(u64, ComparableOperator),
    DelegationDeactivationEpoch(u64, ComparableOperator),
    // DelegationWarmupCooldownRate(f64, ComparableOperator),
    /// stake account's credits observed at the time of delegation
    CreditsObserved(u64, ComparableOperator),
}

impl Assert<Stake> for StakeAssertion {
    fn format(&self) -> String {
        format!("StakeAssertion[{:?}]", self)
    }

    fn evaluate(&self, stake: &Stake, include_output: bool) -> Result<Box<EvaluationResult>> {
        let result = match self {
            StakeAssertion::DelegationVoterPubkey(voter_pubkey, operator) => {
                operator.evaluate(&stake.delegation.voter_pubkey, voter_pubkey, include_output)
            }
            StakeAssertion::DelegationStake(delegation_stake, operator) => {
                operator.evaluate(&stake.delegation.stake, delegation_stake, include_output)
            }
            StakeAssertion::DelegationActivationEpoch(activation_epoch, operator) => operator
                .evaluate(
                    &stake.delegation.activation_epoch,
                    activation_epoch,
                    include_output,
                ),
            StakeAssertion::DelegationDeactivationEpoch(deactivation_epoch, operator) => operator
                .evaluate(
                    &stake.delegation.deactivation_epoch,
                    deactivation_epoch,
                    include_output,
                ),
            // StakeAssertion::DelegationWarmupCooldownRate(_warmup_cooldown_rate, _operator) => {
            //     panic!("Not implemented");
            // }
            StakeAssertion::CreditsObserved(credits_observed, operator) => {
                operator.evaluate(&stake.credits_observed, credits_observed, include_output)
            }
        };

        Ok(result)
    }
}
