/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Address } from '@solana/addresses';
import { Codec, Decoder, Encoder } from '@solana/codecs';
import { IAccountMeta, IInstruction, IInstructionWithAccounts, IInstructionWithData, ReadonlyAccount } from '@solana/instructions';
import { LogLevel, LogLevelArgs, StakeAccountAssertion, StakeAccountAssertionArgs } from '../types';
export type AssertStakeAccountMultiInstruction<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []> = IInstruction<TProgram> & IInstructionWithData<Uint8Array> & IInstructionWithAccounts<[
    TAccountTargetAccount extends string ? ReadonlyAccount<TAccountTargetAccount> : TAccountTargetAccount,
    ...TRemainingAccounts
]>;
export type AssertStakeAccountMultiInstructionWithSigners<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []> = IInstruction<TProgram> & IInstructionWithData<Uint8Array> & IInstructionWithAccounts<[
    TAccountTargetAccount extends string ? ReadonlyAccount<TAccountTargetAccount> : TAccountTargetAccount,
    ...TRemainingAccounts
]>;
export type AssertStakeAccountMultiInstructionData = {
    discriminator: number;
    logLevel: LogLevel;
    assertions: Array<StakeAccountAssertion>;
};
export type AssertStakeAccountMultiInstructionDataArgs = {
    logLevel?: LogLevelArgs;
    assertions: Array<StakeAccountAssertionArgs>;
};
export declare function getAssertStakeAccountMultiInstructionDataEncoder(): Encoder<AssertStakeAccountMultiInstructionDataArgs>;
export declare function getAssertStakeAccountMultiInstructionDataDecoder(): Decoder<AssertStakeAccountMultiInstructionData>;
export declare function getAssertStakeAccountMultiInstructionDataCodec(): Codec<AssertStakeAccountMultiInstructionDataArgs, AssertStakeAccountMultiInstructionData>;
export type AssertStakeAccountMultiInput<TAccountTargetAccount extends string> = {
    /** Target account to be asserted */
    targetAccount: Address<TAccountTargetAccount>;
    logLevel?: AssertStakeAccountMultiInstructionDataArgs['logLevel'];
    assertions: AssertStakeAccountMultiInstructionDataArgs['assertions'];
};
export type AssertStakeAccountMultiInputWithSigners<TAccountTargetAccount extends string> = {
    /** Target account to be asserted */
    targetAccount: Address<TAccountTargetAccount>;
    logLevel?: AssertStakeAccountMultiInstructionDataArgs['logLevel'];
    assertions: AssertStakeAccountMultiInstructionDataArgs['assertions'];
};
export declare function getAssertStakeAccountMultiInstruction<TAccountTargetAccount extends string, TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'>(input: AssertStakeAccountMultiInputWithSigners<TAccountTargetAccount>): AssertStakeAccountMultiInstructionWithSigners<TProgram, TAccountTargetAccount>;
export declare function getAssertStakeAccountMultiInstruction<TAccountTargetAccount extends string, TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'>(input: AssertStakeAccountMultiInput<TAccountTargetAccount>): AssertStakeAccountMultiInstruction<TProgram, TAccountTargetAccount>;
export declare function getAssertStakeAccountMultiInstructionRaw<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []>(accounts: {
    targetAccount: TAccountTargetAccount extends string ? Address<TAccountTargetAccount> : TAccountTargetAccount;
}, args: AssertStakeAccountMultiInstructionDataArgs, programAddress?: Address<TProgram>, remainingAccounts?: TRemainingAccounts): AssertStakeAccountMultiInstruction<TProgram, TAccountTargetAccount, TRemainingAccounts>;
export type ParsedAssertStakeAccountMultiInstruction<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[]> = {
    programAddress: Address<TProgram>;
    accounts: {
        /** Target account to be asserted */
        targetAccount: TAccountMetas[0];
    };
    data: AssertStakeAccountMultiInstructionData;
};
export declare function parseAssertStakeAccountMultiInstruction<TProgram extends string, TAccountMetas extends readonly IAccountMeta[]>(instruction: IInstruction<TProgram> & IInstructionWithAccounts<TAccountMetas> & IInstructionWithData<Uint8Array>): ParsedAssertStakeAccountMultiInstruction<TProgram, TAccountMetas>;
//# sourceMappingURL=assertStakeAccountMulti.d.ts.map