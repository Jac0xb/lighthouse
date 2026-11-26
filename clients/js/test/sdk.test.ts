import assert from 'node:assert/strict';
import test from 'node:test';
import { publicKey, type Context } from '@metaplex-foundation/umi';
import { PublicKey } from '@solana/web3.js';
import {
  EquatableOperator,
  IntegerOperator,
  LIGHTHOUSE_PROGRAM_ID,
  LogLevel,
  assertAccountData,
  assertAccountInfo,
  assertSysvarClock,
  assertVoteAccount,
  createLighthouseProgram,
  findMemoryPda,
  getCompactU64Serializer,
} from '../src';

const mockContext: Pick<Context, 'programs'> = {
  programs: {
    getPublicKey: (_name, fallback) => fallback ?? LIGHTHOUSE_PROGRAM_ID,
  } as Context['programs'],
};

test('CompactU64 serializer encodes and decodes unsigned LEB128 accurately', () => {
  const serializer = getCompactU64Serializer();
  for (const value of [
    0,
    1,
    63,
    64,
    127,
    128,
    1000,
    16384,
    2 ** 31,
    2 ** 32,
    Number.MAX_SAFE_INTEGER,
  ]) {
    const encoded = serializer.serialize(value);
    const [decoded, bytesRead] = serializer.deserialize(encoded, 0);
    assert.equal(decoded, value);
    assert.equal(bytesRead, encoded.length);
  }
  assert.deepEqual(Array.from(serializer.serialize(1000)), [0xe8, 0x07]);
  assert.throws(() => serializer.serialize(-1), RangeError);
  assert.throws(() => serializer.deserialize(new Uint8Array([0x80]), 0), RangeError);
});

test('findMemoryPda and Umi builders produce valid instructions', () => {
  const payerWeb3 = new PublicKey('11111111111111111111111111111111');
  const [pda, bump] = findMemoryPda({ payer: payerWeb3, memoryId: 0 });
  assert.ok(pda instanceof PublicKey);
  assert.ok(bump >= 0 && bump <= 255);

  const program = createLighthouseProgram();
  assert.equal(program.publicKey, LIGHTHOUSE_PROGRAM_ID);

  const target = publicKey('11111111111111111111111111111111');

  const infoIxs = assertAccountInfo(mockContext, {
    targetAccount: target,
    assertion: {
      __kind: 'Lamports',
      value: 1_000_000n,
      operator: IntegerOperator.GreaterThanOrEqual,
    },
  }).getInstructions();
  assert.equal(infoIxs.length, 1);
  assert.equal(infoIxs[0].data[0], 5);

  const dataIxs = assertAccountData(mockContext, {
    targetAccount: target,
    offset: 64,
    assertion: {
      __kind: 'U64',
      value: 42n,
      operator: IntegerOperator.Equal,
    },
  }).getInstructions();
  assert.equal(dataIxs.length, 1);
  assert.equal(dataIxs[0].data[0], 2);

  const clockIxs = assertSysvarClock(mockContext, {
    logLevel: LogLevel.PlaintextMessage,
    assertion: {
      __kind: 'Slot',
      value: 100n,
      operator: IntegerOperator.GreaterThan,
    },
  }).getInstructions();
  assert.equal(clockIxs.length, 1);
  assert.equal(clockIxs[0].data[0], 15);
  assert.equal(clockIxs[0].keys.length, 0);

  const voteIxs = assertVoteAccount(mockContext, {
    targetAccount: target,
    __kind: 'AuthorizedWithdrawer',
    value: target,
    operator: EquatableOperator.Equal,
  }).getInstructions();
  assert.equal(voteIxs.length, 1);
  assert.equal(voteIxs[0].data[0], 3);
});
