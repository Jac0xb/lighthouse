---
title: Overview
metaTitle: Inscriptions - Overview
description: Provides a high-level overview of the Metaplex Inscriptions standard.
---

The Metaplex Inscription Program allows you to write data directly to Solana, using the blockchain as a method of data storage. The Inscription program also allows for this data storage to be optionally linked to an NFT. In this overview, we explain how this program works and how we can leverage its various features at a high level. {% .lead %}

{% quick-links %}

{% quick-link title="Getting Started" icon="InboxArrowDown" href="/inscription/getting-started" description="Find the language or library of your choice and get started with digital assets on Solana." /%}

{% quick-link title="API reference" icon="CodeBracketSquare" href="/inscription/references" description="Looking for something specific? Have a peak at our API References and find your answer." /%}

{% /quick-links %}

## Introduction

NFT JSON data and Images have historically been stored on decentralized storage providers like Arweave or IPFS. The Inscription program introduces Solana as another option for NFT data storage, allowing you to write that data directly to the chain. The Metaplex Inscription program introduces the novel use case of all of an NFT's associated data now being stored on Solana. This enables many new use cases such as Solana programs with trait-based bids, dynamic images that are updated via programs, or even RPG game state on-chain.

There are two different kinds of Inscriptions:

1. Inscriptions **[attached to NFT Mints](#inscriptions-attached-to-nft-mints)** - NFT data is written to the chain instead or in addition to off chain storage
2. Inscriptions as **[storage providers](#inscriptions-as-storage-provider)** - Write arbitrary data to the chain

### Inscriptions attached to NFT Mints

Inscriptions can be used in addition to off chain storage like Arweave, where the metadata JSON and the media is stored, or can be completely replace those off chain storage using the [Inscription Gateway](#inscription-gateway).

In both cases the same process to create the inscription is used. When using the gateway the only difference is the URI used in the on-chain metadata. Read more on this in the [Gateway section](#inscription-gateway).

When storing the NFT Metadata on-chain three inscription accounts are used:

1. `inscriptionAccount` which stores the JSON Metadata.
2. `inscriptionMetadata` which stores the metadata of the inscription
3. `associatedInscriptionAccount` which is storing the media / image.

{% diagram height="h-64 md:h-[500px]" %}

{% node %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node theme="dimmed" %}
Owner: Token Program {% .whitespace-nowrap %}
{% /node %}
{% /node %}

{% node parent="mint" x="-17" y="180" %}
{% node #inscriptionAccount theme="crimson" %}
Inscription Account {% .whitespace-nowrap %}
{% /node %}
{% node theme="dimmed" %}
Owner: Inscription Program {% .whitespace-nowrap %}
{% /node %}
{% /node %}

{% node parent="inscriptionAccount" x="-40" y="160" %}
{% node #inscriptionMetadata theme="crimson" %}
Inscription Metadata Account {% .whitespace-nowrap %}
{% /node %}
{% node label="Owner: Inscription Program" theme="dimmed" /%}
{% /node %}

{% node parent="inscriptionMetadata" x="500" y="0" %}
{% node #associatedInscription theme="crimson" %}
Associated Inscription Account {% .whitespace-nowrap %}
{% /node %}
{% node label="Owner: Inscription Program" theme="dimmed" /%}
{% /node %}

{% edge from="mint" to="metadata" path="straight" /%}
{% edge from="mint" to="inscriptionAccount" path="straight" %}
Seeds:

"Inscription"

programId

mintAddress
{% /edge %}
{% edge from="inscriptionAccount" to="inscriptionMetadata" path="straight" %}
Seeds:

"Inscription"

programId

inscriptionAccount
{% /edge %}

{% edge from="inscriptionMetadata" to="associatedInscription" path="straight" %}
Seeds:

"Inscription"

"Association"

associationTag

inscriptionMetadataAccount

{% /edge %}

{% /diagram %}

The below script creates both of these Accounts for you and points the newly minted NFT to the Metaplex gateway. With this your NFT is completely on-chain.

{% dialect-switcher title="Inscribe Data for new NFT using the Gateway" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
const umi = await createUmi()
umi.use(mplTokenMetadata())
umi.use(mplInscription())

// Create and mint the NFT to be inscribed.
const mint = generateSigner(umi)
const inscriptionAccount = await findMintInscriptionPda(umi, {
  mint: mint.publicKey,
})
await createV1(umi, {
  mint,
  name: 'My NFT',
  uri: `https://igw.metaplex.com/devnet/${inscriptionAccount[0]}`,
  sellerFeeBasisPoints: percentAmount(5.5),
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)

await mintV1(umi, {
  mint: mint.publicKey,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount[0],
})

let builder = new TransactionBuilder()

// We initialize the Inscription and create the account where the JSON will be stored.
builder = builder.add(
  initializeFromMint(umi, {
    mintAccount: mint.publicKey,
  })
)

// And then write the JSON data for the NFT to the Inscription account.
builder = builder.add(
  writeData(umi, {
    inscriptionAccount: inscriptionAccount[0],
    inscriptionMetadataAccount,
    value: Buffer.from(
      '{"description": "A bread! But on-chain!", "external_url": "https://breadheads.io"}'
    ),
    associatedTag: null,
    offset: 0,
  })
)

// We then create the associated Inscription that will contain the image.
const associatedInscriptionAccount = findAssociatedInscriptionPda(umi, {
  associated_tag: 'image',
  inscriptionMetadataAccount,
})

builder = builder.add(
  initializeAssociatedInscription(umi, {
    inscriptionMetadataAccount,
    associatedInscriptionAccount,
    associationTag: 'image',
  })
)

await builder.sendAndConfirm(umi, { confirm: { commitment: 'finalized' } })

// Open the image file to fetch the raw bytes.
const imageBytes: Buffer = await fs.promises.readFile('bread.png')

// And write the image.
const chunkSize = 800
for (let i = 0; i < imageBytes.length; i += chunkSize) {
  const chunk = imageBytes.slice(i, i + chunkSize)
  await writeData(umi, {
    inscriptionAccount: associatedInscriptionAccount,
    inscriptionMetadataAccount,
    value: chunk,
    associatedTag: 'image',
    offset: i,
  }).sendAndConfirm(umi)
}
```

{% /totem %}
{% /dialect %}

{% dialect title="Bash" id="bash" %}
{% totem %}

```bash
pnpm cli inscribe -r <RPC_ENDPOINT> -k <KEYPAIR_FILE> -m <NFT_ADDRESS>

```

{% /totem %}
{% /dialect %}

{% /dialect-switcher %}

### Inscriptions as a Storage Provider

In addition to the usage with NFT Mints Inscriptions can also be used to store arbitrary data up to 10 MB on-chain. An unlimited number of [Associated Inscriptions](inscription/associatedInscriptions) can be created.

This can be useful when writing an on-chain game that needs to store JSON data, storing text on-chain, or storing any program-related data that's not an NFT.

{% diagram height="h-64 md:h-[500px]" %}
{% node %}
{% node #inscriptionAccount1 theme="crimson" %}
Inscription Account {% .whitespace-nowrap %}
{% /node %}
{% node theme="dimmed" %}
Owner: Inscription Program {% .whitespace-nowrap %}
{% /node %}
{% /node %}

{% node parent="inscriptionAccount1" x="-40" y="160" %}
{% node #inscriptionMetadata1 theme="crimson" %}
Inscription Metadata Account {% .whitespace-nowrap %}
{% /node %}
{% node label="Owner: Inscription Program" theme="dimmed" /%}
{% /node %}

{% node parent="inscriptionMetadata1" x="500" y="0" %}
{% node #associatedInscription1 theme="crimson" %}
Associated Inscription Account {% .whitespace-nowrap %}
{% /node %}
{% node label="Owner: Inscription Program" theme="dimmed" /%}
{% /node %}

{% edge from="mint" to="inscriptionAccount1" path="straight" %}
Seeds:

"Inscription"

programId

mintAddress
{% /edge %}
{% edge from="inscriptionAccount1" to="inscriptionMetadata1" path="straight" %}
Seeds:

"Inscription"

programId

inscriptionAccount
{% /edge %}

{% edge from="inscriptionMetadata1" to="associatedInscription1" path="straight" %}
Seeds:

"Inscription"

"Association"

associationTag

inscriptionMetadataAccount

{% /edge %}

{% /diagram %}

The following example shows how to write NFT JSON data to an Inscription in three different transactions to avoid the 1280 byte transaction size limit.

{% dialect-switcher title="Find the rank of a specific NFT inscription" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
const inscriptionAccount = generateSigner(umi)

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
})

let builder = new TransactionBuilder()

builder = builder.add(
  initialize(umi, {
    inscriptionAccount,
  })
)

builder = builder.add(
  writeData(umi, {
    inscriptionAccount: inscriptionAccount.publicKey,
    inscriptionMetadataAccount,
    value: Buffer.from('{"description": "A bread! But on-chain!"'),
    associatedTag: null,
    offset: 0,
  })
)

builder = builder.add(
  writeData(umi, {
    inscriptionAccount: inscriptionAccount.publicKey,
    inscriptionMetadataAccount,
    value: Buffer.from(', "external_url":'),
    associatedTag: null,
    offset: '{"description": "A bread! But on-chain!"'.length,
  })
)

builder = builder.add(
  writeData(umi, {
    inscriptionAccount: inscriptionAccount.publicKey,
    inscriptionMetadataAccount,
    value: Buffer.from(' "https://breadheads.io"}'),
    associatedTag: null,
    offset: '{"description": "A bread! But on-chain!", "external_url":'.length,
  })
)

await builder.sendAndConfirm(umi, { confirm: { commitment: 'finalized' } })
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Associated Inscription Accounts

The [Metaplex JSON standards](/token-metadata/token-standard) include the option of linking associated files to a token via the files properties in the JSON schemas. The Inscription program introduces a new method of associating additional data using the power of PDAs! A PDA is derived from the Inscription and an **Association Tag**, resulting in a programmatic way to derive additional inscribed data, rather that requiring expensive JSON deserialization and parsing.

## Inscription Gateway

Together with the [Inscription Gateway](https://github.com/metaplex-foundation/inscription-gateway) you can use the normal Token Metadata Standard and just point the URI to the gateway which again reads your data directly from chain without all tools like wallets and explorers reading the data have to read it any differently than NFTs are read usually.

You can either use the gateway that is hosted by Metaplex using the following URL structure: `https://igw.metaplex.com/<network>/<account>`, e.g. [https://igw.metaplex.com/devnet/Fgf4Wn3wjVcLWp5XnMQ4t4Gpaaq2iRbc2cmtXjrQd5hF](https://igw.metaplex.com/devnet/Fgf4Wn3wjVcLWp5XnMQ4t4Gpaaq2iRbc2cmtXjrQd5hF) or host the gateway yourself with a custom URL.

## Inscription Rank

The Inscription Rank is the unique number of each inscription. This number represents a sequential, global ranking of all Metaplex Inscriptions in existence based on the total Inscription count at the time of creation. Inscription Rank is managed through a parallelized counter that is explained further in [Inscription Sharding](/inscription/sharding).

To find the `inscriptionRank` of your Inscription you need to fetch the `inscriptionMetadata` Account and read the `inscriptionRank` `bigint`:

{% dialect-switcher title="Find the rank of a specific NFT inscription" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
const inscriptionAccount = await findMintInscriptionPda(umi, {
  mint: mint.publicKey,
})
const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount,
})

const { inscriptionRank } = await fetchInscriptionMetadata(
  umi,
  inscriptionMetadataAccount
)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

When creating your inscriptions you should always use a random shard to avoid write locks. You can just calculate the random number like this:

{% dialect-switcher title="Find random shard" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
const randomShard = Math.floor(Math.random() * 32)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

The total amount of Metaplex Inscriptions on Solana can be calculated like this:

{% dialect-switcher title="Fetch total Inscription amount" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import {
  fetchAllInscriptionShard,
  findInscriptionShardPda,
} from '@metaplex-foundation/mpl-inscription'

const shardKeys = []
for (let shardNumber = 0; shardNumber < 32; shardNumber += 1) {
  k.push(findInscriptionShardPda(umi, { shardNumber }))
}

const shards = await fetchAllInscriptionShard(umi, shardKeys)
let numInscriptions = 0
shards.forEach((shard) => {
  const rank = 32 * Number(shard.count) + shard.shardNumber
  numInscriptions = Math.max(numInscriptions, rank)
})
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## And a lot more

Whilst this provides a good overview of the Inscription program and what it has to offer, there’s still a lot more that can be done with it.

The other pages of this documentation aim to document it further and explain significant features in their own individual pages.

- [Initialize](/inscription/initialize)
- [Write](/inscription/write)
- [Fetch](/inscription/fetch)
- [Clear](/inscription/clear)
- [close](/inscription/close)
- [Authorities](/inscription/authority)
- [Inscription Gateway](/inscription/gateway)
