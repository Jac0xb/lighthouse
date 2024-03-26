---
title: Assert Account Info
metaTitle: Assert - Account Info
description:
---

## AssertAccountInfo Instruction

The AccountInfoAssertion exposes the fields accessible by the AccountInfo struct passed into the rust entrypoint during runtime. The struct itself looks like

```rust
pub struct AccountInfo<'a> {
    /// Public key of the account
    pub key: &'a Pubkey,
    /// The lamports in the account. Modifiable by programs.
    pub lamports: Rc<RefCell<&'a mut u64>>,
    /// The data held in this account. Modifiable by programs.
    pub data: Rc<RefCell<&'a mut [u8]>>,
    /// Program that owns this account
    pub owner: &'a Pubkey,
    /// The epoch at which this account will next owe rent
    pub rent_epoch: Epoch,
    /// Was the transaction signed by this account's public key?
    pub is_signer: bool,
    /// Is the account writable?
    pub is_writable: bool,
    /// This account's data contains a loaded program (and is read-only)
    pub executable: bool,
}
```

Lighthouse exposes asserting on these values through the AssertAccountInfo , AssertAccountInfoMulti , and AssertAccountDelta (which is discussed in ???).

### Lamports

It's possible to make assertions on the value of lamports of an account at runtime.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts
TODO
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let ix = AssertAccountInfoBuilder::new()
    .target_account(account_key)
    .assertion(AccountInfoAssertion::Lamports {
        value: 5_000_000,
        operator: IntegerOperator::GreaterThan,
    })
    .instruction();
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Owner

It's possible to make assertions on which programs owns the account.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts
TODO
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let ix = AssertAccountInfoBuilder::new()
    .target_account(account_key)
    .assertion(AccountInfoAssertion::Owner {
        value: system_program::ID,
        operator: EquatableOperator::Equal,
    })
    .instruction();
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### KnownOwner

It's possible to make assertions on which programs owns the account using **KnownOwner** which is a enum of common program ids. This reduces the size of instruction data you need to pack into your transaction by 31 bytes.

```rust
pub enum KnownProgram {
    System,
    Token,
    Token2022,
    Rent,
    Stake,
    Vote,
    BpfLoader,
    UpgradeableLoader,
    SysvarConfig,
}
```

Here is an example of asserting that the account is owned by the system program.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts
TODO
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let ix = AssertAccountInfoBuilder::new()
    .target_account(account_key)
    .assertion(AccountInfoAssertion::KnownOwner {
        value: KnownProgram::System,
        operator: EquatableOperator::Equal,
    })
    .instruction();
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### RentEpoch

It's possible to assert the

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts
TODO
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
AssertAccountInfoBuilder::new()
.target_account(account_key)
.assertion(AccountInfoAssertion::RentEpoch {
value: 0,
operator: IntegerOperator::Equal,
})
.instruction();
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### IsSigner

It's possible to get whether an account is a signer in the runtime.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts

```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust

let ix = AssertAccountInfoBuilder::new()
    .target_account(user_pubkey)
    .log_level(LogLevel::PlaintextMessage) // Logs assertion results.
    .assertion(AccountInfoAssertion::IsSigner {
        value: true,
        operator: EquatableOperator::Equal,
    })
    .instruction();

```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### IsWritable

It's possible to get whether an account is writable in the runtime.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts

```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let ix = AssertAccountInfoBuilder::new()
    .target_account(account_key)
    .assertion(AccountInfoAssertion::IsWritable {
        value: true,
        operator: EquatableOperator::Equal,
    })
    .instruction();

```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Executable

It's possible to get whether an account is an executable account.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts

```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
AssertAccountInfoBuilder::new()
    .target_account(program_id)
    .assertion(AccountInfoAssertion::IsWritable {
        value: true,
        operator: EquatableOperator::Equal,
    })
    .instruction();

```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### VerifyDatahash

To save transaction space it is possible to assert on account data by hashing a slice of the account data and passing it into the VerifyDatahash assertion. This costs more compute but is very useful if you need to verify that a writable account matches exactly what you expected. The type used to represent the slice to be hashed is u16 so you are limited accounts of size `len < 65_535`.
expected_hash - is the expected hash which must be exactly equal to what the lighthouse program hashes at runtime or lighthouse will through an AssertionFailed error.
start - the start index of the account data slice to be hashed. If None, start is 0.
length - the length of the slice to be hashed where the end index of the slice will be `start + length`. If `None`, length is `(length of account data) - start`.

The following is an example using the entire account data.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts

```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust

let hash = keccak::hashv(&[&account.data]).0;

let tx = Transaction::new_signed_with_payer(
&[AssertAccountInfoBuilder::new()
.target_account(account_key)
.assertion(AccountInfoAssertion::VerifyDatahash {
expected_hash: hash,
start: None,
length: None,
})
.instruction()],
Some(&user_pubkey),
&[&user_keypair],
blockhash,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

The following is an example using start and length.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts

```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let hash = keccak::hashv(&[&account.data[128..256]]).0;

let tx = Transaction::new_signed_with_payer(
    &[
        AssertAccountInfoBuilder::new()
            .target_account(account_key)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(128),
                length: Some(128),
            })
            .instruction()
    ],
    Some(&user_pubkey),
    &[&user_keypair],
    blockhash,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### AssertAccountInfoMulti Instruction

To save transaction space there is an instruction AssertAccountInfoMulti which allows you join all your assertions into one vector. This elimiates duplicating instruction data: program id (u8), target account (u8), instruction disciminator (u8), and the compute unit overhead of program entry per assertion (3 bytes per instruction - 4 bytes for vector, ~500 CU per instruction).

Note: The error code is different than for a normal assertion. `0x1900 + (index of failed assertion)`.

This is for indexers who want to know easily determine which assertion failed.

{% dialect-switcher title="" %}
{% dialect title="TypeScript" id="typescript" %}
{% totem %}

```ts
TODO
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let tx = Transaction::new_signed_with_payer(
    &[AssertAccountInfoMultiBuilder::new()
        .target_account(user_pubkey)
        .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
        .assertions(vec![
            AccountInfoAssertion::Owner {
                value: system_program::ID,
                operator: EquatableOperator::Equal,
            },
            AccountInfoAssertion::KnownOwner {
                value: KnownProgram::System,
                operator: EquatableOperator::Equal,
            },
            AccountInfoAssertion::Lamports {
                value: user_prebalance - 5000,
                operator: IntegerOperator::Equal,
            },
            AccountInfoAssertion::DataLength {
                value: 0,
                operator: IntegerOperator::Equal,
            },
            AccountInfoAssertion::Executable {
                value: true,
                operator: EquatableOperator::NotEqual,
            },
            AccountInfoAssertion::Executable {
                value: false,
                operator: EquatableOperator::Equal,
            },
            AccountInfoAssertion::Executable {
                value: true,
                operator: EquatableOperator::NotEqual,
            },
            AccountInfoAssertion::RentEpoch {
                value: account.rent_epoch,
                operator: IntegerOperator::Equal,
            },
        ])
    .instruction()],
    Some(&user_pubkey),
    &[&user_keypair],
    blockhash,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
