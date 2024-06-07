
import { AssertAccountDataMultiInstructionAccounts, EquatableOperator, IntegerOperator, assertAccountDataMulti, dataValueAssertion } from "../generated";
import { Context, TransactionBuilder, PublicKey } from "@metaplex-foundation/umi";
import { AccountDataAssertion } from '../hooked';

export type VoteAccountAssertion = 
  | {
    __kind: 'AuthorizedWithdrawer';
    value: PublicKey;
    operator: EquatableOperator;
};


export function assertVoteAccount(
  context: Pick<Context, 'programs'>,
  input: AssertAccountDataMultiInstructionAccounts & VoteAccountAssertion
): TransactionBuilder {
let assertions: AccountDataAssertion[] = [];

  if (input.__kind === 'AuthorizedWithdrawer') {
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
          value: input.value,
          operator: input.operator,
        }),
      },
    ];
  }

  return assertAccountDataMulti(context, {
    targetAccount: input.targetAccount,
    assertions
  });
}
