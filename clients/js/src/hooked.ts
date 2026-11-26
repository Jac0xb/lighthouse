import {
  Serializer,
  mapSerializer,
} from '@metaplex-foundation/umi-serializers-core';
import { array, struct, u8 } from '@metaplex-foundation/umi/serializers';
import {
  AccountInfoAssertion,
  AccountInfoAssertionArgs,
  DataValueAssertion,
  MintAccountAssertion,
  MintAccountAssertionArgs,
  StakeAccountAssertion,
  StakeAccountAssertionArgs,
  TokenAccountAssertion,
  TokenAccountAssertionArgs,
  UpgradeableLoaderStateAssertion,
  UpgradeableLoaderStateAssertionArgs,
  getAccountInfoAssertionSerializer,
  getDataValueAssertionSerializer,
  getMintAccountAssertionSerializer,
  getStakeAccountAssertionSerializer,
  getTokenAccountAssertionSerializer,
  getUpgradeableLoaderStateAssertionSerializer,
} from './generated';

const MAX_U64 = 0xffff_ffff_ffff_ffffn;

function encodeUnsignedLeb128(value: bigint | number): Uint8Array {
  let n = typeof value === 'bigint' ? value : BigInt(value);
  if (n < 0n || n > MAX_U64) {
    throw new RangeError(`CompactU64 value out of range: ${value}`);
  }
  const bytes: number[] = [];
  do {
    let byte = Number(n & 0x7fn);
    n >>= 7n;
    if (n !== 0n) {
      byte |= 0x80;
    }
    bytes.push(byte);
  } while (n !== 0n);
  return new Uint8Array(bytes);
}

function decodeUnsignedLeb128(bytes: Uint8Array, offset = 0): [bigint, number] {
  let result = 0n;
  let shift = 0n;
  let pos = offset;
  while (pos < bytes.length) {
    const byte = bytes[pos++];
    if (shift === 63n && byte > 1) {
      throw new RangeError('CompactU64 LEB128 overflow');
    }
    result |= BigInt(byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) {
      return [result, pos];
    }
    shift += 7n;
    if (shift > 63n) {
      throw new RangeError('CompactU64 LEB128 overflow');
    }
  }
  throw new RangeError('Unexpected end of input while decoding CompactU64');
}

export type CompactU64 = number;
export type CompactU64Args = number;

export const getCompactU64Serializer = (): Serializer<number> => ({
  description: 'leb128',
  fixedSize: null,
  maxSize: 10,
  serialize: (value: number): Uint8Array => encodeUnsignedLeb128(value),
  deserialize: (bytes: Uint8Array, offset = 0): [number, number] => {
    const [value, nextOffset] = decodeUnsignedLeb128(bytes, offset);
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) {
      throw new RangeError('CompactU64 value exceeds Number.MAX_SAFE_INTEGER');
    }

    return [Number(value), nextOffset];
  },
});

export type AccountInfoAssertions = Array<AccountInfoAssertion>;
export type AccountInfoAssertionsArgs = Array<AccountInfoAssertionArgs>;

export function getAccountInfoAssertionsSerializer() {
  return array(getAccountInfoAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type MintAccountAssertions = Array<MintAccountAssertion>;
export type MintAccountAssertionsArgs = Array<MintAccountAssertionArgs>;

export function getMintAccountAssertionsSerializer() {
  return array(getMintAccountAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type StakeAccountAssertions = Array<StakeAccountAssertion>;
export type StakeAccountAssertionsArgs = Array<StakeAccountAssertionArgs>;

export function getStakeAccountAssertionsSerializer() {
  return array(getStakeAccountAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type TokenAccountAssertions = Array<TokenAccountAssertion>;
export type TokenAccountAssertionsArgs = Array<TokenAccountAssertionArgs>;

export function getTokenAccountAssertionsSerializer() {
  return array(getTokenAccountAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type UpgradeableLoaderStateAssertions =
  Array<UpgradeableLoaderStateAssertion>;
export type UpgradeableLoaderStateAssertionsArgs =
  Array<UpgradeableLoaderStateAssertionArgs>;

export function getUpgradeableLoaderStateAssertionsSerializer() {
  return array(getUpgradeableLoaderStateAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type AccountDataAssertion = {
  offset: number;
  assertion: DataValueAssertion;
};

export type AccountDataAssertionArgs = {
  offset: number;
  assertion: DataValueAssertion;
};

export function getAccountDataAssertionSerializer(): Serializer<
  AccountDataAssertionArgs,
  AccountDataAssertion
> {
  return mapSerializer<AccountDataAssertionArgs, any, AccountDataAssertion>(
    struct<AccountDataAssertion>(
      [
        ['offset', getCompactU64Serializer()],
        ['assertion', getDataValueAssertionSerializer()],
      ],
      { description: 'AccountDataAssertion' }
    ),
    (value) => value
  ) as Serializer<AccountDataAssertionArgs, AccountDataAssertion>;
}

export type AccountDataAssertions = Array<AccountDataAssertion>;
export type AccountDataAssertionsArgs = Array<AccountDataAssertionArgs>;

export function getAccountDataAssertionsSerializer() {
  return array(getAccountDataAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type CompactBytes = number[];
export type CompactBytesArgs = number[];

export function getCompactBytesSerializer() {
  return array(u8(), {
    size: getCompactU64Serializer(),
  });
}
