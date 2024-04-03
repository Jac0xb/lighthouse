---
title: Memory Close
metaTitle: Lighthouse - Memory Close
description: MemoryClose Instruction.
---

## Summary

The **MemoryClose** instruction

{% dialect-switcher title="" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
MemoryWriteBuilder::new()
  .payer(user.encodable_pubkey())
  .source_account(token_account)
  .program_id(lighthouse_sdk::ID)
  .memory(memory)
  .memory_id(0)
  .write_offset(0)
  .memory_bump(memory_bump)
  .write_type(WriteType::AccountData {
      offset: 0,
      data_length: 72,
  })
  .instruction()
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Example: Writing token account data to memory and asserting on

{% dialect-switcher title="" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let tx = Transaction::new_signed_with_payer(
  &[
    MemoryWriteBuilder::new()
        .payer(user.encodable_pubkey())
        .source_account(token_account)
        .program_id(lighthouse_sdk::ID)
        .memory(memory)
        .memory_id(0)
        .write_offset(0)
        .memory_bump(memory_bump)
        .write_type(WriteType::AccountData {
            offset: 0,
            data_length: 72,
        })
        .instruction(),
    spl_token::instruction::transfer(
        &spl_token::id(),
        &src_token_account,
        &dest_token_account,
        sender,
        &[],
        amount,
    )
    .unwrap(),
    AssertAccountDeltaBuilder::new()
        .account_a(memory)
        .account_b(token_account)
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
        .account_a(memory)
        .account_b(token_account)
        .assertion(AccountDeltaAssertion::Data {
            a_offset: 64,
            b_offset: 64,
            assertion: DataValueDeltaAssertion::U64 {
                value: -70,
                operator: IntegerOperator::GreaterThan,
            },
        })
        .instruction(),
    MemoryCloseBuilder::new()
        .payer(user.encodable_pubkey())
        .program_id(lighthouse_sdk::ID)
        .memory(memory)
        .memory_bump(memory_bump)
        .memory_id(0)
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
