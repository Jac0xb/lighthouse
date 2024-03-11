use super::{Assert, LogLevel};
use crate::{
    err, err_msg,
    error::LighthouseError,
    types::assert::operator::{ComparableOperator, EquatableOperator, EvaluationResult, Operator},
    utils::{keys_equal, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, bpf_loader_upgradeable,
    bpf_loader_upgradeable::UpgradeableLoaderState, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
#[repr(u8)]

pub enum UpgradeableLoaderStateType {
    Uninitialized = 0,
    Buffer = 1,
    Program = 2,
    ProgramData = 3,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]

pub enum UpgradeableLoaderStateAssertion {
    State {
        value: UpgradeableLoaderStateType,
        operator: EquatableOperator,
    },
    Buffer(UpgradableBufferAssertion),
    Program(UpgradeableProgramAssertion),
    ProgramData(UpgradeableProgramDataAssertion),
}

impl Assert<&AccountInfo<'_>> for UpgradeableLoaderStateAssertion {
    fn evaluate(
        &self,
        account: &AccountInfo<'_>,
        log_level: LogLevel,
    ) -> Result<Box<EvaluationResult>> {
        if !keys_equal(account.owner, &bpf_loader_upgradeable::ID) {
            return Err(LighthouseError::AccountOwnerMismatch.into());
        }

        let account_data = account.try_borrow_data().map_err(|e| {
            err_msg!("Failed to borrow data for target account", e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        let get_state = || {
            let state: UpgradeableLoaderState =
                bincode::deserialize(&account_data).map_err(|e| {
                    err_msg!("Failed to deserialize upgradeable loader state", e);
                    err!(LighthouseError::AccountBorrowFailed)
                })?;

            Ok::<UpgradeableLoaderState, ProgramError>(state)
        };

        match &self {
            UpgradeableLoaderStateAssertion::State {
                value: assertion_value,
                operator,
            } => {
                let casted_assertion_value: u8 = assertion_value.clone() as u8;
                let actual_state: u8 = account_data[0];

                if actual_state > 4 {
                    msg!("Failed to deserialize upgradeable loader state: enum out of bounds");
                    return Err(LighthouseError::FailedToDeserialize.into());
                }

                Ok(operator.evaluate(&actual_state, &casted_assertion_value, log_level))
            }
            UpgradeableLoaderStateAssertion::Buffer(assertion) => {
                let state = get_state()?;
                assertion.evaluate(&state, log_level)
            }
            UpgradeableLoaderStateAssertion::Program(assertion) => {
                assertion.evaluate(&get_state()?, log_level)
            }
            UpgradeableLoaderStateAssertion::ProgramData(assertion) => {
                assertion.evaluate(&get_state()?, log_level)
            }
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum UpgradableBufferAssertion {
    Authority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
}

impl Assert<&UpgradeableLoaderState> for UpgradableBufferAssertion {
    fn evaluate(
        &self,
        upgradable_loader_state: &UpgradeableLoaderState,
        log_level: LogLevel,
    ) -> Result<Box<EvaluationResult>> {
        let result = match &upgradable_loader_state {
            UpgradeableLoaderState::Buffer { authority_address } => match &self {
                UpgradableBufferAssertion::Authority {
                    value: assertion_value,
                    operator,
                } => operator.evaluate(authority_address, assertion_value, log_level),
            },
            _ => Box::new(EvaluationResult {
                passed: false,
                output: Some(format!(
                    "Account is not in program state was {}",
                    get_state_enum(upgradable_loader_state)
                )),
            }),
        };

        Ok(result)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum UpgradeableProgramAssertion {
    ProgramDataAddress {
        value: Pubkey,
        operator: EquatableOperator,
    },
}

impl Assert<&UpgradeableLoaderState> for UpgradeableProgramAssertion {
    fn evaluate(
        &self,
        upgradable_loader_state: &UpgradeableLoaderState,
        log_level: LogLevel,
    ) -> Result<Box<EvaluationResult>> {
        let result = match &upgradable_loader_state {
            UpgradeableLoaderState::Program {
                programdata_address,
            } => match &self {
                UpgradeableProgramAssertion::ProgramDataAddress {
                    value: assertion_value,
                    operator,
                } => operator.evaluate(programdata_address, assertion_value, log_level),
            },
            _ => Box::new(EvaluationResult {
                passed: false,
                output: Some(format!(
                    "Account is not in program state was {}",
                    get_state_enum(upgradable_loader_state)
                )),
            }),
        };

        Ok(result)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum UpgradeableProgramDataAssertion {
    UpgradeAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    Slot {
        value: u64,
        operator: ComparableOperator,
    },
}

impl Assert<&UpgradeableLoaderState> for UpgradeableProgramDataAssertion {
    fn evaluate(
        &self,
        upgradable_loader_state: &UpgradeableLoaderState,
        log_level: LogLevel,
    ) -> Result<Box<EvaluationResult>> {
        Ok(match &upgradable_loader_state {
            UpgradeableLoaderState::ProgramData {
                upgrade_authority_address,
                slot,
            } => match &self {
                UpgradeableProgramDataAssertion::UpgradeAuthority {
                    value: assertion_value,
                    operator,
                } => operator.evaluate(upgrade_authority_address, assertion_value, log_level),
                UpgradeableProgramDataAssertion::Slot {
                    value: assertion_value,
                    operator,
                } => operator.evaluate(slot, assertion_value, log_level),
            },
            _ => Box::new(EvaluationResult {
                passed: false,
                output: Some(format!(
                    "Account is not in program data state was {}",
                    get_state_enum(upgradable_loader_state)
                )),
            }),
        })
    }
}

pub fn get_state_enum(state: &UpgradeableLoaderState) -> u8 {
    match state {
        UpgradeableLoaderState::Uninitialized => 0,
        UpgradeableLoaderState::Buffer { .. } => 1,
        UpgradeableLoaderState::Program { .. } => 2,
        UpgradeableLoaderState::ProgramData { .. } => 3,
    }
}
