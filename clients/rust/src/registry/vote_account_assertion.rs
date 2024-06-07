
#[derive(Debug, Clone)]
pub enum VoteAccountAssertion {
    AuthorizedWithdrawer {
        value: solana_program::pubkey::Pubkey,
        operator: crate::generated::types::EquatableOperator,
    },
}

pub struct AssertVoteAccountBuilder(crate::generated::instructions::AssertAccountDataMultiBuilder);

impl AssertVoteAccountBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(crate::generated::instructions::AssertAccountDataMultiBuilder::new())
    }

    #[allow(clippy::redundant_closure_call)]
    #[allow(clippy::redundant_field_names)]
    pub fn assertion(&mut self, assertion: VoteAccountAssertion) -> &mut Self {
        match assertion {
            VoteAccountAssertion::AuthorizedWithdrawer { value, operator } => {
                self.0.assertions(vec![
                    crate::hooked::AccountDataAssertion {
                        offset: crate::CompactU64(0),
                        assertion: crate::generated::types::DataValueAssertion::U8 {
                            value: 2,
                            operator: crate::types::IntegerOperator::Equal,
                        },
                    },
                    crate::hooked::AccountDataAssertion {
                        offset: crate::CompactU64(36),
                        assertion: crate::generated::types::DataValueAssertion::Pubkey {
                            value: value,
                            operator: operator,
                        },
                    },
                ].into());
            },
        }
        self
    }

    pub fn log_level(&mut self, log_level: crate::generated::types::LogLevel) -> &mut Self {
        self.0.log_level(log_level);
        self
    }

    pub fn target_account(&mut self, target_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.0.target_account(target_account);
        self
    }

    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.0.instruction()
    }
}
