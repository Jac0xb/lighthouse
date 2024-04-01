---
title: Assert Token Account
metaTitle: Assert - Token Account
description:
---

## AssertTokenAccount Instruction

The **AssertTokenAccount** instruction is for making assertions on the data of a spl-token token account.

This could also be accomplished by using the [AssertAccountData](/assert/account-data) instruction, but this instruction is a convenience instruction for token accounts which checks that the account is owned by the spl-token program and maps enums to offset / type deserialization.

Below are the types of assertions you can make about a mint account:

```rust
pub enum TokenAccountAssertion {
    Mint {
        value: Pubkey,
        operator: EquatableOperator,
    },
    Owner {
        value: Pubkey,
        operator: EquatableOperator,
    },
    Amount {
        value: u64,
        operator: IntegerOperator,
    },
    Delegate {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    State {
        value: u8,
        operator: IntegerOperator,
    },
    IsNative {
        value: Option<u64>,
        operator: EquatableOperator,
    },
    DelegatedAmount {
        value: u64,
        operator: IntegerOperator,
    },
    CloseAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    TokenAccountOwnerIsDerived,
}
```

{% callout type="warning" %}
**Note**: This instruction will only check that this account is **owned** by the spl-token program and not the type of account it is. Checking the type of account should be done by the transaction builder.
{% /callout %}

### Example: multi-assertion instruction building showcase.

Assuming there is a token account that looks like the following

```rust
let account = Account {
    mint: mint_key,
    owner: user_key,
    amount: 100,
    delegate: None,
    state: TokenAccountState::Initialized,
    is_native: None,
    delegated_amount: 0,
    close_authority: None,
};
```

Then, the following example shows how to build an instruction that asserts the token account's data.

{% dialect-switcher title="Example of the types of assertions there are for the token account" %}
{% dialect title="Rust" id="rust" %}

```rust
let tx = Transaction::new_signed_with_payer(
    &[AssertTokenAccountMultiBuilder::new()
        .target_account(token_account)
        .assertions(vec![
            TokenAccountAssertion::Mint {
                value: mint_key,
                operator: EquatableOperator::Equal,
            },
            TokenAccountAssertion::Owner {
                value: user_key,
                operator: EquatableOperator::Equal,
            },
            TokenAccountAssertion::Amount {
                value: 100,
                operator: IntegerOperator::Equal,
            },
            TokenAccountAssertion::Delegate {
                value: None,
                operator: EquatableOperator::Equal,
            },
            TokenAccountAssertion::State {
                value: TokenAccountState::Frozen as u8,
                operator: IntegerOperator::NotEqual,
            },
            TokenAccountAssertion::IsNative {
                value: None,
                operator: EquatableOperator::Equal,
            },
            TokenAccountAssertion::DelegatedAmount {
                value: 0,
                operator: IntegerOperator::LessThanOrEqual,
            },
            TokenAccountAssertion::CloseAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            },
            TokenAccountAssertion::TokenAccountOwnerIsDerived,
        ])
        .instruction()],
    Some(&user_key),
    &[&user_keypair],
    blockhash,
);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

{% callout %}
The enum variant **TokenAccountOwnerIsDerived** is a special assertion that checks if account key is a PDA using the current owner and the mint. This is a optimization for saving transaction space since you do not build owner or mint key assertions.
{% /callout %}
