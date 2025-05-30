/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getEnumDecoder,
  getEnumEncoder,
  type Codec,
  type Decoder,
  type Encoder,
} from '@solana/kit';

export enum UpgradeableLoaderStateType {
  Uninitialized,
  Buffer,
  Program,
  ProgramData,
}

export type UpgradeableLoaderStateTypeArgs = UpgradeableLoaderStateType;

export function getUpgradeableLoaderStateTypeEncoder(): Encoder<UpgradeableLoaderStateTypeArgs> {
  return getEnumEncoder(UpgradeableLoaderStateType);
}

export function getUpgradeableLoaderStateTypeDecoder(): Decoder<UpgradeableLoaderStateType> {
  return getEnumDecoder(UpgradeableLoaderStateType);
}

export function getUpgradeableLoaderStateTypeCodec(): Codec<
  UpgradeableLoaderStateTypeArgs,
  UpgradeableLoaderStateType
> {
  return combineCodec(
    getUpgradeableLoaderStateTypeEncoder(),
    getUpgradeableLoaderStateTypeDecoder()
  );
}
