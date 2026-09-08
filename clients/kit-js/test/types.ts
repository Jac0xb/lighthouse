import {
  address,
  appendTransactionMessageInstruction,
  createTransactionMessage,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageLoadedAccountsDataSizeLimit,
  setTransactionMessagePriorityFeeLamports,
} from '@solana/kit';
import { getAssertAccountInfoInstruction, IntegerOperator } from '../src';

const instruction = getAssertAccountInfoInstruction({
  targetAccount: address('11111111111111111111111111111111'),
  assertion: {
    __kind: 'Lamports',
    value: 42n,
    operator: IntegerOperator.Equal,
  },
});
const message = appendTransactionMessageInstruction(
  instruction,
  createTransactionMessage({ version: 1 })
);
const configured = setTransactionMessagePriorityFeeLamports(
  5_000n,
  setTransactionMessageLoadedAccountsDataSizeLimit(
    65_536,
    setTransactionMessageComputeUnitLimit(200_000, message)
  )
);
const version: 1 = configured.version;
void version;
