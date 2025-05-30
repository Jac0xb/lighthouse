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

export enum StakeStateType {
  Uninitialized,
  Initialized,
  Stake,
  RewardsPool,
}

export type StakeStateTypeArgs = StakeStateType;

export function getStakeStateTypeEncoder(): Encoder<StakeStateTypeArgs> {
  return getEnumEncoder(StakeStateType);
}

export function getStakeStateTypeDecoder(): Decoder<StakeStateType> {
  return getEnumDecoder(StakeStateType);
}

export function getStakeStateTypeCodec(): Codec<
  StakeStateTypeArgs,
  StakeStateType
> {
  return combineCodec(getStakeStateTypeEncoder(), getStakeStateTypeDecoder());
}
