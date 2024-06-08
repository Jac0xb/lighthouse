import Handlebars from 'handlebars';
import { AssertionTemplateArgs, renderAssertionArgs } from './args';

const template = `
import { AssertAccountDataMultiInstructionAccounts, EquatableOperator, IntegerOperator, assertAccountDataMulti, dataValueAssertion } from "../generated";
import { Context, TransactionBuilder, PublicKey } from "@metaplex-foundation/umi";
import { AccountDataAssertion } from '../hooked';

export type {{assertionName}}Assertion = 
{{#each variants}}
  | {
    __kind: '{{this.name}}';
    value: {{this.valueType}};
    operator: {{this.operatorType}};
}{{/each}};


export function assert{{assertionName}}(
  context: Pick<Context, 'programs'>,
  input: AssertAccountDataMultiInstructionAccounts & {{assertionName}}Assertion
): TransactionBuilder {
let assertions: AccountDataAssertion[] = [];

{{#each variants}}
  if (input.__kind === '{{this.name}}') {
      assertions = [
{{#each this.assertions}}
      {
        offset: {{this.offset}},
        assertion: dataValueAssertion('{{this.kind}}', {
          value: {{this.value}},
          operator: {{this.valueOperator}},
        }),
      },
{{/each}}
    ];
  }
{{/each}}

  return assertAccountDataMulti(context, {
    targetAccount: input.targetAccount,
    assertions
  });
}
`;

export const CompiledTypescriptAssertionTemplate = (
  args: AssertionTemplateArgs
) => Handlebars.compile(template)(renderAssertionArgs(args, 'typescript'));
