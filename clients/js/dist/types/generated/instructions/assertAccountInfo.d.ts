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
import { AccountInfoAssertion, AccountInfoAssertionArgs, LogLevel, LogLevelArgs } from '../types';
export type AssertAccountInfoInstruction<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []> = IInstruction<TProgram> & IInstructionWithData<Uint8Array> & IInstructionWithAccounts<[
    TAccountTargetAccount extends string ? ReadonlyAccount<TAccountTargetAccount> : TAccountTargetAccount,
    ...TRemainingAccounts
]>;
export type AssertAccountInfoInstructionWithSigners<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []> = IInstruction<TProgram> & IInstructionWithData<Uint8Array> & IInstructionWithAccounts<[
    TAccountTargetAccount extends string ? ReadonlyAccount<TAccountTargetAccount> : TAccountTargetAccount,
    ...TRemainingAccounts
]>;
export type AssertAccountInfoInstructionData = {
    discriminator: number;
    logLevel: LogLevel;
    assertion: AccountInfoAssertion;
};
export type AssertAccountInfoInstructionDataArgs = {
    logLevel?: LogLevelArgs;
    assertion: AccountInfoAssertionArgs;
};
export declare function getAssertAccountInfoInstructionDataEncoder(): Encoder<AssertAccountInfoInstructionDataArgs>;
export declare function getAssertAccountInfoInstructionDataDecoder(): Decoder<AssertAccountInfoInstructionData>;
export declare function getAssertAccountInfoInstructionDataCodec(): Codec<AssertAccountInfoInstructionDataArgs, AssertAccountInfoInstructionData>;
export type AssertAccountInfoInput<TAccountTargetAccount extends string> = {
    /** Target account to be asserted */
    targetAccount: Address<TAccountTargetAccount>;
    logLevel?: AssertAccountInfoInstructionDataArgs['logLevel'];
    assertion: AssertAccountInfoInstructionDataArgs['assertion'];
};
export type AssertAccountInfoInputWithSigners<TAccountTargetAccount extends string> = {
    /** Target account to be asserted */
    targetAccount: Address<TAccountTargetAccount>;
    logLevel?: AssertAccountInfoInstructionDataArgs['logLevel'];
    assertion: AssertAccountInfoInstructionDataArgs['assertion'];
};
export declare function getAssertAccountInfoInstruction<TAccountTargetAccount extends string, TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'>(input: AssertAccountInfoInputWithSigners<TAccountTargetAccount>): AssertAccountInfoInstructionWithSigners<TProgram, TAccountTargetAccount>;
export declare function getAssertAccountInfoInstruction<TAccountTargetAccount extends string, TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'>(input: AssertAccountInfoInput<TAccountTargetAccount>): AssertAccountInfoInstruction<TProgram, TAccountTargetAccount>;
export declare function getAssertAccountInfoInstructionRaw<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []>(accounts: {
    targetAccount: TAccountTargetAccount extends string ? Address<TAccountTargetAccount> : TAccountTargetAccount;
}, args: AssertAccountInfoInstructionDataArgs, programAddress?: Address<TProgram>, remainingAccounts?: TRemainingAccounts): AssertAccountInfoInstruction<TProgram, TAccountTargetAccount, TRemainingAccounts>;
export type ParsedAssertAccountInfoInstruction<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[]> = {
    programAddress: Address<TProgram>;
    accounts: {
        /** Target account to be asserted */
        targetAccount: TAccountMetas[0];
    };
    data: AssertAccountInfoInstructionData;
};
export declare function parseAssertAccountInfoInstruction<TProgram extends string, TAccountMetas extends readonly IAccountMeta[]>(instruction: IInstruction<TProgram> & IInstructionWithAccounts<TAccountMetas> & IInstructionWithData<Uint8Array>): ParsedAssertAccountInfoInstruction<TProgram, TAccountMetas>;
//# sourceMappingURL=assertAccountInfo.d.ts.map