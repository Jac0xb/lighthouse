---
title: Assert Account Delta
metaTitle: Assert - Account Delta
description:
---

## AssertStakeAccount Instruction

The **AssertStakeAccount** instruction is for making assertions on the various states and fields of a stake account.

This could also be accomplished by using the [AssertAccountData](/assert/account-data) instruction, but this instruction is a convenience instruction for stake accounts which checks that the account is owned by the stake program and maps enums to offset / type deserialization.

The stake account is an enum that looks like

```rust
pub enum StakeStateV2 {
    Uninitialized,
    Initialized(Meta),
    Stake(Meta, Stake, StakeFlags),
    RewardsPool,
}
```

And the assertion types for a stake account are as follows:

```rust
pub enum StakeAccountAssertion {
    State {
        value: StakeStateType,
        operator: EquatableOperator,
    },
    MetaAssertion(MetaAssertion),
    StakeAssertion(StakeAssertion),
    StakeFlags {
        value: u8,
        operator: IntegerOperator,
    },
}
```

You can make assertions about the `Meta` and `Stake` fields of the stake account, as well as the `StakeFlags`. `StakeAccountAssertionState` is an assertion type that lets you assert on the state of the account.

If you make an assertion about the `Meta` or `Stake` structs and the stake account is in the `Uninitialized` or `RewardsPool` state, the assertion will fail.

If you make an assertion about the `StakeFlags` field and the account is in the `Uninitialized`, `RewardsPool`, or `Initialized` state, the assertion will fail.

The `MetaAssertion` and `StakeAssertion` enums are as follows:

```rust
pub enum MetaAssertion {
    RentExemptReserve {
        value: u64,
        operator: IntegerOperator,
    },
    AuthorizedStaker {
        value: Pubkey,
        operator: EquatableOperator,
    },
    AuthorizedWithdrawer {
        value: Pubkey,
        operator: EquatableOperator,
    },
    LockupUnixTimestamp {
        value: i64,
        operator: IntegerOperator,
    },
    LockupEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    LockupCustodian {
        value: Pubkey,
        operator: EquatableOperator,
    },
}
```

```rust
pub enum StakeAssertion {
    DelegationVoterPubkey {
        value: Pubkey,
        operator: EquatableOperator,
    },
    DelegationStake {
        value: u64,
        operator: IntegerOperator,
    },
    DelegationActivationEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    DelegationDeactivationEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    CreditsObserved {
        value: u64,
        operator: IntegerOperator,
    },
}
```

### Example: Asserting on the state of a stake account

In this example, we assert that the stake account is in the `Stake` state.

