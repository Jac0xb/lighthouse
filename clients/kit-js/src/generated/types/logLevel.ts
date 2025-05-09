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

export enum LogLevel {
  Silent,
  PlaintextMessage,
  EncodedMessage,
  EncodedNoop,
  FailedPlaintextMessage,
  FailedEncodedMessage,
  FailedEncodedNoop,
}

export type LogLevelArgs = LogLevel;

export function getLogLevelEncoder(): Encoder<LogLevelArgs> {
  return getEnumEncoder(LogLevel);
}

export function getLogLevelDecoder(): Decoder<LogLevel> {
  return getEnumDecoder(LogLevel);
}

export function getLogLevelCodec(): Codec<LogLevelArgs, LogLevel> {
  return combineCodec(getLogLevelEncoder(), getLogLevelDecoder());
}
