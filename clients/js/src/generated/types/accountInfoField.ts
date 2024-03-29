/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getScalarEnumDecoder,
  getScalarEnumEncoder,
} from '@solana/codecs';

export enum AccountInfoField {
  Key,
  Lamports,
  DataLength,
  Owner,
  RentEpoch,
  Executable,
}

export type AccountInfoFieldArgs = AccountInfoField;

export function getAccountInfoFieldEncoder(): Encoder<AccountInfoFieldArgs> {
  return getScalarEnumEncoder(AccountInfoField);
}

export function getAccountInfoFieldDecoder(): Decoder<AccountInfoField> {
  return getScalarEnumDecoder(AccountInfoField);
}

export function getAccountInfoFieldCodec(): Codec<
  AccountInfoFieldArgs,
  AccountInfoField
> {
  return combineCodec(
    getAccountInfoFieldEncoder(),
    getAccountInfoFieldDecoder()
  );
}
