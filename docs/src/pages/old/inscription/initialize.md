---
title: Initialize Inscriptions
metaTitle: Inscriptions - Initialize Inscriptions
description: Learn how to create Metaplex Inscriptions
---

The `initialize` instruction creates the inscription accounts for you where the data will be stored. There are three types of initializations:

1. `initializeFromMint` - for Inscriptions attached to NFTs - **you probably want this**
2. `initialize` - for Inscriptions as a storage provider
3. `initializeAssociatedInscription` - Additional Data Accounts

After the initialization has been done you can [write Data](write) to the inscriptions.

When initializing you can choose a `shard` that is used for numbering. Make sure to use a random one to minimize locks. Read more about [Sharding here](sharding)

## `initializeFromMint`

{% callout type="note" %}

These inscriptions are tradable similar to NFTs. If you are unsure you probably want to use this.

{% /callout %}

If you want a tradable inscription you want to use this kind of inscription. It is derived from your NFT. When using this function you have to be the update authority of the NFT.

It can be done like this:

{% dialect-switcher title="Initialize Mint Inscription" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import {
  findInscriptionShardPda,
  initializeFromMint,
} from '@metaplex-foundation/mpl-inscription'

const inscriptionShardAccount = await findInscriptionShardPda(umi, {
  shardNumber: 0, //random number between 0 and 31
})
await initializeFromMint(umi, {
  mintAccount: mint.publicKey,
  inscriptionShardAccount,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## `Initialize`

{% callout type="warning" %}

This kind of inscriptions is **not tradable**. We recommended it only for advanced use cases like gaming.

{% /callout %}

An Inscription has to be initialized before data can written to it. It can be done like so:

{% dialect-switcher title="Initialize Inscription" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import {
  findInscriptionMetadataPda,
  findInscriptionShardPda,
  initialize,
} from '@metaplex-foundation/mpl-inscription'

const inscriptionAccount = generateSigner(umi)

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
})
const inscriptionShardAccount = await findInscriptionShardPda(umi, {
  shardNumber: 0, //random number between 0 and 31
})

await initialize(umi, {
  inscriptionAccount,
  inscriptionShardAccount,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}


## `initializeAssociatedInscription`

One Inscription account can have multiple Associated Inscription Accounts. They are derived based on the `associationTag`. For example the tag can be the datatype of the file, e.g. `image/png`.

Pointers to the associated inscriptions are stored in an array in the `inscriptionMetadata` Account in the field `associatedInscriptions`.

To initialize a new Associated Inscription you can use the following function:

{% dialect-switcher title="Initialize Associated Inscription" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import {
  findInscriptionMetadataPda,
  initializeAssociatedInscription,
} from '@metaplex-foundation/mpl-inscription'

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
})

await initializeAssociatedInscription(umi, {
  inscriptionMetadataAccount,
  associationTag: 'image/png',
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
