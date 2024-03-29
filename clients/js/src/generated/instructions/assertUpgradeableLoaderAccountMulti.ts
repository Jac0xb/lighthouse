/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  mapEncoder,
} from '@solana/codecs';
import {
  AccountRole,
  IAccountMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
  ReadonlyAccount,
} from '@solana/instructions';
import {
  ResolvedAccount,
  accountMetaWithDefault,
  getAccountMetasWithSigners,
} from '../shared';
import {
  LogLevel,
  LogLevelArgs,
  UpgradeableLoaderStateAssertion,
  UpgradeableLoaderStateAssertionArgs,
  getLogLevelDecoder,
  getLogLevelEncoder,
  getUpgradeableLoaderStateAssertionDecoder,
  getUpgradeableLoaderStateAssertionEncoder,
} from '../types';

export type AssertUpgradeableLoaderAccountMultiInstruction<
  TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK',
  TAccountTargetAccount extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends Array<IAccountMeta<string>> = []
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountTargetAccount extends string
        ? ReadonlyAccount<TAccountTargetAccount>
        : TAccountTargetAccount,
      ...TRemainingAccounts
    ]
  >;

export type AssertUpgradeableLoaderAccountMultiInstructionWithSigners<
  TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK',
  TAccountTargetAccount extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends Array<IAccountMeta<string>> = []
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountTargetAccount extends string
        ? ReadonlyAccount<TAccountTargetAccount>
        : TAccountTargetAccount,
      ...TRemainingAccounts
    ]
  >;

export type AssertUpgradeableLoaderAccountMultiInstructionData = {
  discriminator: number;
  logLevel: LogLevel;
  assertions: Array<UpgradeableLoaderStateAssertion>;
};

export type AssertUpgradeableLoaderAccountMultiInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  assertions: Array<UpgradeableLoaderStateAssertionArgs>;
};

export function getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder(): Encoder<AssertUpgradeableLoaderAccountMultiInstructionDataArgs> {
  return mapEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['logLevel', getLogLevelEncoder()],
      [
        'assertions',
        getArrayEncoder(getUpgradeableLoaderStateAssertionEncoder()),
      ],
    ]),
    (value) => ({
      ...value,
      discriminator: 13,
      logLevel: value.logLevel ?? LogLevel.Silent,
    })
  );
}

export function getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder(): Decoder<AssertUpgradeableLoaderAccountMultiInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['logLevel', getLogLevelDecoder()],
    [
      'assertions',
      getArrayDecoder(getUpgradeableLoaderStateAssertionDecoder()),
    ],
  ]);
}

export function getAssertUpgradeableLoaderAccountMultiInstructionDataCodec(): Codec<
  AssertUpgradeableLoaderAccountMultiInstructionDataArgs,
  AssertUpgradeableLoaderAccountMultiInstructionData
> {
  return combineCodec(
    getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder(),
    getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder()
  );
}

export type AssertUpgradeableLoaderAccountMultiInput<
  TAccountTargetAccount extends string
> = {
  /** Target account to be asserted */
  targetAccount: Address<TAccountTargetAccount>;
  logLevel?: AssertUpgradeableLoaderAccountMultiInstructionDataArgs['logLevel'];
  assertions: AssertUpgradeableLoaderAccountMultiInstructionDataArgs['assertions'];
};

export type AssertUpgradeableLoaderAccountMultiInputWithSigners<
  TAccountTargetAccount extends string
> = {
  /** Target account to be asserted */
  targetAccount: Address<TAccountTargetAccount>;
  logLevel?: AssertUpgradeableLoaderAccountMultiInstructionDataArgs['logLevel'];
  assertions: AssertUpgradeableLoaderAccountMultiInstructionDataArgs['assertions'];
};

export function getAssertUpgradeableLoaderAccountMultiInstruction<
  TAccountTargetAccount extends string,
  TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'
>(
  input: AssertUpgradeableLoaderAccountMultiInputWithSigners<TAccountTargetAccount>
): AssertUpgradeableLoaderAccountMultiInstructionWithSigners<
  TProgram,
  TAccountTargetAccount
>;
export function getAssertUpgradeableLoaderAccountMultiInstruction<
  TAccountTargetAccount extends string,
  TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'
>(
  input: AssertUpgradeableLoaderAccountMultiInput<TAccountTargetAccount>
): AssertUpgradeableLoaderAccountMultiInstruction<
  TProgram,
  TAccountTargetAccount
>;
export function getAssertUpgradeableLoaderAccountMultiInstruction<
  TAccountTargetAccount extends string,
  TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'
>(
  input: AssertUpgradeableLoaderAccountMultiInput<TAccountTargetAccount>
): IInstruction {
  // Program address.
  const programAddress =
    'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK' as Address<'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'>;

  // Original accounts.
  type AccountMetas = Parameters<
    typeof getAssertUpgradeableLoaderAccountMultiInstructionRaw<
      TProgram,
      TAccountTargetAccount
    >
  >[0];
  const accounts: Record<keyof AccountMetas, ResolvedAccount> = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false },
  };

  // Original args.
  const args = { ...input };

  // Get account metas and signers.
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    'programId',
    programAddress
  );

  const instruction = getAssertUpgradeableLoaderAccountMultiInstructionRaw(
    accountMetas as Record<keyof AccountMetas, IAccountMeta>,
    args as AssertUpgradeableLoaderAccountMultiInstructionDataArgs,
    programAddress
  );

  return instruction;
}

export function getAssertUpgradeableLoaderAccountMultiInstructionRaw<
  TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK',
  TAccountTargetAccount extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends Array<IAccountMeta<string>> = []
>(
  accounts: {
    targetAccount: TAccountTargetAccount extends string
      ? Address<TAccountTargetAccount>
      : TAccountTargetAccount;
  },
  args: AssertUpgradeableLoaderAccountMultiInstructionDataArgs,
  programAddress: Address<TProgram> = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK' as Address<TProgram>,
  remainingAccounts?: TRemainingAccounts
) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
      ...(remainingAccounts ?? []),
    ],
    data: getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder().encode(
      args
    ),
    programAddress,
  } as AssertUpgradeableLoaderAccountMultiInstruction<
    TProgram,
    TAccountTargetAccount,
    TRemainingAccounts
  >;
}

export type ParsedAssertUpgradeableLoaderAccountMultiInstruction<
  TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK',
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[]
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Target account to be asserted */
    targetAccount: TAccountMetas[0];
  };
  data: AssertUpgradeableLoaderAccountMultiInstructionData;
};

export function parseAssertUpgradeableLoaderAccountMultiInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[]
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedAssertUpgradeableLoaderAccountMultiInstruction<
  TProgram,
  TAccountMetas
> {
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
    data: getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
