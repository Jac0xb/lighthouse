import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {
  type Address,
  type Blockhash,
  type Instruction,
  appendTransactionMessageInstruction,
  appendTransactionMessageInstructions,
  assertIsTransactionWithinSizeLimit,
  compileTransaction,
  createTransactionMessage,
  decompileTransactionMessage,
  generateKeyPairSigner,
  getAddressDecoder,
  getBase58Decoder,
  getBase64EncodedWireTransaction,
  getCompiledTransactionMessageDecoder,
  getTransactionDecoder,
  getTransactionEncoder,
  getTransactionSize,
  pipe,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayer,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageHeapSize,
  setTransactionMessageLifetimeUsingBlockhash,
  setTransactionMessageLoadedAccountsDataSizeLimit,
  setTransactionMessagePriorityFeeLamports,
  signTransactionMessageWithSigners,
} from '@solana/kit';
import {
  EquatableOperator,
  IntegerOperator,
  LIGHTHOUSE_PROGRAM_ADDRESS,
  getAssertAccountDataInstruction,
  getAssertAccountInfoInstruction,
  parseAssertAccountDataInstruction,
  parseAssertAccountInfoInstruction,
} from '../src';

const payer = getAddressDecoder().decode(new Uint8Array(32).fill(1));
const target = getAddressDecoder().decode(new Uint8Array(32).fill(2));
const blockhash = getBase58Decoder().decode(
  new Uint8Array(32).fill(9)
) as Blockhash;

function assertion() {
  return getAssertAccountInfoInstruction({
    targetAccount: target,
    assertion: {
      __kind: 'Lamports',
      value: 42n,
      operator: IntegerOperator.Equal,
    },
  });
}

function buildMessage(
  version: 'legacy' | 0 | 1,
  instructions: Instruction[],
  feePayer: Address = payer
) {
  if (version === 1) {
    return pipe(
      createTransactionMessage({ version: 1 }),
      (m) => setTransactionMessageFeePayer(feePayer, m),
      (m) =>
        setTransactionMessageLifetimeUsingBlockhash(
          { blockhash, lastValidBlockHeight: 100n },
          m
        ),
      (m) => appendTransactionMessageInstructions(instructions, m),
      (m) => setTransactionMessageComputeUnitLimit(200_000, m),
      (m) => setTransactionMessageLoadedAccountsDataSizeLimit(65_536, m),
      (m) => setTransactionMessageHeapSize(65_536, m),
      (m) => setTransactionMessagePriorityFeeLamports(5_000n, m)
    );
  }

  if (version === 0) {
    return pipe(
      createTransactionMessage({ version: 0 }),
      (m) => setTransactionMessageFeePayer(feePayer, m),
      (m) =>
        setTransactionMessageLifetimeUsingBlockhash(
          { blockhash, lastValidBlockHeight: 100n },
          m
        ),
      (m) => appendTransactionMessageInstructions(instructions, m)
    );
  }

  return pipe(
    createTransactionMessage({ version: 'legacy' }),
    (m) => setTransactionMessageFeePayer(feePayer, m),
    (m) =>
      setTransactionMessageLifetimeUsingBlockhash(
        { blockhash, lastValidBlockHeight: 100n },
        m
      ),
    (m) => appendTransactionMessageInstructions(instructions, m)
  );
}

for (const version of ['legacy', 0, 1] as const) {
  test(`Lighthouse assertions round-trip in ${version} transactions`, () => {
    const instruction = assertion();
    const original = buildMessage(version, [instruction]);
    const appended = appendTransactionMessageInstruction(instruction, original);
    const tx = compileTransaction(appended);
    const bytes = getTransactionEncoder().encode(tx);
    const decoded = getTransactionDecoder().decode(bytes);
    const compiled = getCompiledTransactionMessageDecoder().decode(
      decoded.messageBytes
    );
    const restored = decompileTransactionMessage(compiled);

    assert.equal(restored.version, version);
    assert.equal(restored.feePayer.address, payer);
    assert.ok('blockhash' in restored.lifetimeConstraint);
    assert.equal(restored.lifetimeConstraint.blockhash, blockhash);
    assert.equal(original.instructions.length, 1);
    assert.equal(restored.instructions.length, 2);

    for (const ix of restored.instructions) {
      assert.equal(ix.programAddress, LIGHTHOUSE_PROGRAM_ADDRESS);
      assert.deepEqual(ix.accounts, instruction.accounts);
      assert.deepEqual(ix.data, instruction.data);
      assert.deepEqual(
        parseAssertAccountInfoInstruction(ix as never).data.assertion,
        {
          __kind: 'Lamports',
          value: 42n,
          operator: IntegerOperator.Equal,
        }
      );
    }

    if (version === 1 && 'config' in restored && 'config' in original) {
      assert.deepEqual(restored.config, original.config);
      assert.equal(bytes[0], 0x81);
      assert.deepEqual(bytes.slice(-64), new Uint8Array(64));
    }

    assertIsTransactionWithinSizeLimit(tx);
  });
}

