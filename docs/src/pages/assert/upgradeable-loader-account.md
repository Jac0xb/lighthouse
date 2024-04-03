---
title: Assert Upgradeable Loader Account
metaTitle: Assert - Upgradeable Loader Account
description:
---

## AssertUpgradeableLoaderAccount Instruction

The **AssertUpgradeableLoaderAccount** instruction is for making assertions on the data of an upgradeable loader account.

This could also be accomplished by using the [AssertAccountData](/assert/account-data) instruction, but this instruction is a convenience instruction for stake accounts which checks that the account is owned by the stake program and maps enums to offset / type deserialization.

The upgradeable loader account is an enum that looks like

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
        #[cfg_attr(
            feature = "serde",
            serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
        )]
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

{% dialect-switcher title="" %}
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

{% dialect-switcher title="" %}
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

{% dialect-switcher title="" %}
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

In this example, we assert that the upgrade authority of the program data of the upgradeable loader account is equal to a specific pubkey.

{% dialect-switcher title="" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let tx = Transaction::new_signed_with_payer(
    &[
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(programdata_key)
            .log_level(lighthouse_sdk::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::State {
                value: UpgradeableLoaderStateType::ProgramData,
                operator: EquatableOperator::Equal,
            })
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(programdata_key)
            .log_level(lighthouse_sdk::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                UpgradeableProgramDataAssertion::UpgradeAuthority {
                    value: Some(upgrade_authority),
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(programdata_key)
            .log_level(lighthouse_sdk::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                UpgradeableProgramDataAssertion::Slot {
                    value: u64::MAX,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
    ],

    &[&user_keypair],
    blockhash,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
