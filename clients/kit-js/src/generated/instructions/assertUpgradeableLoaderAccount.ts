/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
} from '@solana/kit';
import { LIGHTHOUSE_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';
import {
  LogLevel,
  getLogLevelDecoder,
  getLogLevelEncoder,
  getUpgradeableLoaderStateAssertionDecoder,
  getUpgradeableLoaderStateAssertionEncoder,
  type LogLevelArgs,
  type UpgradeableLoaderStateAssertion,
  type UpgradeableLoaderStateAssertionArgs,
} from '../types';

export const ASSERT_UPGRADEABLE_LOADER_ACCOUNT_DISCRIMINATOR = 13;

export function getAssertUpgradeableLoaderAccountDiscriminatorBytes() {
  return getU8Encoder().encode(ASSERT_UPGRADEABLE_LOADER_ACCOUNT_DISCRIMINATOR);
}

export type AssertUpgradeableLoaderAccountInstruction<
  TProgram extends string = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
  TAccountTargetAccount extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountTargetAccount extends string
        ? ReadonlyAccount<TAccountTargetAccount>
        : TAccountTargetAccount,
      ...TRemainingAccounts,
    ]
  >;

export type AssertUpgradeableLoaderAccountInstructionData = {
  discriminator: number;
  logLevel: LogLevel;
  assertion: UpgradeableLoaderStateAssertion;
};

export type AssertUpgradeableLoaderAccountInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  assertion: UpgradeableLoaderStateAssertionArgs;
};

export function getAssertUpgradeableLoaderAccountInstructionDataEncoder(): Encoder<AssertUpgradeableLoaderAccountInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['logLevel', getLogLevelEncoder()],
      ['assertion', getUpgradeableLoaderStateAssertionEncoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: ASSERT_UPGRADEABLE_LOADER_ACCOUNT_DISCRIMINATOR,
      logLevel: value.logLevel ?? LogLevel.Silent,
    })
  );
}

export function getAssertUpgradeableLoaderAccountInstructionDataDecoder(): Decoder<AssertUpgradeableLoaderAccountInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['logLevel', getLogLevelDecoder()],
    ['assertion', getUpgradeableLoaderStateAssertionDecoder()],
  ]);
}

export function getAssertUpgradeableLoaderAccountInstructionDataCodec(): Codec<
  AssertUpgradeableLoaderAccountInstructionDataArgs,
  AssertUpgradeableLoaderAccountInstructionData
> {
  return combineCodec(
    getAssertUpgradeableLoaderAccountInstructionDataEncoder(),
    getAssertUpgradeableLoaderAccountInstructionDataDecoder()
  );
}

export type AssertUpgradeableLoaderAccountInput<
  TAccountTargetAccount extends string = string,
> = {
  /** Target account to be asserted */
  targetAccount: Address<TAccountTargetAccount>;
  logLevel?: AssertUpgradeableLoaderAccountInstructionDataArgs['logLevel'];
  assertion: AssertUpgradeableLoaderAccountInstructionDataArgs['assertion'];
};

export function getAssertUpgradeableLoaderAccountInstruction<
  TAccountTargetAccount extends string,
  TProgramAddress extends Address = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
>(
  input: AssertUpgradeableLoaderAccountInput<TAccountTargetAccount>,
  config?: { programAddress?: TProgramAddress }
): AssertUpgradeableLoaderAccountInstruction<
  TProgramAddress,
  TAccountTargetAccount
> {
  // Program address.
  const programAddress = config?.programAddress ?? LIGHTHOUSE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [getAccountMeta(accounts.targetAccount)],
    programAddress,
    data: getAssertUpgradeableLoaderAccountInstructionDataEncoder().encode(
      args as AssertUpgradeableLoaderAccountInstructionDataArgs
    ),
  } as AssertUpgradeableLoaderAccountInstruction<
    TProgramAddress,
    TAccountTargetAccount
  >;

  return instruction;
}

export type ParsedAssertUpgradeableLoaderAccountInstruction<
  TProgram extends string = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Target account to be asserted */
    targetAccount: TAccountMetas[0];
  };
  data: AssertUpgradeableLoaderAccountInstructionData;
};

export function parseAssertUpgradeableLoaderAccountInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAssertUpgradeableLoaderAccountInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 1) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount(),
    },
    data: getAssertUpgradeableLoaderAccountInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
