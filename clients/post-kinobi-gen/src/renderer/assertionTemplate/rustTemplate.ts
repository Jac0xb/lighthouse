import Handlebars from 'handlebars';
import { AssertionTemplateArgs, renderAssertionArgs } from './args';

const template = `
#[derive(Debug, Clone)]
pub enum {{assertionName}}Assertion {
{{#each variants}}
    {{this.name}} {
        value: {{this.valueType}},
        operator: crate::generated::types::{{this.operatorType}},
    },
{{/each}}
}

pub struct Assert{{assertionName}}Builder(crate::generated::instructions::AssertAccountDataMultiBuilder);

impl Assert{{assertionName}}Builder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(crate::generated::instructions::AssertAccountDataMultiBuilder::new())
    }

    #[allow(clippy::redundant_closure_call)]
    #[allow(clippy::redundant_field_names)]
    pub fn assertion(&mut self, assertion: {{assertionName}}Assertion) -> &mut Self {
        match assertion {
{{#each variants}}
            {{@root.assertionName}}Assertion::{{this.name}} { value, operator } => {
                self.0.assertions(vec![
{{#each this.assertions}}
                    crate::hooked::AccountDataAssertion {
                        offset: crate::CompactU64({{offset}}),
                        assertion: crate::generated::types::DataValueAssertion::{{this.kind}} {
                            value: {{this.value}},
                            operator: {{this.valueOperator}},
                        },
                    },
{{/each}}
                ].into());
            },
{{/each}}
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
`;

export const CompiledRustAssertionTemplate = (args: AssertionTemplateArgs) =>
  Handlebars.compile(template)(renderAssertionArgs(args, 'rust'));
