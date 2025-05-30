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
import {
  getCompactU64Decoder,
  getCompactU64Encoder,
  type CompactU64,
  type CompactU64Args,
} from '../../hooked';
import { LIGHTHOUSE_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';
import {
  LogLevel,
  getDataValueAssertionDecoder,
  getDataValueAssertionEncoder,
  getLogLevelDecoder,
  getLogLevelEncoder,
  type DataValueAssertion,
  type DataValueAssertionArgs,
  type LogLevelArgs,
} from '../types';

export const ASSERT_ACCOUNT_DATA_DISCRIMINATOR = 2;

export function getAssertAccountDataDiscriminatorBytes() {
  return getU8Encoder().encode(ASSERT_ACCOUNT_DATA_DISCRIMINATOR);
}

export type AssertAccountDataInstruction<
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

export type AssertAccountDataInstructionData = {
  discriminator: number;
  logLevel: LogLevel;
  offset: CompactU64;
  assertion: DataValueAssertion;
};

export type AssertAccountDataInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  offset: CompactU64Args;
  assertion: DataValueAssertionArgs;
};

export function getAssertAccountDataInstructionDataEncoder(): Encoder<AssertAccountDataInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['logLevel', getLogLevelEncoder()],
      ['offset', getCompactU64Encoder()],
      ['assertion', getDataValueAssertionEncoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: ASSERT_ACCOUNT_DATA_DISCRIMINATOR,
      logLevel: value.logLevel ?? LogLevel.Silent,
    })
  );
}

export function getAssertAccountDataInstructionDataDecoder(): Decoder<AssertAccountDataInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['logLevel', getLogLevelDecoder()],
    ['offset', getCompactU64Decoder()],
    ['assertion', getDataValueAssertionDecoder()],
  ]);
}

export function getAssertAccountDataInstructionDataCodec(): Codec<
  AssertAccountDataInstructionDataArgs,
  AssertAccountDataInstructionData
> {
  return combineCodec(
    getAssertAccountDataInstructionDataEncoder(),
    getAssertAccountDataInstructionDataDecoder()
  );
}

export type AssertAccountDataInput<
  TAccountTargetAccount extends string = string,
> = {
  /** Target account to be asserted */
  targetAccount: Address<TAccountTargetAccount>;
  logLevel?: AssertAccountDataInstructionDataArgs['logLevel'];
  offset: AssertAccountDataInstructionDataArgs['offset'];
  assertion: AssertAccountDataInstructionDataArgs['assertion'];
};

export function getAssertAccountDataInstruction<
  TAccountTargetAccount extends string,
  TProgramAddress extends Address = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
>(
  input: AssertAccountDataInput<TAccountTargetAccount>,
  config?: { programAddress?: TProgramAddress }
): AssertAccountDataInstruction<TProgramAddress, TAccountTargetAccount> {
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
    data: getAssertAccountDataInstructionDataEncoder().encode(
      args as AssertAccountDataInstructionDataArgs
    ),
  } as AssertAccountDataInstruction<TProgramAddress, TAccountTargetAccount>;

  return instruction;
}

export type ParsedAssertAccountDataInstruction<
  TProgram extends string = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Target account to be asserted */
    targetAccount: TAccountMetas[0];
  };
  data: AssertAccountDataInstructionData;
};

export function parseAssertAccountDataInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAssertAccountDataInstruction<TProgram, TAccountMetas> {
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
    data: getAssertAccountDataInstructionDataDecoder().decode(instruction.data),
  };
}
