/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getAddressDecoder,
  getAddressEncoder,
  getBooleanDecoder,
  getBooleanEncoder,
  getDiscriminatedUnionDecoder,
  getDiscriminatedUnionEncoder,
  getI128Decoder,
  getI128Encoder,
  getI16Decoder,
  getI16Encoder,
  getI32Decoder,
  getI32Encoder,
  getI64Decoder,
  getI64Encoder,
  getI8Decoder,
  getI8Encoder,
  getStructDecoder,
  getStructEncoder,
  getU128Decoder,
  getU128Encoder,
  getU16Decoder,
  getU16Encoder,
  getU32Decoder,
  getU32Encoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type GetDiscriminatedUnionVariant,
  type GetDiscriminatedUnionVariantContent,
} from '@solana/kit';
import {
  getEquatableOperatorDecoder,
  getEquatableOperatorEncoder,
  getIntegerOperatorDecoder,
  getIntegerOperatorEncoder,
  type EquatableOperator,
  type EquatableOperatorArgs,
  type IntegerOperator,
  type IntegerOperatorArgs,
} from '.';
import {
  getCompactBytesDecoder,
  getCompactBytesEncoder,
  type CompactBytes,
  type CompactBytesArgs,
} from '../../hooked';

export type DataValueAssertion =
  | { __kind: 'Bool'; value: boolean; operator: EquatableOperator }
  | { __kind: 'U8'; value: number; operator: IntegerOperator }
  | { __kind: 'I8'; value: number; operator: IntegerOperator }
  | { __kind: 'U16'; value: number; operator: IntegerOperator }
  | { __kind: 'I16'; value: number; operator: IntegerOperator }
  | { __kind: 'U32'; value: number; operator: IntegerOperator }
  | { __kind: 'I32'; value: number; operator: IntegerOperator }
  | { __kind: 'U64'; value: bigint; operator: IntegerOperator }
  | { __kind: 'I64'; value: bigint; operator: IntegerOperator }
  | { __kind: 'U128'; value: bigint; operator: IntegerOperator }
  | { __kind: 'I128'; value: bigint; operator: IntegerOperator }
  | { __kind: 'Bytes'; value: CompactBytes; operator: EquatableOperator }
  | { __kind: 'Pubkey'; value: Address; operator: EquatableOperator };

export type DataValueAssertionArgs =
  | { __kind: 'Bool'; value: boolean; operator: EquatableOperatorArgs }
  | { __kind: 'U8'; value: number; operator: IntegerOperatorArgs }
  | { __kind: 'I8'; value: number; operator: IntegerOperatorArgs }
  | { __kind: 'U16'; value: number; operator: IntegerOperatorArgs }
  | { __kind: 'I16'; value: number; operator: IntegerOperatorArgs }
  | { __kind: 'U32'; value: number; operator: IntegerOperatorArgs }
  | { __kind: 'I32'; value: number; operator: IntegerOperatorArgs }
  | { __kind: 'U64'; value: number | bigint; operator: IntegerOperatorArgs }
  | { __kind: 'I64'; value: number | bigint; operator: IntegerOperatorArgs }
  | { __kind: 'U128'; value: number | bigint; operator: IntegerOperatorArgs }
  | { __kind: 'I128'; value: number | bigint; operator: IntegerOperatorArgs }
  | {
      __kind: 'Bytes';
      value: CompactBytesArgs;
      operator: EquatableOperatorArgs;
    }
  | { __kind: 'Pubkey'; value: Address; operator: EquatableOperatorArgs };

export function getDataValueAssertionEncoder(): Encoder<DataValueAssertionArgs> {
  return getDiscriminatedUnionEncoder([
    [
      'Bool',
      getStructEncoder([
        ['value', getBooleanEncoder()],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
    [
      'U8',
      getStructEncoder([
        ['value', getU8Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'I8',
      getStructEncoder([
        ['value', getI8Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'U16',
      getStructEncoder([
        ['value', getU16Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'I16',
      getStructEncoder([
        ['value', getI16Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'U32',
      getStructEncoder([
        ['value', getU32Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'I32',
      getStructEncoder([
        ['value', getI32Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'U64',
      getStructEncoder([
        ['value', getU64Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'I64',
      getStructEncoder([
        ['value', getI64Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'U128',
      getStructEncoder([
        ['value', getU128Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'I128',
      getStructEncoder([
        ['value', getI128Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'Bytes',
      getStructEncoder([
        ['value', getCompactBytesEncoder()],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
    [
      'Pubkey',
      getStructEncoder([
        ['value', getAddressEncoder()],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
  ]);
}

export function getDataValueAssertionDecoder(): Decoder<DataValueAssertion> {
  return getDiscriminatedUnionDecoder([
    [
      'Bool',
      getStructDecoder([
        ['value', getBooleanDecoder()],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
    [
      'U8',
      getStructDecoder([
        ['value', getU8Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'I8',
      getStructDecoder([
        ['value', getI8Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'U16',
      getStructDecoder([
        ['value', getU16Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'I16',
      getStructDecoder([
        ['value', getI16Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'U32',
      getStructDecoder([
        ['value', getU32Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'I32',
      getStructDecoder([
        ['value', getI32Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'U64',
      getStructDecoder([
        ['value', getU64Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'I64',
      getStructDecoder([
        ['value', getI64Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'U128',
      getStructDecoder([
        ['value', getU128Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'I128',
      getStructDecoder([
        ['value', getI128Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'Bytes',
      getStructDecoder([
        ['value', getCompactBytesDecoder()],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
    [
      'Pubkey',
      getStructDecoder([
        ['value', getAddressDecoder()],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
  ]);
}

export function getDataValueAssertionCodec(): Codec<
  DataValueAssertionArgs,
  DataValueAssertion
> {
  return combineCodec(
    getDataValueAssertionEncoder(),
    getDataValueAssertionDecoder()
  );
}

// Data Enum Helpers.
export function dataValueAssertion(
  kind: 'Bool',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'Bool'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'Bool'>;
export function dataValueAssertion(
  kind: 'U8',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'U8'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'U8'>;
export function dataValueAssertion(
  kind: 'I8',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'I8'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'I8'>;
export function dataValueAssertion(
  kind: 'U16',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'U16'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'U16'>;
export function dataValueAssertion(
  kind: 'I16',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'I16'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'I16'>;
export function dataValueAssertion(
  kind: 'U32',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'U32'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'U32'>;
export function dataValueAssertion(
  kind: 'I32',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'I32'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'I32'>;
export function dataValueAssertion(
  kind: 'U64',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'U64'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'U64'>;
export function dataValueAssertion(
  kind: 'I64',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'I64'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'I64'>;
export function dataValueAssertion(
  kind: 'U128',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'U128'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'U128'>;
export function dataValueAssertion(
  kind: 'I128',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'I128'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'I128'>;
export function dataValueAssertion(
  kind: 'Bytes',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'Bytes'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'Bytes'>;
export function dataValueAssertion(
  kind: 'Pubkey',
  data: GetDiscriminatedUnionVariantContent<
    DataValueAssertionArgs,
    '__kind',
    'Pubkey'
  >
): GetDiscriminatedUnionVariant<DataValueAssertionArgs, '__kind', 'Pubkey'>;
export function dataValueAssertion<
  K extends DataValueAssertionArgs['__kind'],
  Data,
>(kind: K, data?: Data) {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}

export function isDataValueAssertion<K extends DataValueAssertion['__kind']>(
  kind: K,
  value: DataValueAssertion
): value is DataValueAssertion & { __kind: K } {
  return value.__kind === kind;
}
