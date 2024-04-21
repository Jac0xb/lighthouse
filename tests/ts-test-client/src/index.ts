import { Address, address } from '@solana/addresses';
import { AccountRole, WritableSignerAccount } from '@solana/instructions';
import {
  SystemProgram,
  Transaction,
  VersionedTransaction,
  TransactionInstruction,
  PublicKey,
} from '@solana/web3.js';
import fs from 'fs';
import {
  LIGHTHOUSE_PROGRAM_ID,
  getLighthouseErrorFromCode,
  createLighthouseProgram,
  assertAccountData,
  DataValueAssertionArgs,
  IntegerOperator,
  assertAccountInfo,
  EquatableOperator,
  KnownProgram,
  assertAccountInfoMulti,
  assertAccountDelta,
  memoryWrite,
  findMemoryPda,
  AccountInfoFieldArgs,
  AccountInfoField,
  assertMintAccount,
  assertMintAccountMulti,
  assertTokenAccountMulti,
  assertUpgradeableLoaderAccount,
  UpgradeableLoaderStateType,
  assertMerkleTreeAccount,
  assertStakeAccount,
  StakeStateType,
  memoryClose,
  WriteType,
  writeType,
  ClockField,
} from 'lighthouse-sdk-legacy';
import { AccountMeta, publicKey } from '@metaplex-foundation/umi';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { keccak_256 } from 'js-sha3';
import { toWeb3JsInstruction } from '@metaplex-foundation/umi-web3js-adapters';
import { u128, u8 } from '@metaplex-foundation/umi/serializers';

(async () => {
  const umi = createUmi('https://api.mainnet-beta.solana.com');
  umi.programs.add(createLighthouseProgram());
})();
