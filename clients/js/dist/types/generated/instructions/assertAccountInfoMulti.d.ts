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
export type AssertAccountInfoMultiInstruction<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []> = IInstruction<TProgram> & IInstructionWithData<Uint8Array> & IInstructionWithAccounts<[
    TAccountTargetAccount extends string ? ReadonlyAccount<TAccountTargetAccount> : TAccountTargetAccount,
    ...TRemainingAccounts
]>;
export type AssertAccountInfoMultiInstructionWithSigners<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []> = IInstruction<TProgram> & IInstructionWithData<Uint8Array> & IInstructionWithAccounts<[
    TAccountTargetAccount extends string ? ReadonlyAccount<TAccountTargetAccount> : TAccountTargetAccount,
    ...TRemainingAccounts
]>;
export type AssertAccountInfoMultiInstructionData = {
    discriminator: number;
    logLevel: LogLevel;
    assertions: Array<AccountInfoAssertion>;
};
export type AssertAccountInfoMultiInstructionDataArgs = {
    logLevel?: LogLevelArgs;
    assertions: Array<AccountInfoAssertionArgs>;
};
export declare function getAssertAccountInfoMultiInstructionDataEncoder(): Encoder<AssertAccountInfoMultiInstructionDataArgs>;
export declare function getAssertAccountInfoMultiInstructionDataDecoder(): Decoder<AssertAccountInfoMultiInstructionData>;
export declare function getAssertAccountInfoMultiInstructionDataCodec(): Codec<AssertAccountInfoMultiInstructionDataArgs, AssertAccountInfoMultiInstructionData>;
export type AssertAccountInfoMultiInput<TAccountTargetAccount extends string> = {
    /** Target account to be asserted */
    targetAccount: Address<TAccountTargetAccount>;
    logLevel?: AssertAccountInfoMultiInstructionDataArgs['logLevel'];
    assertions: AssertAccountInfoMultiInstructionDataArgs['assertions'];
};
export type AssertAccountInfoMultiInputWithSigners<TAccountTargetAccount extends string> = {
    /** Target account to be asserted */
    targetAccount: Address<TAccountTargetAccount>;
    logLevel?: AssertAccountInfoMultiInstructionDataArgs['logLevel'];
    assertions: AssertAccountInfoMultiInstructionDataArgs['assertions'];
};
export declare function getAssertAccountInfoMultiInstruction<TAccountTargetAccount extends string, TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'>(input: AssertAccountInfoMultiInputWithSigners<TAccountTargetAccount>): AssertAccountInfoMultiInstructionWithSigners<TProgram, TAccountTargetAccount>;
export declare function getAssertAccountInfoMultiInstruction<TAccountTargetAccount extends string, TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK'>(input: AssertAccountInfoMultiInput<TAccountTargetAccount>): AssertAccountInfoMultiInstruction<TProgram, TAccountTargetAccount>;
export declare function getAssertAccountInfoMultiInstructionRaw<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountTargetAccount extends string | IAccountMeta<string> = string, TRemainingAccounts extends Array<IAccountMeta<string>> = []>(accounts: {
    targetAccount: TAccountTargetAccount extends string ? Address<TAccountTargetAccount> : TAccountTargetAccount;
}, args: AssertAccountInfoMultiInstructionDataArgs, programAddress?: Address<TProgram>, remainingAccounts?: TRemainingAccounts): AssertAccountInfoMultiInstruction<TProgram, TAccountTargetAccount, TRemainingAccounts>;
export type ParsedAssertAccountInfoMultiInstruction<TProgram extends string = 'L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK', TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[]> = {
    programAddress: Address<TProgram>;
    accounts: {
        /** Target account to be asserted */
        targetAccount: TAccountMetas[0];
    };
    data: AssertAccountInfoMultiInstructionData;
};
export declare function parseAssertAccountInfoMultiInstruction<TProgram extends string, TAccountMetas extends readonly IAccountMeta[]>(instruction: IInstruction<TProgram> & IInstructionWithAccounts<TAccountMetas> & IInstructionWithData<Uint8Array>): ParsedAssertAccountInfoMultiInstruction<TProgram, TAccountMetas>;
//# sourceMappingURL=assertAccountInfoMulti.d.ts.map