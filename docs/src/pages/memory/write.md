---
title: Memory Write
metaTitle: Lighthouse - Memory Write
description: MemoryWrite Instruction.
---

## Summary

The **MemoryWrite** instruction lets you write different types of data to a memory account. A memory account is a user owned account that can store arbitrary amounts of data and is meant to be used to assert on instruction-level delta changes in onchain state.

There are several instruction arguments that you can use to configure the **MemoryWrite** instruction:

- **source_account**: The account whose data you want to write to memory (Ignored for DataValue and Clock write types).
- **write_offset**: The offset in the source account data where you want to write. The account will resize (realloc) to whatever the offset + data length is calculated to be, so multiple write instructions can be combined without worrying about the length of the account.

- **write_type**: The type of data you want to write to memory. The data can be account data, account info fields, data values, or clock fields.

- **memory_id**: The memory account ID. One user can have 256 memory accounts, or even more if you find and utilize different off-curve bumps per id.

```rust
pub enum WriteType {
    AccountData { offset: u16, data_length: u16 },
    AccountInfoField(AccountInfoField),
    DataValue(DataValue),
    Clock(ClockField),
}
```

You can write data to a memory account in the following ways:

- **AccountData**: Write the target account data to a memory account.
- **AccountInfoField**: Write the target account info field to a memory account.
- **DataValue**: Write a data value to a memory account.
- **Clock**: Write sysvar clock fields to a memory account.

### Example: Writing token account data to a memory account using AccountData write type

The following instruction writes the first 72 bytes of a token account to memory. The first 72 bytes being the **Mint** (Pubkey), **Owner** (Pubkey), and **Amount** (u64).

{% dialect-switcher title="Writing token account data to memory" %}
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const [memory, memoryBump] = await findMemoryPda({
  memoryId: 0,
  payer: userPubkey,
})

const ix = getMemoryWriteInstruction({
  payer: userPubkey,
  sourceAccount: tokenAccountKey,
  programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  memory,
  memoryId: 0,
  memoryBump: memoryBump,
  writeOffset: 0,
  writeType: {
    __kind: 'AccountData',
    offset: 0,
    dataLength: 72,
  },
})
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const [memory, memoryBump] = findMemoryPda({
  payer: publicKey(userPubkey),
  memoryId: 0,
})

const tx = memoryWrite(umi, {
  memory: publicKey(memory),
  sourceAccount: publicKey(tokenAccountKey),
  payer: publicKey(userPubkey),
  writeType: {
    __kind: 'AccountData',
    offset: 0,
    dataLength: 72,
  },
  memoryId: 0,
  writeOffset: 0,
  memoryBump,
}).build(umi)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
MemoryWriteBuilder::new()
  .payer(user.encodable_pubkey())
  .source_account(token_account_key)
  .program_id(lighthouse_sdk::ID)
  .memory(memory_key)
  .memory_id(0)
  .memory_bump(memory_bump)
  .write_offset(0)
  .write_type(WriteType::AccountData {
      offset: 0,
      data_length: 72,
  })
  .instruction()
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

Writing these fields to memory means we can now assert on the delta changes between the token account and the memory account.

{% dialect-switcher title="Asserting on stored memory" %}
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const [memory, memoryBump] = await findMemoryPda({
  memoryId: 0,
  payer: userPubkey,
})

const ixs = [
  getAssertAccountDeltaInstruction({
    accountA: memory,
    accountB: tokenAccountKey,
    assertion: accountDeltaAssertion('Data', {
      aOffset: 0,
      bOffset: 0,
      assertion: dataValueDeltaAssertion('Bytes', {
        operator: ByteSliceOperator.Equal,
        length: 64,
      }),
    }),
  }),
  getAssertAccountDeltaInstruction({
    accountA: memory,
    accountB: tokenAccountKey,
    assertion: accountDeltaAssertion('Data', {
      aOffset: 64,
      bOffset: 64,
      assertion: dataValueDeltaAssertion('U64', {
        value: -50,
        operator: IntegerOperator.GreaterThan,
      }),
    }),
  }),
]
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const ixs = assertAccountDelta(umi, {
  accountA: publicKey(memory),
  accountB: publicKey(tokenAccountKey),
  assertion: {
    __kind: 'Data',
    aOffset: 0,
    bOffset: 0,
    assertion: {
      __kind: 'Bytes',
      length: 64,
      operator: ByteSliceOperator.Equal,
    },
  },
})
  .append(
    assertAccountDelta(umi, {
      accountA: publicKey(memory),
      accountB: publicKey(tokenAccountKey),
      assertion: {
        __kind: 'Data',
        aOffset: 64,
        bOffset: 64,
        assertion: {
          __kind: 'U64',
          value: -50,
          operator: IntegerOperator.GreaterThan,
        },
      },
    })
  )
  .getInstructions()
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let ixs = vec![
    AssertAccountDeltaBuilder::new()
        .account_a(memory_key)
        .account_b(token_account_key)
        .assertion(AccountDeltaAssertion::Data {
            a_offset: 0,
            b_offset: 0,
            assertion: DataValueDeltaAssertion::Bytes {
                operator: ByteSliceOperator::Equal,
                length: 64,
            },
        })
        .instruction(),
    AssertAccountDeltaBuilder::new()
        .account_a(memory_key)
        .account_b(token_account_key)
        .assertion(AccountDeltaAssertion::Data {
            a_offset: 64,
            b_offset: 64,
            assertion: DataValueDeltaAssertion::U64 {
                value: -50,
                operator: IntegerOperator::GreaterThan,
            },
        })
        .instruction(),
];
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

