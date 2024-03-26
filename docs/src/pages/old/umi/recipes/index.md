---
title: Recipes
metaTitle: Umi - Recipes
description: How-to guides for Metaplex's Umi product.
---

## Passing Transactions from Backend to Frontend (and vice versa)

When passing a transaction from backend to Frontend or vice versa you will need to serialize the transaction in order to transport it between locations. This can also be performed the same way in reverse.

#### Backend

```ts
const backendSigner = generateSigner(umi)
// Create a noop signer that allows you to sign later
const frontEndSigner = createNoopSigner(
  'tst24HZ6pbcnraCv4r8acexfgXvyQwMSRgZRCg9gEX1'
)

const ix = createNft(umi, {
  mint,
  name: 'My NFT',
  uri: 'https://example.com/my-nft.json',
  sellerFeeBasisPoints: percentAmount(5.5),
  tokenOwner: newOwner.publicKey,
  token: newOwnerTokenAccount,
}).getInstructions()

const blockhash = await umi.rpc.getLatestBlockhash()

const tx = umi.transactions.create({
  version: 0,
  payer: umi.identity.publicKey,
  instructions: ix,
  blockhash: blockhash.blockhash,
})

backendSigner.signTransaction(tx)

const serialized = umi.transactions.serialize(tx)

// Return the transaction to your front end as you see fit.
return { status: 200, tx: serialized }
```

#### Front End

```ts
// On the front end deserialize the transaction
const deserialized = umi.transactions.deserialize(serialized)

// Get the user to sign the transaction
await umi.identity.signTransaction(deserialized)

// Send the transaction
await umi.rpc.sendTransaction(deserialized)
```
