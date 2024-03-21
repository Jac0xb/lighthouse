import { Address, address } from '@solana/addresses';
import { AccountRole, WritableSignerAccount } from '@solana/instructions';
import {
  TransactionSigner,
  addSignersToTransaction,
  createKeyPairSignerFromBytes,
  createSignerFromKeyPair,
  generateKeyPairSigner,
  isTransactionPartialSigner,
  signAndSendTransactionWithSigners,
  signTransactionWithSigners,
} from '@solana/signers';
import { CompilableTransaction } from '@solana/transactions';
import {
  MemoryCloseInstruction,
  WriteType,
  dataValue,
  findMemoryPda,
  getAssertAccountDataInstruction,
  getMemoryCloseInstruction,
  getMemoryWriteInstruction,
  isTransactionSigner,
  writeType,
  DataValueAssertion,
  dataValueAssertion,
  IntegerOperator,
  LogLevel,
} from 'lighthouse-sdk';
import {
  appendTransactionInstructions,
  createSolanaRpc,
  createTransaction,
  getBase64EncodedWireTransaction,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionFeePayer,
  setTransactionLifetimeUsingBlockhash,
  signTransaction,
} from '@solana/web3.js';
import fs from 'fs';

async function buildWriteTransaction(
  payer: TransactionSigner,
  sourceAccount: Address<string>,
  writeType: WriteType,
  writeOffset: number
) {
  const [memory, memoryBump] = await findMemoryPda({
    memoryId: 0,
    payer: payer.address,
  });

  let ix = getMemoryWriteInstruction({
    memory,
    payer,
    memoryBump,
    memoryId: 0,
    writeOffset,
    sourceAccount,
    writeType,
  });
}

async function buildAssertDataInstruction(targetAccount: Address) {
  const ix = getAssertAccountDataInstruction({
    targetAccount,
    offset: 0,
    assertion: dataValueAssertion('U64', {
      value: 128,
      operator: IntegerOperator.Equal,
    }),
    logLevel: LogLevel.EncodedMessage,
  });

  // ix.accounts.push({
  //   role: AccountRole.READONLY,
  //   address: address('noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV'),
  // });

  return ix;
}

(async () => {
  const keypairPath = '';

  const array = JSON.parse(fs.readFileSync(keypairPath).toString());
  const signer = await createKeyPairSignerFromBytes(Uint8Array.from(array));

  let [memoryPda] = await findMemoryPda({
    memoryId: 0,
    payer: signer.address,
  });
  let ix = await buildAssertDataInstruction(memoryPda);
  const rpc = createSolanaRpc('https://api.devnet.solana.com');

  const { value: recentBlockhash } = await rpc.getLatestBlockhash().send();

  // const tx: CompilableTransaction = {
  //   instructions: [ix],
  //   feePayer: signer.address,
  //   version: 'legacy',
  //   lifetimeConstraint: latestBlockhash,
  // };

  // const tx = createTransaction({
  //   version: 'legacy',
  // });
  const transaction = await pipe(
    createTransaction({ version: 0 }),
    (tx) => setTransactionFeePayer(signer.address, tx),
    (tx) => appendTransactionInstructions([ix], tx),
    (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
    (tx) => signTransaction([signer.keyPair], tx)
    // (tx) => addSignersToTransaction([signer], tx)
  );

  console.log(transaction.instructions[0].accounts);

  // const transactionWithFeePayerV0 = setTransactionFeePayer(signer.address, tx);
  // const transactionWithFeePayerAndLifetimeV0 =
  //   setTransactionLifetimeUsingBlockhash(
  //     recentBlockhash,
  //     transactionWithFeePayerV0
  //   );

  // const signedTx = await signer.signTransactions([
  //   transactionWithFeePayerAndLifetimeV0,
  // ]);

  // const transactionSignaturesStripped = setTransactionLifetimeUsingBlockhash(
  //   recentBlockhash,
  //   transactionSignedWithFeePayerAndLifetime
  // );

  let result = await rpc
    .sendTransaction(getBase64EncodedWireTransaction(transaction), {
      encoding: 'base64',
    })
    .send();

  console.log(result);
})();
