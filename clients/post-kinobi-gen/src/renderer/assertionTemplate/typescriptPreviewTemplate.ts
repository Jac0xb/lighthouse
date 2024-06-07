import Handlebars from 'handlebars';
import { AssertionTemplateArgs, renderAssertionArgs } from './args';

const template = `
import { Address } from '@solana/web3.js';
import {
  EquatableOperator,
  IntegerOperator,
  dataValueAssertion,
  getAssertAccountDataMultiInstruction,
  LogLevelArgs,
} from '../generated';
import { AccountDataAssertion } from '../hooked';

export type {{assertionName}}AssertionArgs = {{#each variants}}
  | {
    __kind: '{{this.name}}';
    value: Address<string>;
    operator: EquatableOperator;
}{{/each}};

export type AssertAccountDataMultiInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  assertion: {{assertionName}}AssertionArgs;
};

export type AssertAccountDataMultiInput<
  TAccountTargetAccount extends string = string,
> = {
  /** Target account to be asserted */
  targetAccount: Address<TAccountTargetAccount>;
  logLevel?: AssertAccountDataMultiInstructionDataArgs['logLevel'];
  assertion: AssertAccountDataMultiInstructionDataArgs['assertion'];
};

export function getAssert{{assertionName}}Instruction(
  input: AssertAccountDataMultiInput<string>
) {
  let assertions: AccountDataAssertion[] = [];

{{#each variants}}
  {{#unless @last}}{{#if @first}}if{{/if}} (input.assertion.__kind === '{{this.name}}'){{/unless}} {
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
  } {{#unless @last}}else{{/unless}}
{{/each}}

  return getAssertAccountDataMultiInstruction({
    targetAccount: input.targetAccount,
    assertions,
  });
}
`;

export const CompiledTypescriptPreviewAssertionTemplate = (
  args: AssertionTemplateArgs
) =>
  Handlebars.compile(template)(renderAssertionArgs(args, 'typescript-preview'));
