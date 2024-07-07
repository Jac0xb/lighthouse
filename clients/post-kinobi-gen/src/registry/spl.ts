import {
  AssertionTemplateArgs,
  IntegerOperator,
} from '../renderer/assertionTemplate/args';
import {
  DefaultValueFn,
  DefaultValueOperatorFn,
} from '../renderer/assertionTemplate/valueRenderer';

export const voteAccountTemplateArgs: AssertionTemplateArgs = {
  assertionName: 'VoteAccount',
  variants: [
    {
      name: 'AuthorizedWithdrawer',
      kind: 'Pubkey',
      assertions: [
        {
          kind: 'U8',
          value: 2,
          valueOperator: { type: 'Integer', enum: IntegerOperator.Equal },
          offset: 0,
        },
        {
          kind: 'Pubkey',
          value: DefaultValueFn,
          valueOperator: DefaultValueOperatorFn,
          offset: 36,
        },
      ],
    },
  ],
};
