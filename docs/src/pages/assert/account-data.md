---
title: Assert Account Data
metaTitle: Assert - Account Data
description:
---

## AssertAccountData Instruction

The **AssertAccountData** instruction gives you tools to make assertions about arbitrary account data by deserializing account data slices into a specified type and comparing it to an expected value.

The `assertion` argument for the instruction is the `DataValueAssertion` enum which allows you to describe the type you would like to deserialize the account data into and the expected value and operator used in the comparison.

**Supported types**: `bool`, `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `bytes`, `Pubkey`.

### Example: Asserting on integer values in account data.

Imagine a struct with the following layout:

```rust
pub struct TestAccount {
    pub account_discriminator: [u8; 8], // bytes 0 to 7
    pub balance: u64,                   // bytes 8 to 16
}
```

Say we wanted to assert on the `balance` field in our `TestAccount` struct. If the struct uses a serialization schema that uses little-endian for integers (borsh, bytemuck, ...) we can deserialize the field and assert on that value at runtime!

{% dialect-switcher title="Data value assertion instruction builder example" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
AssertAccountDataBuilder::new()
    .target_account(test_account_key)
    .assertion(DataValueAssertion::U64 {
        value: 420, // The expected value.
        operator: IntegerOperator::GreaterThanOrEqual,
    })
    .offset(8) // The account data byte offset where 'balance' is stored.
    .instruction();
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Example: Equality assertion on pubkey in account data.

Imagine we expand the struct from before and add a pubkey field.

```rust
pub struct TestAccount {
    pub account_discriminator: [u8; 8], // bytes 0 to 7
    pub balance: u64,                   // bytes 8 to 16
    pub owner: Pubkey,                  // bytes 16 to 48
}
```

Say we wanted to assert on the `owner` field in our `TestAccount` struct. We can deserialize the Pubkey and assert using the example code below.

{% dialect-switcher title="Data value assertion instruction builder example" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
AssertAccountDataBuilder::new()
    .target_account(test_account_key)
    .assertion(DataValueAssertion::Pubkey {
        value: owner_key,
        operator: EquatableOperator::Equal,
    })
    .offset(16) // The account data byte offset where 'owner' is stored.
    .instruction(),

```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
