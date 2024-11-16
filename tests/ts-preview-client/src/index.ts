import { Address, address } from '@solana/addresses';
import {
  AccountRole,
  IInstruction,
  WritableSignerAccount,
} from '@solana/instructions';

import fs from 'fs';

import {
  SYSTEM_PROGRAM_ADDRESS,
  getTransferSolInstruction,
} from '@solana-program/system';
import {
  KeyPairSigner,
  addSignersToTransactionMessage,
  createKeyPairSignerFromBytes,
  generateKeyPairSigner,
  setTransactionMessageFeePayerSigner,
  signTransactionMessageWithSigners,
} from '@solana/signers';
import {
  AccountInfoBase,
  AccountInfoWithBase64EncodedData,
  ITransactionMessageWithFeePayer,
  Rpc,
  SolanaRpcApi,
  Transaction,
  TransactionMessage,
  appendTransactionMessageInstruction,
  appendTransactionMessageInstructions,
  compileTransactionMessage,
  createSolanaRpc,
  createTransactionMessage,
  decompileTransactionMessage,
  getBase64EncodedWireTransaction,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransaction,
} from '@solana/web3.js';
import { generateKeyPairSync } from 'crypto';
import {
  createInitializeMint2Instruction,
  createMint,
} from '@solana/spl-token';

import crypto from 'crypto';
import {
  EquatableOperator,
  getAssertAccountDataInstruction,
  getAssertAccountInfoInstruction,
  getAssertAccountInfoMultiInstruction,
  IntegerOperator,
} from 'lighthouse-sdk';

(async () => {
  const keypairPath =
    '/Users/jacob/Desktop/W1ckNjLqZ2UoijogQj9rMCacdFtn6KJm3Caxs1oPgdx.json';

  console.log(globalThis.crypto);

  const array = JSON.parse(fs.readFileSync(keypairPath).toString());
  const signer = await createKeyPairSignerFromBytes(Uint8Array.from(array));
  const rpc = createSolanaRpc('https://api.devnet.solana.com');

  // const result = await rpc
  //   .sendTransaction(getBase64EncodedWireTransaction(tx), {
  //     encoding: 'base64',
  //   })
  //   .send();

  // console.log(result);

  // const config: AccountsConfigWithBase64EncodingZstdCompression &
  //   SigVerifyAndReplaceRecentBlockhashConfig &
  //   SimulateTransactionConfigBase = {
  //   replaceRecentBlockhash: true,
  //   sigVerify: true,
  //   accounts: {
  //     addresses: [],
  //     encoding: 'base64+zstd',
  //   },
  //   encoding: 'base64',
  // };

  const destination = await generateKeyPairSigner();

  const tx = await buildTransaction(
    rpc,
    [
      getTransferSolInstruction({
        source: signer,
        destination: signer.address,
        amount: 10,
      }),
      getTransferSolInstruction({
        source: signer,
        destination: destination.address,
        amount: 0.001e9,
      }),
      getAssertAccountInfoMultiInstruction({
        targetAccount: destination.address,
        assertions: [
          {
            __kind: 'Lamports',
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: 'Lamports',
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: 'Lamports',
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: 'Lamports',
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: 'Lamports',
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
        ],
      }),
    ],
    [signer]
  );

  const { writeableAccounts, signerAccounts } = await getWriteablesAndSigners(
    tx
  );

  console.log(writeableAccounts);
  console.log(signerAccounts);

  const { value: accounts } = await rpc
    .getMultipleAccounts(writeableAccounts)
    .send();

  const result = await rpc
    .simulateTransaction(getBase64EncodedWireTransaction(tx), {
      replaceRecentBlockhash: true,
      sigVerify: false,
      accounts: {
        addresses: [signer.address],
        encoding: 'base64',
      },
      encoding: 'base64',
    })
    .send();

  console.log(JSON.stringify(result, undefined, 2));
})();

async function buildTransaction(
  rpc: Rpc<SolanaRpcApi>,
  instructions: IInstruction[],
  signers: KeyPairSigner[]
) {
  const { value: recentBlockhash } = await rpc.getLatestBlockhash().send();

  const message = pipe(
    createTransactionMessage({ version: 0 }),
    (msg) => appendTransactionMessageInstructions(instructions, msg),
    (msg) => setTransactionMessageFeePayer(signers[0].address, msg),
    (msg) => addSignersToTransactionMessage(signers, msg),
    (m) => setTransactionMessageLifetimeUsingBlockhash(recentBlockhash, m)
  );

  return await signTransactionMessageWithSigners(message);
}

(BigInt.prototype as any).toJSON = function () {
  return this.toString();
};

async function strictVerify() {}

async function getWriteablesAndSigners(tx: Transaction) {
  const signerAccounts: Set<Address> = new Set();
  const writeableAccounts: Set<Address> = new Set();

  // const message: TransactionMessage & ITransactionMessageWithFeePayer =
  //   createTransactionMessage({});

  // for (const instruction of tx.messageBytes.instructions) {
  //   for (const account of instruction.accounts ?? []) {
  //     if (account.role === AccountRole.WRITABLE) {
  //       writeableAccounts.add(account.address);
  //     }

  //     if (account.role === AccountRole.WRITABLE_SIGNER) {
  //       writeableAccounts.add(account.address);
  //       signerAccounts.add(account.address);
  //     }
  //   }
  // }

  // signerAccounts.add(tx.feePayer);

  return {
    writeableAccounts: [...writeableAccounts],
    signerAccounts: [...signerAccounts],
  };
}

type AccountInfo = AccountInfoBase & AccountInfoWithBase64EncodedData;

enum AccountType {
  SYSTEM_PROGRAM,
  SPL_TOKEN_PROGRAM,
  SPL_TOKEN_2022_PROGRAM,
  SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM,
  UNKNOWN,
  UNOWNED,
}

type ResolvedAccount = {
  accountType: AccountType;
};

type ResolvedSystemProgramAccount = ResolvedAccount & {
  accountType: AccountType.SYSTEM_PROGRAM;
};

async function resolveAccount(
  account: AccountInfo | null
): Promise<ResolvedAccount> {
  if (!account) {
    return { accountType: AccountType.UNOWNED };
  }

  switch (account.owner) {
    case SYSTEM_PROGRAM_ADDRESS:
      return await resolveSystemProgramAccount(account);
    default:
      return { accountType: AccountType.UNKNOWN };
  }
}

async function resolveSystemProgramAccount(
  account: AccountInfo
): Promise<ResolvedSystemProgramAccount> {
  return { accountType: AccountType.SYSTEM_PROGRAM };
}
