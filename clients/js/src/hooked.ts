import { Serializer } from '@metaplex-foundation/umi-serializers-core';
import { LEB128 } from '@minhducsun2002/leb128';
import { array, u8 } from '@metaplex-foundation/umi/serializers';
import {
  AccountInfoAssertion,
  MintAccountAssertion,
  StakeAccountAssertion,
  TokenAccountAssertion,
  UpgradeableLoaderStateAssertion,
  getAccountInfoAssertionSerializer,
  getMintAccountAssertionSerializer,
  getStakeAccountAssertionSerializer,
  getTokenAccountAssertionSerializer,
  getUpgradeableLoaderStateAssertionSerializer,
} from './generated';

export type CompactU64 = number;
export type CompactU64Args = number;

export const getCompactU64Serializer = (): Serializer<number> => ({
  description: 'leb128',
  fixedSize: null,
  maxSize: null,
  serialize: (value: number): Uint8Array => LEB128.encode(value),
  deserialize: (bytes: Uint8Array, offset = 0): [number, number] => {
    const value = LEB128.decode(bytes as Uint8Array, offset);

    return [value, offset + LEB128.encode(value).length];
  },
});

export type AccountInfoAssertions = Array<AccountInfoAssertion>;
export type AccountInfoAssertionsArgs = Array<AccountInfoAssertion>;

export function getAccountInfoAssertionsSerializer() {
  return array(getAccountInfoAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type MintAccountAssertions = Array<MintAccountAssertion>;
export type MintAccountAssertionsArgs = Array<MintAccountAssertion>;

export function getMintAccountAssertionsSerializer() {
  return array(getMintAccountAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type StakeAccountAssertions = Array<StakeAccountAssertion>;
export type StakeAccountAssertionsArgs = Array<StakeAccountAssertion>;

export function getStakeAccountAssertionsSerializer() {
  return array(getStakeAccountAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type TokenAccountAssertions = Array<TokenAccountAssertion>;
export type TokenAccountAssertionsArgs = Array<TokenAccountAssertion>;

export function getTokenAccountAssertionsSerializer() {
  return array(getTokenAccountAssertionSerializer(), {
    size: getCompactU64Serializer(),
  });
}

export type UpgradeableLoaderStateAssertions =
  Array<UpgradeableLoaderStateAssertion>;
export type UpgradeableLoaderStateAssertionsArgs =
  Array<UpgradeableLoaderStateAssertion>;

export function getUpgradeableLoaderStateAssertionsSerializer() {
  return array(getUpgradeableLoaderStateAssertionSerializer(), {
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