{% dialect-switcher title="Assert state instruction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertStakeAccount(umi, {
  targetAccount: publicKey(stakeAccount),
  assertion: {
    __kind: 'State',
    value: StakeStateType.Stake,
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
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::State {
                value: StakeStateType::Stake,
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

### Example: Asserting on the meta of a stake account

Using the **solana_sdk** and deserializing an example state account in the state of `Stake` or `Initialized`, we can make assertions on the stake account. The following assertions will pass:

{% dialect-switcher title="Assert meta instruction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const meta = assertStakeAccount(umi, {
  targetAccount,
  assertion: {
    __kind: 'MetaAssertion',
    fields: [
      {
        __kind: 'LockupCustodian',
        value: publicKey(meta.lockup.custodian),
        operator: EquatableOperator.Equal,
      },
    ],
  },
})
  .append(
    assertStakeAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'MetaAssertion',
        fields: [
          {
            __kind: 'LockupEpoch',
            value: meta.lockup.epoch,
            operator: IntegerOperator.Equal,
          },
        ],
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'MetaAssertion',
        fields: [
          {
            __kind: 'LockupUnixTimestamp',
            value: meta.lockup.unixTimestamp,
            operator: IntegerOperator.Equal,
          },
        ],
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'MetaAssertion',
        fields: [
          {
            __kind: 'AuthorizedStaker',
            value: publicKey(meta.authorized.staker),
            operator: EquatableOperator.Equal,
          },
        ],
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'MetaAssertion',
        fields: [
          {
            __kind: 'AuthorizedWithdrawer',
            value: publicKey(meta.authorized.withdrawer),
            operator: EquatableOperator.Equal,
          },
        ],
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'MetaAssertion',
        fields: [
          {
            __kind: 'RentExemptReserve',
            value: meta.rentExemptReserve,
            operator: IntegerOperator.Equal,
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
let tx: Transaction = Transaction::new_signed_with_payer(
    &[
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupCustodian {
                    value: meta.lockup.custodian,
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupEpoch {
                    value: meta.lockup.epoch,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupUnixTimestamp {
                    value: meta.lockup.unix_timestamp,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedStaker {
                    value: meta.authorized.staker,
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedWithdrawer {
                    value: meta.authorized.withdrawer,
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::RentExemptReserve {
                    value: meta.rent_exempt_reserve,
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

### Example: Asserting on the stake state of a stake account

Using the **solana_sdk** and deserializing an example state account in the state of `Stake`, we can make assertions on the stake account. The following assertions will pass:

{% dialect-switcher title="Assert stake instruction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertStakeAccount(umi, {
  targetAccount: publicKey(stakeAccount),
  assertion: {
    __kind: 'StakeAssertion',
    fields: [
      {
        __kind: 'CreditsObserved',
        value: stake.creditsObserved,
        operator: IntegerOperator.Equal,
      },
    ],
  },
})
  .append(
    assertStakeAccount(umi, {
      targetAccount: publicKey(stakeAccount),
      assertion: {
        __kind: 'StakeAssertion',
        fields: [
          {
            __kind: 'DelegationStake',
            value: stake.delegation.stake,
            operator: IntegerOperator.Equal,
          },
        ],
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount: publicKey(stakeAccount),
      assertion: {
        __kind: 'StakeAssertion',
        fields: [
          {
            __kind: 'DelegationDeactivationEpoch',
            value: stake.delegation.deactivationEpoch,
            operator: IntegerOperator.Equal,
          },
        ],
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount: publicKey(stakeAccount),
      assertion: {
        __kind: 'StakeAssertion',
        fields: [
          {
            __kind: 'DelegationActivationEpoch',
            value: stake.delegation.activationEpoch,
            operator: IntegerOperator.Equal,
          },
        ],
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount: publicKey(stakeAccount),
      assertion: {
        __kind: 'StakeAssertion',
        fields: [
          {
            __kind: 'DelegationVoterPubkey',
            value: publicKey(stake.delegation.voterPubkey),
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
let tx: Transaction = Transaction::new_signed_with_payer(
    &[
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::CreditsObserved {
                    value: stake.credits_observed,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationStake {
                    value: stake.delegation.stake,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationDeactivationEpoch {
                    value: stake.delegation.deactivation_epoch,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationActivationEpoch {
                    value: stake.delegation.activation_epoch,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationVoterPubkey {
                    value: stake.delegation.voter_pubkey,
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

### Example: Asserting on the stake flags of a stake account

The `IntegerOperator` allows you to make bitwise assertions on the `StakeFlags` field of the stake account.

Assuming the `StakeFlags` of a particular stake account is `0b00000000`, the following assertions will pass:

{% dialect-switcher title="Assert stake flags instruction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertStakeAccount(umi, {
  targetAccount: publicKey(stakeAccount),
  assertion: {
    __kind: 'StakeFlags',
    value: 255,
    operator: IntegerOperator.DoesNotContain,
  },
})
  .append(
    assertStakeAccount(umi, {
      targetAccount: publicKey(stakeAccount),
      assertion: {
        __kind: 'StakeFlags',
        value: 0,
        operator: IntegerOperator.Contains,
      },
    })
  )
  .append(
    assertStakeAccount(umi, {
      targetAccount: publicKey(stakeAccount),
      assertion: {
        __kind: 'StakeFlags',
        value: 0,
        operator: IntegerOperator.Equal,
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
let tx: Transaction = Transaction::new_signed_with_payer(
    &[
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeFlags {
                value: u8::MAX,
                operator: IntegerOperator::DoesNotContain,
            })
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeFlags {
                value: u8::MIN,
                operator: IntegerOperator::Contains,
            })
            .instruction(),
        AssertStakeAccountBuilder::new()
            .target_account(stake_account)
            .assertion(StakeAccountAssertion::StakeFlags {
                value: u8::MIN,
                operator: IntegerOperator::Equal,
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
