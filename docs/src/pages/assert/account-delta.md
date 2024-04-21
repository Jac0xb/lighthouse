---
title: Assert Account Delta
metaTitle: Assert - Account Delta
description:
---

## AssertAccountDelta Instruction

The **AssertAccountDelta** instruction is similar to [AssertAccountData](/assert/account-data) and [AssertAccountInfo](/assert/account-info) instructions but with the key difference of allowing you to compare one account's data with another account's data or `AccountInfo`.

### Example: Storing lamports into a memory account and asserting on the delta.

You may want to assert on the lamport delta of an account during a transaction.
To do this we can use the [memory account](/memory) and delta assertion instructions that lighthouse offers. This involves writing the lamports of user's account into a memory account and asserting on the delta change.

{% dialect-switcher title="Memory write + delta assertion transaction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
import {
  createLighthouseProgram,
  IntegerOperator,
  assertAccountDelta,
  memoryWrite,
  findMemoryPda,
  AccountInfoField,
} from 'lighthouse-sdk-legacy'
import { toWeb3JsInstruction } from '@metaplex-foundation/umi-web3js-adapters'

const umi = createUmi('https://api.mainnet-beta.solana.com')
umi.programs.add(createLighthouseProgram())

let tx = new Transaction()

let [memory, memoryBump] = findMemoryPda({
  payer: userPubkey,
  memoryId: 0,
})

tx.add(
  toWeb3JsInstruction(
    memoryWrite(umi, {
      memory: publicKey(memory),
      sourceAccount: publicKey(userPubkey),
      writeType: {
        __kind: 'AccountInfoField',
        fields: [AccountInfoField.Lamports],
      },
      memoryId: 0,
      writeOffset: 0,
      memoryBump,
    }).getInstructions()[0]
  )
)

tx.add(
  SystemProgram.transfer({
    fromPubkey: userPubkey,
    toPubkey: destinationPubkey,
    lamports: 1e9,
  })
)

tx.add(
  toWeb3JsInstruction(
    assertAccountDelta(umi, {
      accountA: publicKey(memory),
      accountB: publicKey(userPubkey),
      assertion: {
        __kind: 'AccountInfo',
        aOffset: 0,
        assertion: {
          __kind: 'Lamports',
          value: -1e9,
          operator: IntegerOperator.Equal,
        },
      },
    }).getInstructions()[0]
  )
)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let (memory_key, bump) = find_memory_pda(user_key, 0);

let tx = Transaction::new_signed_with_payer(
    &[
        MemoryWriteBuilder::new()
            .memory(memory_key)
            .payer(user_key)
            .source_account(user_key)
            .program_id(lighthouse_sdk::ID)
            .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
            .memory_id(0)       // 0 to 255 id for whatever memory account you want to access.
            .write_offset(0)    // byte offset in the memory accountt to write to.
            .memory_bump(bump)
            .instruction(),
        system_instruction::transfer(
            &user_key,
            &destination_key,
            1e9 as u64,
        ),
        AssertAccountDeltaBuilder::new()
            .account_a(memory_key)
            .account_b(user_key)
            .assertion(AccountDeltaAssertion::AccountInfo {
                a_offset: 0,    // The byte offset in account A to deserialize into a u64 (lamport).
                assertion: AccountInfoDeltaAssertion::Lamports {
                    value: -1e9 as i128,
                    operator: IntegerOperator::Equal,
                },
            })
            .instruction(),
    ],
    Some(&user_key),
    &[&user_keypair],
    blockhash,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

The **first** instruction writes the lamports of the user's account into the memory account.
{% /linebreak %}
The **second** instruction transfers `1e9` lamports (1 SOL) from the user's account to another account.
{% /linebreak %}
The **third** instruction asserts that the lamport delta of the user's account is -1e9 (1 SOL), which is what we expect.

### Example: Asserting on delta of values stored in two different accounts.

Imagine a account struct with the following layout:

```rust
pub struct TestAccount {
    pub account_discriminator: [u8; 8], // bytes 0 to 7
    pub vault_balance: u64,             // bytes 8 to 16
}
```

Say we wanted to assert about the difference between two test accounts' vault balance.

{% dialect-switcher title="Instruction building example for account data delta assertion" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
let ixs = assertAccountDelta(umi, {
  accountA: publicKey(accountAKey),
  accountB: publicKey(accountBKey),
  assertion: {
    __kind: 'Data',
    aOffset: 8, // The byte offset in account A to deserialize into a u64 (vault_balance).
    bOffset: 8, // The byte offset in account B to deserialize into a u64 (vault_balance).
    assertion: {
      __kind: 'U64',
      value: expectedDiffBound, // b.vault_balance - a.vault_balance
      operator: IntegerOperator.LessThan,
    },
  },
})
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let ix = AssertAccountDeltaBuilder::new()
        .account_a(test_account_a)
        .account_b(test_account_b)
        .assertion(AccountDeltaAssertion::Data {
            a_offset: 8,    // The byte offset in account A to deserialize into a u64 (vault_balance).
            b_offset: 8,    // The byte offset in account B to deserialize into a u64 (vault_balance).
            assertion: DataValueDeltaAssertion::U64 {
                value: expected_diff_bound, // b.vault_balance - a.vault_balance
                operator: IntegerOperator::LessThan,
            },
        })
        .instruction()
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

This would typically be used after storing account data into memory so you can assert on the delta changes of account data.
