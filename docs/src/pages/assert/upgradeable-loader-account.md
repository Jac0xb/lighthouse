---
title: Assert Account Delta
metaTitle: Assert - Account Delta
description:
---

## AssertAccountDelta Instruction

The **AssertAccountDelta** instruction is similar to [AssertAccountData](/assert/account-data) and [AssertAccountInfo](/assert/account-info) instructions but with the key difference of allowing you to compare one account's data with another account's data or **AccountInfo**.

### Example:

<!--

Imagine a struct with the following layout:

```rust
pub struct TestAccount {
    pub account_discriminator: [u8; 8], // bytes 0 to 7
    pub balance: u64,                   // bytes 8 to 16
}
```

Say we wanted to assert on the _balance_ field in our TestAccount struct. If the struct uses a serialization schema that uses little-endian for integers (borsh, bytemuck, ...) we can deserialize the field and assert on that value at runtime!

{% dialect-switcher title="" %}
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

Say we wanted to assert on the "balance" field in our TestAccount struct. If the struct uses a serialization schema that uses little-endian for integers (borsh, bytemuck, ...) we can deserialize the field and assert on that value at runtime!

{% dialect-switcher title="" %}
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
{% /dialect-switcher %} -->
