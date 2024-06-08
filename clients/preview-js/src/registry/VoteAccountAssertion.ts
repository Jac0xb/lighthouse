import { Address } from '@solana/web3.js';
import {
  EquatableOperator,
  IntegerOperator,
  dataValueAssertion,
  getAssertAccountDataMultiInstruction,
  LogLevelArgs,
} from '../generated';
import { AccountDataAssertion } from '../hooked';

export type VoteAccountAssertionArgs = {
  __kind: 'AuthorizedWithdrawer';
  value: Address<string>;
  operator: EquatableOperator;
};

export type AssertAccountDataMultiInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  assertion: VoteAccountAssertionArgs;
};

export type AssertAccountDataMultiInput<
  TAccountTargetAccount extends string = string,
> = {
  /** Target account to be asserted */
  targetAccount: Address<TAccountTargetAccount>;
  logLevel?: AssertAccountDataMultiInstructionDataArgs['logLevel'];
  assertion: AssertAccountDataMultiInstructionDataArgs['assertion'];
};

export function getAssertVoteAccountInstruction(
  input: AssertAccountDataMultiInput<string>
) {
  let assertions: AccountDataAssertion[] = [];

  {
    assertions = [
      {
        offset: 0,
        assertion: dataValueAssertion('U8', {
          value: 2,
          operator: IntegerOperator.Equal,
        }),
      },
      {
        offset: 36,
        assertion: dataValueAssertion('Pubkey', {
          value: input.assertion.value,
          operator: input.assertion.operator,
        }),
      },
    ];
  }

  return getAssertAccountDataMultiInstruction({
    targetAccount: input.targetAccount,
    assertions,
  });
}
