import { appendTransactionInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import {
  Counter,
  fetchCounterFromSeeds,
  getCreateInstructionAsync,
} from '../src';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup';

test('it creates a new counter account', async (t) => {
  // Given
  const client = createDefaultSolanaClient();
  const authority = await generateKeyPairSignerWithSol(client);

  // When
  const createIx = await getCreateInstructionAsync({ authority });
  await pipe(
    await createDefaultTransaction(client, authority),
    (tx) => appendTransactionInstruction(createIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  // Then
  const counter = await fetchCounterFromSeeds(client.rpc, {
    authority: authority.address,
  });
  t.like(counter, <Counter>{
    data: {
      authority: authority.address,
      value: 0,
    },
  });
});
