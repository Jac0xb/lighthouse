const assert = require('node:assert/strict');
const { test } = require('node:test');
const { readFileSync } = require('node:fs');
const path = require('node:path');
const kit = require('@solana/kit');
const lighthouse = require('../dist/src/index.js');

const payer = kit.getAddressDecoder().decode(new Uint8Array(32).fill(1));
const target = kit.getAddressDecoder().decode(new Uint8Array(32).fill(2));
const blockhash = kit.getBase58Decoder().decode(new Uint8Array(32).fill(9));

function assertion() {
  return lighthouse.getAssertAccountInfoInstruction({
    targetAccount: target,
    assertion: {
      __kind: 'Lamports',
      value: 42n,
      operator: lighthouse.IntegerOperator.Equal,
    },
  });
}

function message(version, instructions, feePayer = payer) {
  let result = kit.pipe(
    kit.createTransactionMessage({ version }),
    (m) => kit.setTransactionMessageFeePayer(feePayer, m),
    (m) =>
      kit.setTransactionMessageLifetimeUsingBlockhash(
        { blockhash, lastValidBlockHeight: 100n },
        m
      ),
    (m) => kit.appendTransactionMessageInstructions(instructions, m)
  );
  if (version === 1) {
    result = kit.pipe(
      result,
      (m) => kit.setTransactionMessageComputeUnitLimit(200_000, m),
      (m) => kit.setTransactionMessageLoadedAccountsDataSizeLimit(65_536, m),
      (m) => kit.setTransactionMessageHeapSize(65_536, m),
      (m) => kit.setTransactionMessagePriorityFeeLamports(5_000n, m)
    );
  }
  return result;
}

for (const version of ['legacy', 0, 1]) {
  test(`Lighthouse assertions round-trip in ${version} transactions`, () => {
    const instruction = assertion();
    const original = message(version, [instruction]);
    const appended = kit.appendTransactionMessageInstruction(
      instruction,
      original
    );
    const tx = kit.compileTransaction(appended);
    const bytes = kit.getTransactionEncoder().encode(tx);
    const decoded = kit.getTransactionDecoder().decode(bytes);
    const compiled = kit
      .getCompiledTransactionMessageDecoder()
      .decode(decoded.messageBytes);
    const restored = kit.decompileTransactionMessage(compiled);

    assert.equal(restored.version, version);
    assert.equal(restored.feePayer.address, payer);
    assert.equal(restored.lifetimeConstraint.blockhash, blockhash);
    assert.equal(original.instructions.length, 1);
    assert.equal(restored.instructions.length, 2);
    for (const ix of restored.instructions) {
      assert.equal(ix.programAddress, lighthouse.LIGHTHOUSE_PROGRAM_ADDRESS);
      assert.deepEqual(ix.accounts, instruction.accounts);
      assert.deepEqual(ix.data, instruction.data);
      assert.deepEqual(
        lighthouse.parseAssertAccountInfoInstruction(ix).data.assertion,
        {
          __kind: 'Lamports',
          value: 42n,
          operator: lighthouse.IntegerOperator.Equal,
        }
      );
    }
    if (version === 1) {
      assert.deepEqual(restored.config, original.config);
      assert.equal(bytes[0], 0x81);
      assert.deepEqual(bytes.slice(-64), new Uint8Array(64));
    }
    kit.assertIsTransactionWithinSizeLimit(tx);
  });
}

test('a signed v1 transaction carries a large Lighthouse assertion and its config', async () => {
  const signer = await kit.generateKeyPairSigner();
  const instruction = lighthouse.getAssertAccountDataInstruction({
    targetAccount: target,
    offset: 0,
    assertion: {
      __kind: 'Bytes',
      value: Array(1800).fill(7),
      operator: lighthouse.EquatableOperator.Equal,
    },
  });
  const original = message(1, [instruction, assertion()], signer.address);
  const tx = await kit.signTransactionMessageWithSigners(
    kit.setTransactionMessageFeePayerSigner(signer, original)
  );
  const bytes = kit.getTransactionEncoder().encode(tx);
  assert.ok(bytes.length > 1232 && bytes.length <= 4096);
  kit.assertIsTransactionWithinSizeLimit(tx);
  assert.equal(bytes[0], 0x81);
  assert.deepEqual(bytes.slice(-64), tx.signatures[signer.address]);
  assert.deepEqual(bytes.slice(0, -64), tx.messageBytes);
  assert.ok(
    await kit.verifySignature(
      signer.keyPair.publicKey,
      tx.signatures[signer.address],
      tx.messageBytes
    )
  );
  const decoded = kit.getTransactionDecoder().decode(bytes);
  const restored = kit.decompileTransactionMessage(
    kit.getCompiledTransactionMessageDecoder().decode(decoded.messageBytes)
  );
  assert.deepEqual(restored.config, original.config);
  assert.deepEqual(
    lighthouse.parseAssertAccountDataInstruction(restored.instructions[0]).data
      .assertion.value,
    Array(1800).fill(7)
  );
  const base64 = kit.getBase64EncodedWireTransaction(tx);
  assert.deepEqual(Buffer.from(base64, 'base64'), Buffer.from(bytes));
  for (const version of ['legacy', 0]) {
    assert.throws(() =>
      kit.assertIsTransactionWithinSizeLimit(
        kit.compileTransaction(message(version, [instruction]))
      )
    );
  }
});

test('v1 enforces the complete 4096-byte size after adding an assertion', () => {
  const empty = { ...assertion(), data: new Uint8Array() };
  const overhead = kit.getTransactionSize(
    kit.compileTransaction(message(1, [empty]))
  );
  const full = { ...empty, data: new Uint8Array(4096 - overhead) };
  const original = message(1, [full]);
  const tx = kit.compileTransaction(original);
  assert.equal(kit.getTransactionSize(tx), 4096);
  kit.assertIsTransactionWithinSizeLimit(tx);
  assert.throws(() =>
    kit.assertIsTransactionWithinSizeLimit(
      kit.compileTransaction(
        kit.appendTransactionMessageInstruction(assertion(), original)
      )
    )
  );
});

test('v1 wire bytes match the shared Rust fixture', () => {
  const expected = readFileSync(
    path.join(__dirname, '../../../tests/fixtures/v1-lighthouse.base64'),
    'utf8'
  ).trim();
  const tx = kit.compileTransaction(message(1, [assertion()]));
  assert.equal(kit.getBase64EncodedWireTransaction(tx), expected);
});
