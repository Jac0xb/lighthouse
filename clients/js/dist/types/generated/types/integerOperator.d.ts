/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Codec, Decoder, Encoder } from '@solana/codecs';
export declare enum IntegerOperator {
    Equal = 0,
    NotEqual = 1,
    GreaterThan = 2,
    LessThan = 3,
    GreaterThanOrEqual = 4,
    LessThanOrEqual = 5,
    Contains = 6,
    DoesNotContain = 7
}
export type IntegerOperatorArgs = IntegerOperator;
export declare function getIntegerOperatorEncoder(): Encoder<IntegerOperatorArgs>;
export declare function getIntegerOperatorDecoder(): Decoder<IntegerOperator>;
export declare function getIntegerOperatorCodec(): Codec<IntegerOperatorArgs, IntegerOperator>;
//# sourceMappingURL=integerOperator.d.ts.map