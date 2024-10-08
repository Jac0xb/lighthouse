/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Address,
  ProgramDerivedAddress,
  getAddressEncoder,
  getProgramDerivedAddress,
  getU8Encoder,
  getUtf8Encoder,
} from '@solana/web3.js';

export type MemorySeeds = {
  payer: Address;

  memoryId: number;
};

export async function findMemoryPda(
  seeds: MemorySeeds,
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const {
    programAddress = 'L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95' as Address<'L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95'>,
  } = config;
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      getUtf8Encoder().encode('memory'),
      getAddressEncoder().encode(seeds.payer),
      getU8Encoder().encode(seeds.memoryId),
    ],
  });
}
