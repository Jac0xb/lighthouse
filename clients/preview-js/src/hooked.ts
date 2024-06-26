import {
  Offset,
  ReadonlyUint8Array,
  VariableSizeDecoder,
  VariableSizeEncoder,
  createDecoder,
  createEncoder,
  getArrayDecoder,
  getArrayEncoder,
  getU8Decoder,
  getU8Encoder,
} from '@solana/web3.js';
import { LEB128, UnsignedLEB128 } from '@minhducsun2002/leb128';
import {
  AccountInfoAssertion,
  MintAccountAssertion,
  StakeAccountAssertion,
  TokenAccountAssertion,
  UpgradeableLoaderStateAssertion,
  getAccountInfoAssertionDecoder,
  getAccountInfoAssertionEncoder,
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

export function getCompactU64Encoder(): VariableSizeEncoder<number> {
  return createEncoder({
    getSizeFromValue: (value: number): number => {
      return LEB128.encode(value).length;
    },
    write: (value: number, bytes: Uint8Array, offset: Offset): Offset => {
      const encodedNumber = UnsignedLEB128.encode(value);
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
export type AccountInfoAssertionsArgs = Array<AccountInfoAssertion>;

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
export type MintAccountAssertionsArgs = Array<MintAccountAssertion>;

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
export type StakeAccountAssertionsArgs = Array<StakeAccountAssertion>;

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
export type TokenAccountAssertionsArgs = Array<TokenAccountAssertion>;

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
  Array<UpgradeableLoaderStateAssertion>;

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
