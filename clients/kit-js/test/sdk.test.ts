import assert from 'node:assert/strict';
import test from 'node:test';
import { address, createNoopSigner } from '@solana/kit';
import {
  ASSERT_ACCOUNT_DATA_DISCRIMINATOR,
  ASSERT_ACCOUNT_DATA_MULTI_DISCRIMINATOR,
  ASSERT_ACCOUNT_INFO_DISCRIMINATOR,
  ASSERT_SYSVAR_CLOCK_DISCRIMINATOR,
  EquatableOperator,
  IntegerOperator,
  LIGHTHOUSE_PROGRAM_ADDRESS,
  LighthouseInstruction,
  LogLevel,
  MEMORY_WRITE_DISCRIMINATOR,
  findMemoryPda,
  getAssertAccountDataInstruction,
  getAssertAccountDataMultiInstruction,
  getAssertAccountInfoInstruction,
  getAssertSysvarClockInstruction,
  getCompactU64Decoder,
  getCompactU64Encoder,
  getMemoryWriteInstruction,
  identifyLighthouseInstruction,
  parseLighthouseInstruction,
} from '../src';

const TEST_ADDRESS = address('11111111111111111111111111111111');

test('CompactU64 encodes and decodes unsigned LEB128 values accurately', () => {
  const encoder = getCompactU64Encoder();
  const decoder = getCompactU64Decoder();

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
    const encoded = encoder.encode(value);
    const [decoded, bytesRead] = decoder.read(encoded, 0);
    assert.equal(decoded, value);
    assert.equal(bytesRead, encoded.length);
  }

  // Values 64..127 must be 1 byte in unsigned LEB128.
  assert.equal(encoder.encode(64).length, 1);
  assert.equal(encoder.encode(127).length, 1);
  // 1000 = 0b11101000, 0b00000111
  assert.deepEqual(Array.from(encoder.encode(1000)), [0xe8, 0x07]);
  // u64::MAX matches lighthouse-common Rust encoding
  assert.deepEqual(Array.from(encoder.encode(0xffff_ffff_ffff_ffffn)), [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
  ]);
  // Reject negative, out-of-range, truncated, and overflowing inputs
  assert.throws(() => encoder.encode(-1), RangeError);
  assert.throws(() => encoder.encode(0x1_0000_0000_0000_0000n), RangeError);
  assert.throws(() => decoder.read(new Uint8Array([0x80]), 0), RangeError);
  assert.throws(
    () =>
      decoder.read(
        new Uint8Array([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02]),
        0
      ),
    RangeError
  );
});

test('findMemoryPda derives a deterministic PDA', async () => {
  const [pda, bump] = await findMemoryPda({
    payer: TEST_ADDRESS,
    memoryId: 0,
  });
  assert.ok(typeof pda === 'string' && pda.length > 0);
  assert.ok(bump >= 0 && bump <= 255);
});

test('instruction builders encode and round-trip parse cleanly', async () => {
  const infoIx = getAssertAccountInfoInstruction({
    targetAccount: TEST_ADDRESS,
    assertion: {
      __kind: 'Lamports',
      value: 1_000_000n,
      operator: IntegerOperator.GreaterThanOrEqual,
    },
  });
  assert.equal(infoIx.programAddress, LIGHTHOUSE_PROGRAM_ADDRESS);
  assert.equal(infoIx.data[0], ASSERT_ACCOUNT_INFO_DISCRIMINATOR);
  assert.equal(
    identifyLighthouseInstruction(infoIx),
    LighthouseInstruction.AssertAccountInfo
  );
  const parsedInfo = parseLighthouseInstruction(infoIx);
  assert.equal(parsedInfo.instructionType, LighthouseInstruction.AssertAccountInfo);
  assert.equal(parsedInfo.data.logLevel, LogLevel.Silent);

  const dataIx = getAssertAccountDataInstruction({
    targetAccount: TEST_ADDRESS,
    offset: 64,
    assertion: {
      __kind: 'U64',
      value: 42n,
      operator: IntegerOperator.Equal,
    },
  });
  assert.equal(dataIx.data[0], ASSERT_ACCOUNT_DATA_DISCRIMINATOR);
  const parsedData = parseLighthouseInstruction(dataIx);
  assert.equal(parsedData.instructionType, LighthouseInstruction.AssertAccountData);
  assert.equal(parsedData.data.offset, 64);

  const multiIx = getAssertAccountDataMultiInstruction({
    targetAccount: TEST_ADDRESS,
    assertions: [
      {
        offset: 0,
        assertion: {
          __kind: 'U8',
          value: 2,
          operator: IntegerOperator.Equal,
        },
      },
      {
        offset: 36,
        assertion: {
          __kind: 'Pubkey',
          value: TEST_ADDRESS,
          operator: EquatableOperator.Equal,
        },
      },
    ],
  });
  assert.equal(multiIx.data[0], ASSERT_ACCOUNT_DATA_MULTI_DISCRIMINATOR);
  const parsedMulti = parseLighthouseInstruction(multiIx);
  assert.equal(
    parsedMulti.instructionType,
    LighthouseInstruction.AssertAccountDataMulti
  );
  assert.equal(parsedMulti.data.assertions.length, 2);

  const clockIx = getAssertSysvarClockInstruction({
    logLevel: LogLevel.PlaintextMessage,
    assertion: {
      __kind: 'Slot',
      value: 100n,
      operator: IntegerOperator.GreaterThan,
    },
  });
  assert.equal(clockIx.data[0], ASSERT_SYSVAR_CLOCK_DISCRIMINATOR);
  assert.equal(clockIx.accounts, undefined);

  const [memory, memoryBump] = await findMemoryPda({
    payer: TEST_ADDRESS,
    memoryId: 0,
  });
  const writeIx = getMemoryWriteInstruction({
    programId: LIGHTHOUSE_PROGRAM_ADDRESS,
    payer: createNoopSigner(TEST_ADDRESS),
    memory,
    sourceAccount: TEST_ADDRESS,
    memoryBump,
    writeOffset: 0,
    writeType: {
      __kind: 'AccountData',
      offset: 0,
      dataLength: 8,
    },
  });
  assert.equal(writeIx.data[0], MEMORY_WRITE_DISCRIMINATOR);
  assert.equal(writeIx.accounts.length, 5);
});