The following delta assertions are checking that nothing has changed in the first 64 bytes of the token account since writing to memory and that the amount (byte offset 64 to 72) has only decreased by 50.

Lastly, you can close the memory account to free up rent.

{% dialect-switcher title="Closing memory account" %}
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const [memory, memoryBump] = await findMemoryPda({
  memoryId: 0,
  payer: userPubkey,
})

const ix = getMemoryCloseInstruction({
  payer: userPubkey,
  programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  memory,
  memoryBump,
  memoryId: 0,
})
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const [memory, memoryBump] = findMemoryPda({
  payer: publicKey(userPubkey),
  memoryId: 0,
})

const ix = memoryClose(umi, {
  memory: publicKey(memory),
  memoryBump,
  memoryId: 0,
  payer: publicKey(userPubkey),
}).getInstructions()[0]
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
    MemoryCloseBuilder::new()
        .payer(user.encodable_pubkey())
        .program_id(lighthouse_sdk::ID)
        .memory(memory)
        .memory_bump(memory_bump)
        .memory_id(0)
        .instruction(),
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Example: Writing account info to a memory account using AccountInfoField write type

The following example writes the account info fields of a token account to memory. The account info fields are **DataLength**, **Executable**, **Owner**, **Lamports**, **RentEpoch**, and **Key**.

{% dialect-switcher title="Writing account info to memory" %}
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const [memory, memoryBump] = await findMemoryPda({
  memoryId: 0,
  payer: userPubkey,
})

const builderFn = (writeType: WriteType, offset: number) => {
  return getMemoryWriteInstruction({
    payer: userPubkey,
    sourceAccount: testAccountKey,
    programId: LIGHTHOUSE_PROGRAM_ADDRESS,
    memory,
    memoryId: 0,
    memoryBump,
    writeOffset: offset,
    writeType,
  })
}

