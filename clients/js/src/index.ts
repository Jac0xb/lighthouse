export * from './generated';

import { PublicKey } from '@solana/web3.js';
import { LIGHTHOUSE_PROGRAM_ID } from './generated';

export type MemorySeeds = {
  payer: PublicKey;
  memoryId: number;
};

export function findMemoryPda(
  seeds: MemorySeeds,
  config: { programAddress?: PublicKey | undefined } = {}
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from('memory'),
      seeds.payer.toBuffer(),
      Buffer.from([seeds.memoryId]),
    ],
    config.programAddress ?? new PublicKey(LIGHTHOUSE_PROGRAM_ID)
  );
}
