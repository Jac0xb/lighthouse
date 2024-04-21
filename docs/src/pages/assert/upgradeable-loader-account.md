---
title: Assert Upgradeable Loader Account
metaTitle: Assert - Upgradeable Loader Account
description:
---

## AssertUpgradeableLoaderAccount Instruction

The **AssertUpgradeableLoaderAccount** instruction is for making assertions on the states and fields of an upgradeable loader account.

This could also be accomplished by using the [AssertAccountData](/assert/account-data) instruction, but this instruction is a convenience instruction for stake accounts which checks that the account is owned by the stake program and maps enums to offset / type deserialization.

The upgradeable loader account is an enum that looks like

```rust
pub enum UpgradeableLoaderState {
    /// Account is not initialized.
    Uninitialized,
    /// A Buffer account.
    Buffer {
        /// Authority address
        authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },
    /// An Program account.
    Program {
        /// Address of the ProgramData account.
        programdata_address: Pubkey,
    },
    // A ProgramData account.
    ProgramData {
        /// Slot that the program was last modified.
        slot: u64,
        /// Address of the Program's upgrade authority.
        upgrade_authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },
}
```

The assertion enum consists of the following

```rust
pub enum UpgradeableLoaderStateAssertion {
    State {
        value: UpgradeableLoaderStateType,
        operator: EquatableOperator,
    },
    Buffer(UpgradableBufferAssertion),
    Program(UpgradeableProgramAssertion),
    ProgramData(UpgradeableProgramDataAssertion),
}
```

You can make assertions about the `State`, `Buffer`, `Program`, and `ProgramData` structs of the upgradeable loader account. `UpgradeableLoaderStateType` is an enum that represents the state of the upgradeable loader account.

The `UpgradableBufferAssertion` struct is as follows:

```rust
pub enum UpgradableBufferAssertion {
    Authority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
}
```

The `UpgradeableProgramAssertion` struct is as follows:

```rust
pub enum UpgradeableProgramAssertion {
    ProgramDataAddress {
        value: Pubkey,
        operator: EquatableOperator,
    },
}
```

The `UpgradeableProgramDataAssertion` struct is as follows:

```rust
pub enum UpgradeableProgramDataAssertion {
    UpgradeAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    Slot {
        value: u64,
        operator: IntegerOperator,
    },
}

```

### Example: Asserting on the state of an upgradeable loader account

In this example, we assert that the upgradeable loader account is in the `Buffer` state.

{% dialect-switcher title="Assert state transaction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertUpgradeableLoaderAccount(umi, {
  targetAccount: publicKey(upgradeableLoaderAccountPubkey),
  assertion: {
    __kind: 'State',
    value: UpgradeableLoaderStateType.Buffer,
    operator: EquatableOperator.Equal,
  },
}).build(umi)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let tx: Transaction = Transaction::new_signed_with_payer(
    &[
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(upgradeable_loader_account)
            .assertion(UpgradeableLoaderStateAssertion::State {
                value: UpgradeableLoaderStateType::Buffer,
                operator: EquatableOperator::Equal,
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

### Example: Asserting on the buffer of an upgradeable loader account

In this example, we assert that the authority of the buffer of the upgradeable loader account is equal to a specific pubkey.

{% dialect-switcher title="Assert state + assert buffer transaction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertUpgradeableLoaderAccount(umi, {
  targetAccount: publicKey(upgradeableLoaderAccountPubkey),
  assertion: {
    __kind: 'State',
    value: UpgradeableLoaderStateType.Buffer,
    operator: EquatableOperator.Equal,
  },
})
  .append(
    assertUpgradeableLoaderAccount(umi, {
      targetAccount: publicKey(upgradeableLoaderAccountPubkey),
      assertion: {
        __kind: 'Buffer',
        fields: [
          {
            __kind: 'Authority',
            value: publicKey(authorityKey),
            operator: EquatableOperator.Equal,
          },
        ],
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
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_key)
            .assertion(UpgradeableLoaderStateAssertion::State {
                value: UpgradeableLoaderStateType::Buffer,
                operator: EquatableOperator::Equal,
            })
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_key)
            .assertion(UpgradeableLoaderStateAssertion::Buffer(
                UpgradableBufferAssertion::Authority {
                    value: Some(authority_key),
                    operator: EquatableOperator::Equal,
                },
            ))
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

### Example: Asserting on the program of an upgradeable loader account

In this example, we assert that the program data address of the upgradeable loader account is equal to a specific pubkey.

{% dialect-switcher title="Assert state + assert program transaction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertUpgradeableLoaderAccount(umi, {
  targetAccount: publicKey(programKey),
  assertion: {
    __kind: 'State',
    value: UpgradeableLoaderStateType.Program,
    operator: EquatableOperator.Equal,
  },
})
  .append(
    assertUpgradeableLoaderAccount(umi, {
      targetAccount: publicKey(programKey),
      assertion: {
        __kind: 'Program',
        fields: [
          {
            __kind: 'ProgramDataAddress',
            value: publicKey(programDataKey),
            operator: EquatableOperator.Equal,
          },
        ],
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
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_key)
            .assertion(UpgradeableLoaderStateAssertion::State {
                value: UpgradeableLoaderStateType::Program,
                operator: EquatableOperator::Equal,
            })
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_key)
            .assertion(UpgradeableLoaderStateAssertion::Program(
                UpgradeableProgramAssertion::ProgramDataAddress {
                    value: programdata_key,
                    operator: EquatableOperator::Equal,
                },
            ))
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

### Example: Asserting on the program data of an upgradeable loader account

In this example, we assert that the upgrade authority of the program data of the upgradeable loader account is equal to a specific pubkey and that the slot of the upgrade matches a specific slot.

{% dialect-switcher title="Assert state + assert program data transaction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertUpgradeableLoaderAccount(umi, {
  targetAccount: publicKey(programDataKey),
  assertion: {
    __kind: 'State',
    value: UpgradeableLoaderStateType.ProgramData,
    operator: EquatableOperator.Equal,
  },
})
  .append(
    assertUpgradeableLoaderAccount(umi, {
      targetAccount: publicKey(programDataKey),
      assertion: {
        __kind: 'ProgramData',
        fields: [
          {
            __kind: 'UpgradeAuthority',
            value: upgradeAuthority,
            operator: EquatableOperator.Equal,
          },
        ],
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
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(programdata_key)
            .assertion(UpgradeableLoaderStateAssertion::State {
                value: UpgradeableLoaderStateType::ProgramData,
                operator: EquatableOperator::Equal,
            })
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(programdata_key)
            .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                UpgradeableProgramDataAssertion::UpgradeAuthority {
                    value: Some(upgrade_authority),
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(programdata_key)
            .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                UpgradeableProgramDataAssertion::Slot {
                    value: expected_slot,
                    operator: IntegerOperator::Equal,
                },
            ))
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
