import {
  Serializer,
  mapSerializer,
} from '@metaplex-foundation/umi-serializers-core';
import { LEB128 } from '@minhducsun2002/leb128';
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
    (value) => ({
      ...value,
      discriminator: 2,
    })
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
