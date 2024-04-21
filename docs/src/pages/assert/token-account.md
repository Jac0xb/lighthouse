---
title: Assert Token Account
metaTitle: Assert - Token Account
description:
---

## AssertTokenAccount Instruction

The **AssertTokenAccount** instruction is for making assertions on the fields of a spl-token token account struct.

This could also be accomplished by using the [AssertAccountData](/assert/account-data) instruction, but this instruction is a convenience instruction for token accounts which checks that the account is owned by the spl-token program and maps enums to offset / type deserialization.

Below are the types of assertions you can make about a token account:

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
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const tx = await pipe(
  createTransaction({ version: 0 }),
  (tx) =>
    appendTransactionInstructions(
      [
        getAssertTokenAccountMultiInstruction({
          targetAccount,
          assertions: [
            tokenAccountAssertion('Mint', {
              value: mintKey,
              operator: EquatableOperator.Equal,
            }),
            tokenAccountAssertion('Owner', {
              value: userKey,
              operator: EquatableOperator.Equal,
            }),
            tokenAccountAssertion('Amount', {
              value: 100,
              operator: IntegerOperator.Equal,
            }),
            tokenAccountAssertion('Delegate', {
              value: null,
              operator: EquatableOperator.Equal,
            }),
            tokenAccountAssertion('State', {
              value: 3,
              operator: IntegerOperator.NotEqual,
            }),
            tokenAccountAssertion('IsNative', {
              value: null,
              operator: EquatableOperator.Equal,
            }),
            tokenAccountAssertion('DelegatedAmount', {
              value: 0,
              operator: IntegerOperator.LessThanOrEqual,
            }),
            tokenAccountAssertion('CloseAuthority', {
              value: null,
              operator: EquatableOperator.Equal,
            }),
            tokenAccountAssertion('TokenAccountOwnerIsDerived'),
          ],
        }),
      ],
      tx
    ),
  (tx) => setTransactionFeePayer(userPubkey, tx),
  (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  (tx) => signTransaction([userKeyPair], tx)
)
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const tx = assertTokenAccountMulti(umi, {
  targetAccount: publicKey(tokenAccount),
  assertions: [
    {
      __kind: 'Mint',
      value: publicKey(mintKey),
      operator: EquatableOperator.Equal,
    },
    {
      __kind: 'Owner',
      value: publicKey(userKey),
      operator: EquatableOperator.Equal,
    },
    {
      __kind: 'Amount',
      value: 100,
      operator: IntegerOperator.Equal,
    },
    {
      __kind: 'Delegate',
      value: null,
      operator: EquatableOperator.Equal,
    },
    {
      __kind: 'State',
      value: 1,
      operator: IntegerOperator.NotEqual,
    },
    {
      __kind: 'IsNative',
      value: null,
      operator: EquatableOperator.Equal,
    },
    {
      __kind: 'DelegatedAmount',
      value: 0,
      operator: IntegerOperator.LessThanOrEqual,
    },
    {
      __kind: 'CloseAuthority',
      value: null,
      operator: EquatableOperator.Equal,
    },
    {
      __kind: 'TokenAccountOwnerIsDerived',
    },
  ],
}).build(umi)
```

{% /totem %}
{% /dialect %}
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
The enum variant **TokenAccountOwnerIsDerived** is a special assertion that checks if token account's account info key is a PDA using the current owner and the mint as seeds. This is a optimization for saving transaction space since you do not build owner or mint key assertions.
{% /callout %}
