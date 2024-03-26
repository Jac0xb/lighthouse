---
title: Overview
metaTitle: Toolbox - Overview
description: Provides a high-level overview of the Toolbox product and what it includes.
---

Mpl Toolbox includes a bunch of essential Solana and Metaplex programs to get you up and running with your decentralized applications. {% .lead %}

{% quick-links %}

{% quick-link title="Getting Started" icon="InboxArrowDown" href="/token-metadata/getting-started" description="Find the language or library of your choice and get started essentials programs." /%}

{% quick-link title="API reference" icon="CodeBracketSquare" href="/token-metadata/references" description="Looking for something specific? Have a peak at our API References and find your answer." /%}

{% /quick-links %}

## Introduction

Whilst all of Metaplex's products offer clients that include all you need to get started with that particular product, they do not include clients for low-level yet essential tasks such as creating an account from the SPL System program or extending a Lookup Table from the SPL Address Lookup Table program. MPL Toolbox aims to fix this by offering a collection of essential Solana and Metaplex programs that can be used to perform these more low-level tasks. Namely, MPL Toolbox includes the following programs:

- [SPL System](#spl-system). The native Solana program that allows us to create accounts.
- [SPL Token and SPL Associated Token](#spl-token-and-spl-associated-token). The native Solana programs that allow us to manage tokens.
- [SPL Memo](#spl-memo). The native Solana program that allows us to attach memos to transactions.
- [SPL Address Lookup Table](#spl-address-lookup-table). The native Solana program that allows us to manage lookup tables.
- [SPL Compute Budget](#spl-compute-budget). The native Solana program that allows us to manage compute units.
- [MPL System Extras](#mpl-system-extras). An immutable Metaplex program that offers a few extra low-level features on top of SPL System.
- [MPL Token Extras](#mpl-token-extras). An immutable Metaplex program that offers a few extra low-level features on top of SPL Token.

## SPL System

The instructions of the SPL System program can be used to create new uninitialized accounts on-chain and transfer SOL between wallets. You can read more about the SPL System program in [Solana's official documentation](https://docs.solana.com/developing/runtime-facilities/programs).

Note that, you may be interested in the [MPL System Extras program](#mpl-system-extras) which offers a few convenient instructions when dealing with creating accounts and transferring SOL.

{% dialect-switcher title="Interact with SPL System" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Create Account" %}

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import { createAccount } from '@metaplex-foundation/mpl-toolbox'

const space = 42
const newAccount = generateSigner(umi)
await createAccount(umi, {
  newAccount,
  lamports: await umi.rpc.getRent(space),
  space,
  programId: umi.programs.get('myProgramName').publicKey,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Transfer SOL" %}

```ts
import { sol } from '@metaplex-foundation/umi'
import { transferSol } from '@metaplex-foundation/mpl-toolbox'

await transferSol(umi, {
  source,
  destination,
  amount: sol(1.3),
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## SPL Token and SPL Associated Token

The SPL Token and SPL Associated Token programs can be used to manage tokens in Solana. It allows us to create Mint accounts, Token accounts, Associated Token PDAs, mint tokens, transfer tokens, delegate tokens, etc. You can read more about these programs in [Solana's official documentation](https://spl.solana.com/token).

Note that, you may be interested in the [Mpl Token Extras program](#mpl-token-extras) which offers a few convenient instructions when dealing with tokens.

{% dialect-switcher title="Manage tokens" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Create Mint" %}

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import { createMint } from '@metaplex-foundation/mpl-toolbox'

const mint = generateSigner(umi)
await createMint(umi, {
  mint,
  decimals: 0,
  mintAuthority,
  freezeAuthority,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Create Token" %}

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import { createToken } from '@metaplex-foundation/mpl-toolbox'

const token = generateSigner(umi)
await createToken(umi, { token, mint, owner }).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Create Associated Token" %}

```ts
import { createAssociatedToken } from '@metaplex-foundation/mpl-toolbox'

await createAssociatedToken(umi, { mint, owner }).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Mint Tokens" %}

```ts
import { mintTokensTo } from '@metaplex-foundation/mpl-toolbox'

await mintTokensTo(umi, {
  mintAuthority,
  mint,
  token,
  amount: 42,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Create Mint with Associated Token (helper)" %}

This helper creates a Mint account and an Associated Token account for the given mint and owner. It also mints tokens to that account if an amount greater than zero is provided.

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import { createMintWithAssociatedToken } from '@metaplex-foundation/mpl-toolbox'

const mint = generateSigner(umi)
await createMintWithAssociatedToken(umi, {
  mint,
  owner,
  amount: 1,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Transfer Tokens" %}

```ts
import { transferTokens } from '@metaplex-foundation/mpl-toolbox'

await transferTokens(umi, {
  source: sourceTokenAccount,
  destination: destinationTokenAccount,
  authority: ownerOrDelegate,
  amount: 30,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Set Authority" %}

```ts
import { setAuthority, AuthorityType } from '@metaplex-foundation/mpl-toolbox'

await setAuthority(umi, {
  owned: tokenAccount,
  owner,
  authorityType: AuthorityType.CloseAccount,
  newAuthority: newCloseAuthority.publicKey,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Fetch Mint and Token accounts" %}

```ts
import {
  fetchMint,
  fetchToken,
  findAssociatedTokenPda,
  fetchAllTokenByOwner,
  fetchAllMintByOwner,
  fetchAllMintPublicKeyByOwner,
} from '@metaplex-foundation/mpl-toolbox'

// Fetch Mint account.
const mintAccount = await fetchMint(umi, mint)

// Fetch Token account.
const tokenAccount = await fetchToken(umi, token)

// Fetch Associated Token account.
const [associatedToken] = findAssociatedTokenPda(umi, { owner, mint })
const associatedTokenAccount = await fetchToken(umi, associatedToken)

// Fetch by owner.
const tokensFromOwner = await fetchAllTokenByOwner(umi, owner)
const mintsFromOwner = await fetchAllMintByOwner(umi, owner)
const mintKeysFromOwner = await fetchAllMintPublicKeyByOwner(umi, owner)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## SPL Memo

The SPL Memo program simply allows us to attach text notes — i.e. memos — to transactions. You can read more about this program in [Solana's official documentation](https://spl.solana.com/memo).

{% dialect-switcher title="Add memos to transactions" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { transactionBuilder } from '@metaplex-foundation/umi'
import { addMemo } from '@metaplex-foundation/mpl-toolbox'

await transactionBuilder()
  .add(...) // Any instruction(s) here.
  .add(addMemo(umi, { memo: 'Hello world!' })) // Add a memo to the transaction.
  .sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

## SPL Address Lookup Table

The SPL Address Lookup Table program can be used to reduce the size of transactions by creating custom lookup tables — a.k.a **LUTs** or **ALTs** — before using them in transactions. This program allows you to create and extend LUTs. You can read more about this program in [Solana's official documentation](https://docs.solana.com/developing/lookup-tables).

{% dialect-switcher title="Manage address lookup tables" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Create empty LUTs" %}

```ts
import { createEmptyLut } from '@metaplex-foundation/mpl-toolbox'

const recentSlot = await umi.rpc.getSlot({ commitment: 'finalized' })
await createEmptyLut(umi, {
  recentSlot,
  authority,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Extend a LUT" %}

```ts
import {
  findAddressLookupTablePda,
  extendLut,
} from '@metaplex-foundation/mpl-toolbox'

// The authority and slot used to create the LUT.
const lutAddress = findAddressLookupTablePda(umi, { authority, recentSlot })

await extendLut(umi, {
  authority,
  address: lutAddress, // The address of the LUT.
  addresses: [addressA, addressB], // The addresses to add to the LUT.
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Create LUT with addresses (helper)" %}

This helper method creates a transaction builder with two instructions: one to create an empty LUT and one to extend it with the given addresses.

```ts
import { createLut } from '@metaplex-foundation/mpl-toolbox'

const recentSlot = await umi.rpc.getSlot({ commitment: 'finalized' })
await createLut(umi, {
  authority,
  recentSlot,
  addresses: [addressA, addressB],
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Create LUT for a transaction builder (helper)" %}

This helper method accepts a "base" transaction builder and a recent slot and returns:

- An array of transaction builders to create all LUTs required by the base transaction builder.
- An array of LUTs to be used in the base transaction builder once the LUTs have been created.

```ts
import { createEmptyLut } from '@metaplex-foundation/mpl-toolbox'

// 1. Get the LUT builders and the LUT accounts for a given transaction builder.
const recentSlot = await umi.rpc.getSlot({ commitment: 'finalized' })
const [createLutBuilders, lutAccounts] = createLutForTransactionBuilder(
  umi,
  baseBuilder,
  recentSlot
)

// 2. Create the LUTs.
for (const createLutBuilder of createLutBuilders) {
  await createLutBuilder.sendAndConfirm(umi)
}

// 3. Use the LUTs in the base transaction builder.
await baseBuilder.setAddressLookupTables(lutAccounts).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Freeze a LUT" %}

```ts
import {
  findAddressLookupTablePda,
  freezeLut,
} from '@metaplex-foundation/mpl-toolbox'

// The authority and slot used to create the LUT.
const lutAddress = findAddressLookupTablePda(umi, { authority, recentSlot })

await freezeLut(umi, {
  authority,
  address: lutAddress,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Deactivate a LUT" %}

```ts
import {
  findAddressLookupTablePda,
  deactivateLut,
} from '@metaplex-foundation/mpl-toolbox'

// The authority and slot used to create the LUT.
const lutAddress = findAddressLookupTablePda(umi, { authority, recentSlot })

await deactivateLut(umi, {
  authority,
  address: lutAddress,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Close a LUT" %}

Note that a LUT can only be closed after it has been deactivated for a certain amount of time.

```ts
import {
  findAddressLookupTablePda,
  closeLut,
} from '@metaplex-foundation/mpl-toolbox'

// The authority and slot used to create the LUT.
const lutAddress = findAddressLookupTablePda(umi, { authority, recentSlot })

await closeLut(umi, {
  authority,
  address: lutAddress,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## SPL Compute Budget

The SPL Compute Budget program allows us to set a custom Compute Unit limit and price. You can read more about this program in [Solana's official documentation](https://docs.solana.com/developing/programming-model/runtime#compute-budget).

{% dialect-switcher title="Manage the Compute Budget of a transaction" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Set Compute Unit limit" %}

```ts
import { transactionBuilder } from '@metaplex-foundation/umi'
import { setComputeUnitLimit } from '@metaplex-foundation/mpl-toolbox'

await transactionBuilder()
  .add(setComputeUnitLimit(umi, { units: 600_000 })) // Set the Compute Unit limit.
  .add(...) // Any instruction(s) here.
  .sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Set Compute Unit price" %}

```ts
import { transactionBuilder } from '@metaplex-foundation/umi'
import { setComputeUnitPrice } from '@metaplex-foundation/mpl-toolbox'

await transactionBuilder()
  .add(setComputeUnitPrice(umi, { microLamports: 1 })) // Set the price per Compute Unit in micro-lamports.
  .add(...) // Any instruction(s) here.
  .sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## MPL System Extras

The MPL System Extras program is an immutable program that offers a few convenient instructions on top of the native SPL System program.

### Create Account with Rent

This instruction creates new accounts without needing to fetch the rent exemption. This instruction uses the `Rent` sysvar on the program to compute the rent exemption from the provided `space` attribute. It then does a CPI call to the SPL System program to create an account with the computed rent.

The advantage is that clients using this instruction no longer need the extra HTTP request that fetches the rent exemption from the RPC node. The inconvenience is that, because we are doing a CPI call, the maximum size account that can be created using this instruction is 10KB, as opposed to 10MB when using the SPL System program directly.

{% dialect-switcher title="Create account with rent" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import { createAccountWithRent } from '@metaplex-foundation/mpl-toolbox'

const newAccount = generateSigner(umi)
await createAccountWithRent(umi, {
  newAccount,
  space: 42,
  programId: umi.programs.get('myProgramName').publicKey,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

### Transfer All SOL

This instruction is similar to the **Transfer SOL** instruction from the SPL System program except that it transfers all the SOL from the source account to the destination account.

This can be useful when we want to drain an account from all of its lamports whilst using this account to pay for the transaction. Without this instruction, we would need to fetch the balance of the account to drain and subtract an estimation of the transaction fee — which can be tricky to estimate when using prioritization fees.

{% dialect-switcher title="Transfer all SOL" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { transferAllSol } from '@metaplex-foundation/mpl-toolbox'

await transferAllSol(umi, {
  source,
  destination,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

## MPL Token Extras

The MPL Token Extras program is an immutable program that offers a few convenient instructions on top of the native SPL Token program.

### Create Token If Missing

This instruction creates a new Token account if and only if it does not already exist. This is useful if a subsequent instruction requires a Token account but we do not know whether it already exists or not. This instruction can be used to ensure the Token account exists without having to fetch it first on the client side.

Whilst this instruction works with both associated Token accounts and regular Token accounts, it's important to note that it will only be able to create an associated Token account if the token account does not already exist. Therefore we can have the following situations:

- The token account is an **associated token account**.
  - If the account exists, the instruction succeeds and does nothing.
  - If the account does not exist, the instruction succeeds and creates the associated token account.
- The token account is a **regular token account** — i.e. non-associated.
  - If the account exists, the instruction succeeds and does nothing.
  - If the account does not exist, the instruction fails.

{% dialect-switcher title="Create token if missing" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { transactionBuilder } from '@metaplex-foundation/umi'
import { createTokenIfMissing } from '@metaplex-foundation/mpl-toolbox'

// If the token account is an associated token account.
await transactionBuilder()
  .add(createTokenIfMissing(umi, { mint, owner }))
  .add(...) // Subsequent instructions can be sure the Associated Token account exists.
  .sendAndConfirm(umi)

// If the token accounts is a regular token account.
await transactionBuilder()
  .add(createTokenIfMissing(umi, { mint, owner, token }))
  .add(...) // Subsequent instructions can be sure the non-Associated Token account exists, otherwise the instruction fails.
  .sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}
