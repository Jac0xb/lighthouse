/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum KnownProgram {
  System,
  Token,
  Token2022,
  Rent,
  Stake,
  Vote,
  BpfLoader,
  UpgradeableLoader,
  SysvarConfig,
}

export type KnownProgramArgs = KnownProgram;

export function getKnownProgramSerializer(): Serializer<
  KnownProgramArgs,
  KnownProgram
> {
  return scalarEnum<KnownProgram>(KnownProgram, {
    description: 'KnownProgram',
  }) as Serializer<KnownProgramArgs, KnownProgram>;
}
