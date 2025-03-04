import {
  Decoder,
  Encoder,
  Offset,
  ReadonlyUint8Array,
  VariableSizeDecoder,
  VariableSizeEncoder,
  createDecoder,
  createEncoder,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
} from '@solana/web3.js';
import { LEB128, UnsignedLEB128 } from '@minhducsun2002/leb128';
import {
  AccountInfoAssertion,
  AccountInfoAssertionArgs,
  AssertAccountDataInstructionData,
  DataValueAssertion,
  DataValueAssertionArgs,
  MintAccountAssertion,
  MintAccountAssertionArgs,
  StakeAccountAssertion,
  StakeAccountAssertionArgs,
  TokenAccountAssertion,
  TokenAccountAssertionArgs,
  UpgradeableLoaderStateAssertion,
  UpgradeableLoaderStateAssertionArgs,
  getAccountInfoAssertionDecoder,
  getAccountInfoAssertionEncoder,
  getDataValueAssertionDecoder,
  getDataValueAssertionEncoder,
  getMintAccountAssertionDecoder,
  getMintAccountAssertionEncoder,
  getStakeAccountAssertionDecoder,
  getStakeAccountAssertionEncoder,
  getTokenAccountAssertionDecoder,
  getTokenAccountAssertionEncoder,
  getUpgradeableLoaderStateAssertionDecoder,
  getUpgradeableLoaderStateAssertionEncoder,
} from './generated';

export type CompactU64 = number;
export type CompactU64Args = number;

export function getCompactU64Encoder(): VariableSizeEncoder<bigint | number> {
  return createEncoder({
    getSizeFromValue: (value: bigint | number): number => {
      if (typeof value === 'bigint' && value > Number.MAX_SAFE_INTEGER) {
        throw new Error('CompactU64 value is too large');
      }

      return LEB128.encode(Number(value)).length;
    },
    write: (
      value: bigint | number,
      bytes: Uint8Array,
      offset: Offset
    ): Offset => {
      if (typeof value === 'bigint' && value > Number.MAX_SAFE_INTEGER) {
        throw new Error('CompactU64 value is too large');
      }

      const encodedNumber = UnsignedLEB128.encode(Number(value));
      bytes.set(encodedNumber, offset);

      return offset + encodedNumber.length;
    },
  });
}

export const getCompactU64Decoder = (): VariableSizeDecoder<number> =>
  createDecoder({
    read: (
      bytes: ReadonlyUint8Array | Uint8Array,
      offset
    ): [number, Offset] => {
      const value = UnsignedLEB128.decode(bytes as Uint8Array, offset);

      return [value, offset + UnsignedLEB128.encode(value).length];
    },
  });

export type AccountInfoAssertions = Array<AccountInfoAssertion>;
export type AccountInfoAssertionsArgs = Array<AccountInfoAssertionArgs>;

export function getAccountInfoAssertionsEncoder() {
  return getArrayEncoder(getAccountInfoAssertionEncoder(), {
    size: getCompactU64Encoder(),
  });
}

export function getAccountInfoAssertionsDecoder() {
  return getArrayDecoder(getAccountInfoAssertionDecoder(), {
    size: getCompactU64Decoder(),
  });
}

export type MintAccountAssertions = Array<MintAccountAssertion>;
export type MintAccountAssertionsArgs = Array<MintAccountAssertionArgs>;

export function getMintAccountAssertionsEncoder() {
  return getArrayEncoder(getMintAccountAssertionEncoder(), {
    size: getCompactU64Encoder(),
  });
}

export function getMintAccountAssertionsDecoder() {
  return getArrayDecoder(getMintAccountAssertionDecoder(), {
    size: getCompactU64Decoder(),
  });
}

export type StakeAccountAssertions = Array<StakeAccountAssertion>;
export type StakeAccountAssertionsArgs = Array<StakeAccountAssertionArgs>;

export function getStakeAccountAssertionsEncoder() {
  return getArrayEncoder(getStakeAccountAssertionEncoder(), {
    size: getCompactU64Encoder(),
  });
}

export function getStakeAccountAssertionsDecoder() {
  return getArrayDecoder(getStakeAccountAssertionDecoder(), {
    size: getCompactU64Decoder(),
  });
}

export type TokenAccountAssertions = Array<TokenAccountAssertion>;
export type TokenAccountAssertionsArgs = Array<TokenAccountAssertionArgs>;

export function getTokenAccountAssertionsEncoder() {
  return getArrayEncoder(getTokenAccountAssertionEncoder(), {
    size: getCompactU64Encoder(),
  });
}

export function getTokenAccountAssertionsDecoder() {
  return getArrayDecoder(getTokenAccountAssertionDecoder(), {
    size: getCompactU64Decoder(),
  });
}

export type UpgradeableLoaderStateAssertions =
  Array<UpgradeableLoaderStateAssertion>;
export type UpgradeableLoaderStateAssertionsArgs =
  Array<UpgradeableLoaderStateAssertionArgs>;

export function getUpgradeableLoaderStateAssertionsEncoder() {
  return getArrayEncoder(getUpgradeableLoaderStateAssertionEncoder(), {
    size: getCompactU64Encoder(),
  });
}

export function getUpgradeableLoaderStateAssertionsDecoder() {
  return getArrayDecoder(getUpgradeableLoaderStateAssertionDecoder(), {
    size: getCompactU64Decoder(),
  });
}

export type AccountDataAssertion = {
  offset: CompactU64;
  assertion: DataValueAssertion;
};

export type AccountDataAssertionArgs = {
  offset: CompactU64Args;
  assertion: DataValueAssertionArgs;
};

export function getAccountDataAssertionEncoder(): Encoder<AccountDataAssertionArgs> {
  return transformEncoder(
    getStructEncoder([
      ['offset', getCompactU64Encoder()],
      ['assertion', getDataValueAssertionEncoder()],
    ]),
    (value) => value
  );
}

export function getAccountDataAssertionDecoder(): Decoder<AccountDataAssertion> {
  return getStructDecoder([
    ['offset', getCompactU64Decoder()],
    ['assertion', getDataValueAssertionDecoder()],
  ]);
}

export type AccountDataAssertions = Array<AccountDataAssertion>;
export type AccountDataAssertionsArgs = Array<AccountDataAssertionArgs>;

export function getAccountDataAssertionsEncoder() {
  return getArrayEncoder(getAccountDataAssertionEncoder(), {
    size: getCompactU64Encoder(),
  });
}

export function getAccountDataAssertionsDecoder() {
  return getArrayDecoder(getAccountDataAssertionDecoder(), {
    size: getCompactU64Decoder(),
  });
}

export type CompactBytes = number[];
export type CompactBytesArgs = number[];

export function getCompactBytesEncoder() {
  return getArrayEncoder(getU8Encoder(), {
    size: getCompactU64Encoder(),
  });
}

export function getCompactBytesDecoder() {
  return getArrayDecoder(getU8Decoder(), {
    size: getCompactU64Decoder(),
  });
}
