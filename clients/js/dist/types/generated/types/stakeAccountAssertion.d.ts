/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Codec, Decoder, Encoder, GetDataEnumKind, GetDataEnumKindContent } from '@solana/codecs';
import { EquatableOperator, EquatableOperatorArgs, IntegerOperator, IntegerOperatorArgs, MetaAssertion, MetaAssertionArgs, StakeAssertion, StakeAssertionArgs, StakeStateType, StakeStateTypeArgs } from '.';
export type StakeAccountAssertion = {
    __kind: 'State';
    value: StakeStateType;
    operator: EquatableOperator;
} | {
    __kind: 'MetaAssertion';
    fields: [MetaAssertion];
} | {
    __kind: 'StakeAssertion';
    fields: [StakeAssertion];
} | {
    __kind: 'StakeFlags';
    value: number;
    operator: IntegerOperator;
};
export type StakeAccountAssertionArgs = {
    __kind: 'State';
    value: StakeStateTypeArgs;
    operator: EquatableOperatorArgs;
} | {
    __kind: 'MetaAssertion';
    fields: [MetaAssertionArgs];
} | {
    __kind: 'StakeAssertion';
    fields: [StakeAssertionArgs];
} | {
    __kind: 'StakeFlags';
    value: number;
    operator: IntegerOperatorArgs;
};
export declare function getStakeAccountAssertionEncoder(): Encoder<StakeAccountAssertionArgs>;
export declare function getStakeAccountAssertionDecoder(): Decoder<StakeAccountAssertion>;
export declare function getStakeAccountAssertionCodec(): Codec<StakeAccountAssertionArgs, StakeAccountAssertion>;
export declare function stakeAccountAssertion(kind: 'State', data: GetDataEnumKindContent<StakeAccountAssertionArgs, 'State'>): GetDataEnumKind<StakeAccountAssertionArgs, 'State'>;
export declare function stakeAccountAssertion(kind: 'MetaAssertion', data: GetDataEnumKindContent<StakeAccountAssertionArgs, 'MetaAssertion'>['fields']): GetDataEnumKind<StakeAccountAssertionArgs, 'MetaAssertion'>;
export declare function stakeAccountAssertion(kind: 'StakeAssertion', data: GetDataEnumKindContent<StakeAccountAssertionArgs, 'StakeAssertion'>['fields']): GetDataEnumKind<StakeAccountAssertionArgs, 'StakeAssertion'>;
export declare function stakeAccountAssertion(kind: 'StakeFlags', data: GetDataEnumKindContent<StakeAccountAssertionArgs, 'StakeFlags'>): GetDataEnumKind<StakeAccountAssertionArgs, 'StakeFlags'>;
export declare function isStakeAccountAssertion<K extends StakeAccountAssertion['__kind']>(kind: K, value: StakeAccountAssertion): value is StakeAccountAssertion & {
    __kind: K;
};
//# sourceMappingURL=stakeAccountAssertion.d.ts.map