const tx = await pipe(
  createTransaction({ version: 0 }),
  (tx) =>
    appendTransactionInstructions(
      [
        builderFn(
          {
            __kind: 'AccountInfoField',
            fields: [AccountInfoField.DataLength],
          },
          0
        ),
        builderFn(
          {
            __kind: 'AccountInfoField',
            fields: [AccountInfoField.Executable],
          },
          8
        ),
        builderFn(
          {
            __kind: 'AccountInfoField',
            fields: [AccountInfoField.Owner],
          },
          16
        ),
        builderFn(
          {
            __kind: 'AccountInfoField',
            fields: [AccountInfoField.Lamports],
          },
          48
        ),
        builderFn(
          {
            __kind: 'AccountInfoField',
            fields: [AccountInfoField.RentEpoch],
          },
          56
        ),
        builderFn(
          {
            __kind: 'AccountInfoField',
            fields: [AccountInfoField.Key],
          },
          64
        ),
      ],
      tx
    ),
  (tx) => setTransactionFeePayer(userPubkey, tx),
  (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  (tx) => signTransaction([user], tx)
)
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const [memory, memoryBump] = findMemoryPda({
  payer: publicKey(userPubkey),
  memoryId: 0,
})

const builderFn = (writeType: WriteType, offset: number) => {
  return memoryWrite(umi, {
    payer: publicKey(userPubkey),
    sourceAccount: publicKey(testAccountKey),
    programId: LIGHTHOUSE_PROGRAM_ID,
    memory: publicKey(memory),
    memoryId: 0,
    memoryBump,
    writeOffset: offset,
    writeType,
  })
}

const tx = builderFn(
  {
    __kind: 'AccountInfoField',
    fields: [AccountInfoField.DataLength],
  },
  0
)
  .append(
    builderFn(
      {
        __kind: 'AccountInfoField',
        fields: [AccountInfoField.Executable],
      },
      8
    )
  )
  .append(
    builderFn(
      {
        __kind: 'AccountInfoField',
        fields: [AccountInfoField.Owner],
      },
      16
    )
  )
  .append(
    builderFn(
      {
        __kind: 'AccountInfoField',
        fields: [AccountInfoField.Lamports],
      },
      48
    )
  )
  .append(
    builderFn(
      {
        __kind: 'AccountInfoField',
        fields: [AccountInfoField.RentEpoch],
      },
      56
    )
  )
  .append(
    builderFn(
      {
        __kind: 'AccountInfoField',
        fields: [AccountInfoField.Key],
      },
      64
    )
  )
  .build(umi)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let builder_fn = |write_type: WriteType, offset: u16| {
    MemoryWriteBuilder::new()
        .payer(user_key)
        .source_account(test_account_key)
        .program_id(lighthouse_sdk::ID)
        .memory(memory)
        .memory_id(0)
        .memory_bump(memory_bump)
        .write_offset(offset)
        .system_program(system_program::id())
        .write_type(write_type)
        .instruction()
};

let tx = Transaction::new_signed_with_payer(
    &[
        builder_fn(WriteType::AccountInfoField(AccountInfoField::DataLength), 0),
        builder_fn(WriteType::AccountInfoField(AccountInfoField::Executable), 8),
        builder_fn(WriteType::AccountInfoField(AccountInfoField::Owner), 16),
        builder_fn(WriteType::AccountInfoField(AccountInfoField::Lamports), 48),
        builder_fn(WriteType::AccountInfoField(AccountInfoField::RentEpoch), 56),
        builder_fn(WriteType::AccountInfoField(AccountInfoField::Key), 64),
    ],
    Some(&user.encodable_pubkey()),
    &[&user],
    context.get_blockhash().await,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Example: Writing data value to a memory account using DataValue write type

The following example writes a u128 and a pubkey to a memory account and then asserts on the written values using lighthouse assertions.

{% dialect-switcher title="Writing data value to memory" %}
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const [memory, memoryBump] = await findMemoryPda({
  memoryId: 0,
  payer: userPubkey,
})

const tx = await pipe(
  createTransaction({ version: 0 }),
  (tx) =>
    appendTransactionInstructions(
      [
        getMemoryWriteInstruction({
          payer: userPubkey,
          sourceAccount: LIGHTHOUSE_PROGRAM_ADDRESS,
          programId: LIGHTHOUSE_PROGRAM_ADDRESS,
          memory,
          memoryId: 0,
          memoryBump,
          writeOffset: 0,
          writeType: {
            __kind: 'DataValue',
            fields: [
              {
                __kind: 'U128',
                fields: [BigInt('340282366920938463463374607431768211455')],
              },
            ],
          },
        }),
        getMemoryWriteInstruction({
          payer: userPubkey,
          sourceAccount: LIGHTHOUSE_PROGRAM_ADDRESS,
          programId: LIGHTHOUSE_PROGRAM_ADDRESS,
          memory,
          memoryId: 0,
          memoryBump,
          writeOffset: 32,
          writeType: {
            __kind: 'DataValue',
            fields: [
              {
                __kind: 'Pubkey',
                fields: [someKey],
              },
            ],
          },
        }),
        getAssertAccountDataInstruction({
          targetAccount: memory,
          assertion: dataValueAssertion('U128', {
            value: BigInt('340282366920938463463374607431768211455'),
            operator: IntegerOperator.Equal,
          }),
          offset: 0,
        }),
        getAssertAccountDataInstruction({
          targetAccount: memory,
          assertion: dataValueAssertion('Pubkey', {
            value: someKey,
            operator: EquatableOperator.Equal,
          }),
          offset: 32,
        }),
      ],
      tx
    ),
  (tx) => setTransactionFeePayer(userPubkey, tx),
  (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  (tx) => signTransaction([user], tx)
)
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const [memory, memoryBump] = findMemoryPda({
  payer: publicKey(userPubkey),
  memoryId: 0,
})

const tx = memoryWrite(umi, {
  memory: publicKey(memory),
  sourceAccount: publicKey(LIGHTHOUSE_PROGRAM_ID),
  programId: LIGHTHOUSE_PROGRAM_ID,
  memoryId: 0,
  memoryBump,
  writeOffset: 0,
  writeType: {
    __kind: 'DataValue',
    fields: [
      {
        __kind: 'U128',
        fields: [BigInt('340282366920938463463374607431768211455')],
      },
    ],
  },
})
  .append(
    memoryWrite(umi, {
      memory: publicKey(memory),
      sourceAccount: publicKey(LIGHTHOUSE_PROGRAM_ID),
      programId: LIGHTHOUSE_PROGRAM_ID,
      memoryId: 0,
      memoryBump,
      writeOffset: 32,
      writeType: {
        __kind: 'DataValue',
        fields: [{ __kind: 'Pubkey', fields: [publicKey(somePubkey)] }],
      },
    })
  )
  .append(
    assertAccountData(umi, {
      targetAccount: publicKey(memory),
      offset: 0,
      assertion: {
        __kind: 'U128',
        value: BigInt('340282366920938463463374607431768211455'),
        operator: IntegerOperator.Equal,
      },
    })
  )
  .append(
    assertAccountData(umi, {
      targetAccount: publicKey(memory),
      offset: 32,
      assertion: {
        __kind: 'Pubkey',
        value: publicKey(somePubkey),
        operator: EquatableOperator.Equal,
      },
    })
  )
  .build(umi)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let tx = Transaction::new_signed_with_payer(
    &[
        MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_sdk::ID)
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_id(0)
            .memory_bump(memory_bump)
            .write_offset(0)
            .system_program(system_program::id())
            .write_type(WriteType::DataValue(DataValue::U128(u128::MAX)))
            .instruction(),
        MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_sdk::ID)
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_id(0)
            .memory_bump(memory_bump)
            .write_offset(32)
            .system_program(system_program::id())
            .write_type(WriteType::DataValue(DataValue::Pubkey(
                some_key,
            )))
            .instruction(),
        AssertAccountDataBuilder::new()
            .target_account(memory)
            .assertion(DataValueAssertion::U128 {
                value: u128::MAX,
                operator: IntegerOperator::Equal,
            })
            .offset(0)
            .instruction(),
        AssertAccountDataBuilder::new()
            .target_account(memory)
            .assertion(DataValueAssertion::Pubkey {
                value: some_key,
                operator: EquatableOperator::Equal,
            })
            .offset(32)
            .instruction(),
    ],
    Some(&user.encodable_pubkey()),
    &[&user],
    context.get_blockhash().await,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Example: Writing clock fields to a memory account using Clock write type

{% dialect-switcher title="Writing clock fields to memory" %}
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const [memory, memoryBump] = await findMemoryPda({
  memoryId: 4,
  payer: userPubkey,
})

const tx = await pipe(
  createTransaction({ version: 0 }),
  (tx) =>
    appendTransactionInstructions(
      [
        getMemoryWriteInstruction({
          memory,
          memoryId: 4, // You can write to multiple memory accounts in a single transaction
          memoryBump,
          programId: LIGHTHOUSE_PROGRAM_ADDRESS,
          payer: userPubkey,
          sourceAccount: LIGHTHOUSE_PROGRAM_ADDRESS, // This account is ignored so should be an account already in the transaction to save transaction space.
          writeOffset: 0,
          writeType: {
            __kind: 'Clock',
            fields: [ClockField.Slot],
          },
        }),
      ],
      tx
    ),
  (tx) => setTransactionFeePayer(userPubkey, tx),
  (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  (tx) => signTransaction([user], tx)
)
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const [memoryKey, memoryBump] = findMemoryPda({
  payer: publicKey(userPubkey),
  memoryId: 4,
})

const tx = memoryWrite(umi, {
  memory: publicKey(memoryKey),
  memoryId: 4, // You can write to multiple memory accounts in a single transaction
  memoryBump,
  programId: LIGHTHOUSE_PROGRAM_ID,
  payer: publicKey(userPubkey),
  sourceAccount: publicKey(LIGHTHOUSE_PROGRAM_ID), // This account is ignored so should be an account already in the transaction to save transaction space.
  writeOffset: 0,
  writeType: {
    __kind: 'Clock',
    fields: [ClockField.Slot],
  },
}).build(umi)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let tx = Transaction::new_signed_with_payer(
    &[MemoryWriteBuilder::new()
        .memory(memory_key)
        .memory_id(4)   // You can write to multiple memory accounts in a single transaction
        .memory_bump(memory_bump)
        .program_id(lighthouse_sdk::ID)
        .payer(user.encodable_pubkey())
        .source_account(lighthouse_sdk::ID) // This account is ignored so should be an account already in the transaction to save transaction space.
        .write_offset(0)
        .write_type(WriteType::Clock(ClockField::Slot))
        .instruction()],
    Some(&user.encodable_pubkey()),
    &[&user],
    ctx.get_blockhash().await,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
