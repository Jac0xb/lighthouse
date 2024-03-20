use super::{Assert, LogLevel};
use crate::{
    error::LighthouseError,
    types::assert::operator::{EquatableOperator, Evaluate, IntegerOperator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{bpf_loader_upgradeable::UpgradeableLoaderState, msg, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
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

impl Assert<&UpgradeableLoaderState> for UpgradeableLoaderStateAssertion {
    fn evaluate(&self, state: &UpgradeableLoaderState, log_level: LogLevel) -> Result<()> {
        match &self {
            UpgradeableLoaderStateAssertion::State {
                value: assertion_value,
                operator,
            } => {
                let actual_state = match state {
                    UpgradeableLoaderState::Uninitialized => {
                        UpgradeableLoaderStateType::Uninitialized
                    }
                    UpgradeableLoaderState::Buffer { .. } => UpgradeableLoaderStateType::Buffer,
                    UpgradeableLoaderState::Program { .. } => UpgradeableLoaderStateType::Program,
                    UpgradeableLoaderState::ProgramData { .. } => {
                        UpgradeableLoaderStateType::ProgramData
                    }
                } as u8;
                let casted_assertion_value = (*assertion_value) as u8;

                u8::evaluate(&actual_state, &casted_assertion_value, operator, log_level)
            }
            UpgradeableLoaderStateAssertion::Buffer(assertion) => {
                assertion.evaluate(state, log_level)
            }
            UpgradeableLoaderStateAssertion::Program(assertion) => {
                assertion.evaluate(state, log_level)
            }
            UpgradeableLoaderStateAssertion::ProgramData(assertion) => {
                assertion.evaluate(state, log_level)
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
    ) -> Result<()> {
        match &upgradable_loader_state {
            UpgradeableLoaderState::Buffer { authority_address } => match &self {
                UpgradableBufferAssertion::Authority {
                    value: assertion_value,
                    operator,
                } => <Option<Pubkey>>::evaluate(
                    authority_address,
                    assertion_value,
                    operator,
                    log_level,
                ),
            },
            _ => {
                msg!(
                    "Account is not in buffer state was {}",
                    get_state_enum(upgradable_loader_state)
                );
                Err(LighthouseError::AssertionFailed.into())
            }
        }
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
    ) -> Result<()> {
        match &upgradable_loader_state {
            UpgradeableLoaderState::Program {
                programdata_address,
            } => match &self {
                UpgradeableProgramAssertion::ProgramDataAddress {
                    value: assertion_value,
                    operator,
                } => Pubkey::evaluate(programdata_address, assertion_value, operator, log_level),
            },
            _ => {
                msg!(
                    "Account is not in program state was {}",
                    get_state_enum(upgradable_loader_state)
                );
                Err(LighthouseError::AssertionFailed.into())
            }
        }
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
        operator: IntegerOperator,
    },
}

impl Assert<&UpgradeableLoaderState> for UpgradeableProgramDataAssertion {
    fn evaluate(
        &self,
        upgradable_loader_state: &UpgradeableLoaderState,
        log_level: LogLevel,
    ) -> Result<()> {
        match &upgradable_loader_state {
            UpgradeableLoaderState::ProgramData {
                upgrade_authority_address,
                slot,
            } => match &self {
                UpgradeableProgramDataAssertion::UpgradeAuthority {
                    value: assertion_value,
                    operator,
                } => <Option<Pubkey>>::evaluate(
                    upgrade_authority_address,
                    assertion_value,
                    operator,
                    log_level,
                ),
                UpgradeableProgramDataAssertion::Slot {
                    value: assertion_value,
                    operator,
                } => u64::evaluate(slot, assertion_value, operator, log_level),
            },
            _ => {
                msg!(
                    "Account is not in program data state was {}",
                    get_state_enum(upgradable_loader_state)
                );
                Err(LighthouseError::AssertionFailed.into())
            }
        }
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
