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
} from '@solana/kit';
import {
  AccountInfoAssertion,
  AccountInfoAssertionArgs,
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

function decodeUnsignedLeb128(
  bytes: ReadonlyUint8Array | Uint8Array,
  offset = 0
): [bigint, number] {
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
export type CompactU64Args = number | bigint;

export function getCompactU64Encoder(): VariableSizeEncoder<bigint | number> {
  return createEncoder({
    getSizeFromValue: (value: bigint | number): number =>
      encodeUnsignedLeb128(value).length,
    write: (
      value: bigint | number,
      bytes: Uint8Array,
      offset: Offset
    ): Offset => {
      const encodedNumber = encodeUnsignedLeb128(value);
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
      const [value, nextOffset] = decodeUnsignedLeb128(bytes, offset);
      if (value > BigInt(Number.MAX_SAFE_INTEGER)) {
        throw new RangeError('CompactU64 value exceeds Number.MAX_SAFE_INTEGER');
      }

      return [Number(value), nextOffset];
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
