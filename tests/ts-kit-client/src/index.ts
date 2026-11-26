import fs from "fs";
import {
  SYSTEM_PROGRAM_ADDRESS,
  getTransferSolInstruction,
} from "@solana-program/system";
import {
  type AccountInfoBase,
  type AccountInfoWithBase64EncodedData,
  type Address,
  type Instruction,
  type KeyPairSigner,
  type Rpc,
  type SolanaRpcApi,
  type Transaction,
  addSignersToTransactionMessage,
  appendTransactionMessageInstructions,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createTransactionMessage,
  generateKeyPairSigner,
  getBase64EncodedWireTransaction,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import {
  getAssertAccountInfoMultiInstruction,
  IntegerOperator,
} from "lighthouse-sdk";

(async () => {
  const keypairPath = process.env.KEYPAIR_PATH ?? "";

  const array = JSON.parse(fs.readFileSync(keypairPath).toString());
  const signer = await createKeyPairSignerFromBytes(Uint8Array.from(array));
  const rpc = createSolanaRpc("https://api.devnet.solana.com");

  const destination = await generateKeyPairSigner();

  const tx = await buildTransaction(
    rpc,
    [
      getTransferSolInstruction({
        source: signer,
        destination: signer.address,
        amount: 10n,
      }),
      getTransferSolInstruction({
        source: signer,
        destination: destination.address,
        amount: BigInt(0.001e9),
      }),
      getAssertAccountInfoMultiInstruction({
        targetAccount: destination.address,
        assertions: [
          {
            __kind: "Lamports",
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: "Lamports",
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: "Lamports",
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: "Lamports",
            value: BigInt(0.001e9),
            operator: IntegerOperator.Equal,
          },
          {
            __kind: "Lamports",
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

  console.log(accounts);

  const result = await rpc
    .simulateTransaction(getBase64EncodedWireTransaction(tx), {
      replaceRecentBlockhash: true,
      sigVerify: false,
      accounts: {
        addresses: [signer.address],
        encoding: "base64",
      },
      encoding: "base64",
    })
    .send();

  console.log(JSON.stringify(result, undefined, 2));
})();

async function buildTransaction(
  rpc: Rpc<SolanaRpcApi>,
  instructions: Instruction[],
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

(BigInt.prototype as unknown as { toJSON: () => string }).toJSON = function () {
  return this.toString();
};

async function getWriteablesAndSigners(_tx: Transaction) {
  const signerAccounts: Set<Address> = new Set();
  const writeableAccounts: Set<Address> = new Set();

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

export async function resolveAccount(
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
  _account: AccountInfo
): Promise<ResolvedSystemProgramAccount> {
  return { accountType: AccountType.SYSTEM_PROGRAM };
}
