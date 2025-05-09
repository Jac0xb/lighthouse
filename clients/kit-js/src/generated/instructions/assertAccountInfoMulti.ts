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
  getAccountInfoAssertionsDecoder,
  getAccountInfoAssertionsEncoder,
  type AccountInfoAssertions,
  type AccountInfoAssertionsArgs,
} from '../../hooked';
import { LIGHTHOUSE_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';
import {
  LogLevel,
  getLogLevelDecoder,
  getLogLevelEncoder,
  type LogLevelArgs,
} from '../types';

export const ASSERT_ACCOUNT_INFO_MULTI_DISCRIMINATOR = 6;

export function getAssertAccountInfoMultiDiscriminatorBytes() {
  return getU8Encoder().encode(ASSERT_ACCOUNT_INFO_MULTI_DISCRIMINATOR);
}

export type AssertAccountInfoMultiInstruction<
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

export type AssertAccountInfoMultiInstructionData = {
  discriminator: number;
  logLevel: LogLevel;
  assertions: AccountInfoAssertions;
};

export type AssertAccountInfoMultiInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  assertions: AccountInfoAssertionsArgs;
};

export function getAssertAccountInfoMultiInstructionDataEncoder(): Encoder<AssertAccountInfoMultiInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['logLevel', getLogLevelEncoder()],
      ['assertions', getAccountInfoAssertionsEncoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: ASSERT_ACCOUNT_INFO_MULTI_DISCRIMINATOR,
      logLevel: value.logLevel ?? LogLevel.Silent,
    })
  );
}

export function getAssertAccountInfoMultiInstructionDataDecoder(): Decoder<AssertAccountInfoMultiInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['logLevel', getLogLevelDecoder()],
    ['assertions', getAccountInfoAssertionsDecoder()],
  ]);
}

export function getAssertAccountInfoMultiInstructionDataCodec(): Codec<
  AssertAccountInfoMultiInstructionDataArgs,
  AssertAccountInfoMultiInstructionData
> {
  return combineCodec(
    getAssertAccountInfoMultiInstructionDataEncoder(),
    getAssertAccountInfoMultiInstructionDataDecoder()
  );
}

export type AssertAccountInfoMultiInput<
  TAccountTargetAccount extends string = string,
> = {
  /** Target account to be asserted */
  targetAccount: Address<TAccountTargetAccount>;
  logLevel?: AssertAccountInfoMultiInstructionDataArgs['logLevel'];
  assertions: AssertAccountInfoMultiInstructionDataArgs['assertions'];
};

export function getAssertAccountInfoMultiInstruction<
  TAccountTargetAccount extends string,
  TProgramAddress extends Address = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
>(
  input: AssertAccountInfoMultiInput<TAccountTargetAccount>,
  config?: { programAddress?: TProgramAddress }
): AssertAccountInfoMultiInstruction<TProgramAddress, TAccountTargetAccount> {
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
    data: getAssertAccountInfoMultiInstructionDataEncoder().encode(
      args as AssertAccountInfoMultiInstructionDataArgs
    ),
  } as AssertAccountInfoMultiInstruction<
    TProgramAddress,
    TAccountTargetAccount
  >;

  return instruction;
}

export type ParsedAssertAccountInfoMultiInstruction<
  TProgram extends string = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Target account to be asserted */
    targetAccount: TAccountMetas[0];
  };
  data: AssertAccountInfoMultiInstructionData;
};

export function parseAssertAccountInfoMultiInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAssertAccountInfoMultiInstruction<TProgram, TAccountMetas> {
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
    data: getAssertAccountInfoMultiInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
