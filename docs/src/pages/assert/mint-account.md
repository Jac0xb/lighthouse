---
title: Assert Mint Account
metaTitle: Assert - Mint Account
description:
---

## AssertMintAccount Instruction

The **AssertMintAccount** instruction is for making assertions on the fields of a spl-token mint account struct.

This could also be accomplished by using the [AssertAccountData](/assert/account-data) instruction, but this instruction is a convenience instruction for mint accounts which checks that the account is owned by the spl-token program and maps enums to offset / type deserialization.

Below are the types of assertions you can make about a mint account:

```rust
pub enum MintAccountAssertion {
    MintAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    Supply {
        value: u64,
        operator: IntegerOperator,
    },
    Decimals {
        value: u8,
        operator: IntegerOperator,
    },
    IsInitialized {
        value: bool,
        operator: EquatableOperator,
    },
    FreezeAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
}
```

{% callout type="warning" %}
**Note**: This instruction will only check that this account is **owned** by the spl-token program and not the type of account it is. Checking the type of account should be done by the transaction builder.
{% /callout %}

### Example: showcase of the mint assertions.

Assuming there is a mint account that looks like the following

```rust
let mint = Mint {
    mint_authority: user_key,
    supply: 69_000,
    decimals: 9,
    is_initialized: true,
    freeze_authority: COption::None,
};
```

Then, the following assertions could be made about that account.

{% dialect-switcher title="Example of the types of assertions there are for the mint account" %}
{% dialect title="web3.js (Preview)" id="js-preview" %}
{% totem %}

```typescript
const tx = await pipe(
  createTransaction({ version: 0 }),
  (tx) =>
    appendTransactionInstructions(
      [
        getAssertTokenAccountMultiInstruction({
          targetAccount: mintKey,
          assertions: [
            mintAccountAssertion('MintAuthority', {
              value: userKey,
              operator: EquatableOperator.Equal,
            }),
            mintAccountAssertion('Supply', {
              value: 69_000,
              operator: IntegerOperator.Equal,
            }),
            mintAccountAssertion('Decimals', {
              value: 9,
              operator: IntegerOperator.Equal,
            }),
            mintAccountAssertion('IsInitialized', {
              value: true,
              operator: EquatableOperator.Equal,
            }),
            mintAccountAssertion('FreezeAuthority', {
              value: null,
              operator: EquatableOperator.Equal,
            }),
          ],
        }),
      ],
      tx
    ),
  (tx) => setTransactionFeePayer(userKey, tx),
  (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  (tx) => signTransaction([userKeyPair], tx)
)
```

{% /totem %}
{% /dialect %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const ixs = assertMintAccount(umi, {
  targetAccount,
  assertion: {
    __kind: 'MintAuthority',
    value: publicKey(userPubkey),
    operator: EquatableOperator.Equal,
  },
})
  .append(
    assertMintAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'Supply',
        value: 69_000,
        operator: IntegerOperator.Equal,
      },
    })
  )
  .append(
    assertMintAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'Decimals',
        value: 9,
        operator: IntegerOperator.Equal,
      },
    })
  )
  .append(
    assertMintAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'IsInitialized',
        value: true,
        operator: EquatableOperator.Equal,
      },
    })
  )
  .append(
    assertMintAccount(umi, {
      targetAccount,
      assertion: {
        __kind: 'FreezeAuthority',
        value: null,
        operator: EquatableOperator.Equal,
      },
    })
  )
  .build(umi)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}

```rust
let tx = Transaction::new_signed_with_payer(
    &[
        AssertMintAccountBuilder::new()
            .target_account(mint_key)
            .assertion(MintAccountAssertion::MintAuthority {
                value: Some(user_key),
                operator: EquatableOperator::Equal,
            })
            .instruction(),
        AssertMintAccountBuilder::new()
            .target_account(mint_key)
            .assertion(MintAccountAssertion::Supply {
                value: 69_000,
                operator: IntegerOperator::Equal,
            })
            .instruction(),
        AssertMintAccountBuilder::new()
            .target_account(mint_key)
            .assertion(MintAccountAssertion::Decimals {
                value: 9,
                operator: IntegerOperator::Equal,
            })
            .instruction(),
        AssertMintAccountBuilder::new()
            .target_account(mint_key)
            .assertion(MintAccountAssertion::IsInitialized {
                value: true,
                operator: EquatableOperator::Equal,
            })
            .instruction(),
        AssertMintAccountBuilder::new()
            .target_account(mint_key)
            .assertion(MintAccountAssertion::FreezeAuthority {
                value: None,
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
{% enddialect-switcher %}