test('signed v1 transaction carries a large Lighthouse assertion and its config', async () => {
  const signer = await generateKeyPairSigner();
  const largeAssertion = getAssertAccountDataInstruction({
    targetAccount: target,
    offset: 0,
    assertion: {
      __kind: 'Bytes',
      value: Array.from({ length: 1800 }, () => 7),
      operator: EquatableOperator.Equal,
    },
  });

  const v1Message = pipe(
    createTransactionMessage({ version: 1 }),
    (m) => setTransactionMessageFeePayerSigner(signer, m),
    (m) =>
      setTransactionMessageLifetimeUsingBlockhash(
        { blockhash, lastValidBlockHeight: 100n },
        m
      ),
    (m) => setTransactionMessageComputeUnitLimit(200_000, m),
    (m) => setTransactionMessageLoadedAccountsDataSizeLimit(65_536, m),
    (m) => setTransactionMessageHeapSize(65_536, m),
    (m) => setTransactionMessagePriorityFeeLamports(5_000n, m),
    (m) => appendTransactionMessageInstruction(largeAssertion, m)
  );

  const tx = await signTransactionMessageWithSigners(v1Message);
  const bytes = getTransactionEncoder().encode(tx);
  assert.ok(bytes.length > 1232 && bytes.length <= 4096);
  assert.equal(bytes[0], 0x81);
  assertIsTransactionWithinSizeLimit(tx);

  const decoded = getTransactionDecoder().decode(bytes);
  const compiled = getCompiledTransactionMessageDecoder().decode(
    decoded.messageBytes
  );
  const restored = decompileTransactionMessage(compiled);
  assert.equal(restored.version, 1);
  const parsed = parseAssertAccountDataInstruction(
    restored.instructions[0] as never
  );
  assert.equal(parsed.data.assertion.__kind, 'Bytes');
  assert.deepEqual(
    parsed.data.assertion.value,
    Array.from({ length: 1800 }, () => 7)
  );

  const base64 = getBase64EncodedWireTransaction(tx);
  assert.deepEqual(Buffer.from(base64, 'base64'), Buffer.from(bytes));

  for (const version of ['legacy', 0] as const) {
    assert.throws(() =>
      assertIsTransactionWithinSizeLimit(
        compileTransaction(buildMessage(version, [largeAssertion]))
      )
    );
  }
});

test('v1 enforces the 4096-byte size limit after appending an assertion', () => {
  const empty = { ...assertion(), data: new Uint8Array() };
  const overhead = getTransactionSize(
    compileTransaction(buildMessage(1, [empty]))
  );
  const full = { ...empty, data: new Uint8Array(4096 - overhead) };
  const original = buildMessage(1, [full]);
  const tx = compileTransaction(original);
  assert.equal(getTransactionSize(tx), 4096);
  assertIsTransactionWithinSizeLimit(tx);

  assert.throws(() =>
    assertIsTransactionWithinSizeLimit(
      compileTransaction(
        appendTransactionMessageInstruction(assertion(), original)
      )
    )
  );
});

test('v1 wire bytes match the shared Rust fixture', () => {
  const expected = readFileSync(
    path.join(__dirname, '../../../tests/fixtures/v1-lighthouse.base64'),
    'utf8'
  ).trim();
  const tx = compileTransaction(buildMessage(1, [assertion()]));
  assert.equal(getBase64EncodedWireTransaction(tx), expected);
});
