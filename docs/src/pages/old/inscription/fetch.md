---

title: Fetching Inscription Data
metaTitle: Inscriptions - Fetching Inscriptions
description: Learn how to fetch the various on-chain accounts of your inscriptions
---


Once Inscription Accounts are [initialized](initialize) their Metadata can be read from chain again. Once data is [written](write) it can also be read. To fetch inscriptions you also have to use different functions according to the inscription type.

## Fetch inscription Metadata

Both inscription types use a metadata account. This Account contains for example the `inscriptionRank`, `associatedInscriptions`, `updateAuthorities` and [more](https://mpl-inscription-js-docs.vercel.app/types/InscriptionMetadata.html). The Metadata can be fetched like so:

{% dialect-switcher title="Fetch Inscription Metadata" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { safeFetchInscriptionMetadataFromSeeds } from '@metaplex-foundation/mpl-inscription'

const inscriptionMetadataAccount = await safeFetchInscriptionMetadataFromSeeds(
  umi,
  {
    inscriptionAccount: inscriptionAccount.publicKey,
  }
)

console.log(inscriptionMetadataAccount)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Fetch mint inscription

To fetch the deserialized mint inscription you can use `safeFetchMintInscriptionFromSeeds` like so:

{% dialect-switcher title="Fetch Mint Inscription" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { fetchInscription, safeFetchMintInscriptionFromSeeds, safeFetchInscriptionMetadataFromSeeds } from '@metaplex-foundation/mpl-inscription'

const mintInscription = await safeFetchMintInscriptionFromSeeds(umi, {
  mint,
})

const inscriptionMetadataAccount = await safeFetchInscriptionMetadataFromSeeds(
  umi,
  {
    inscriptionAccount: inscriptionAccount.publicKey,
  }
)

const associatedInscriptionAccount = findAssociatedInscriptionPda(umi, {
  associated_tag: inscriptionMetadataAccount.associatedInscriptions[0].tag,
  inscriptionMetadataAccount.publicKey,
})
const imageData = await fetchInscription(umi, associatedInscriptionAccount[0])
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Fetch data inscription

To read Inscription Data that is not attached to NFTs a different function is used:

{% dialect-switcher title="Fetch Inscription" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}
```js
import { fetchInscription } from '@metaplex-foundation/mpl-inscription'

const inscription = fetchInscription(umi, inscriptionAddress)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Fetch current Inscription count
The current total inscription count can be fetched like so:
 
{% dialect-switcher title="Fetch current Inscription count" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  fetchAllInscriptionShard,
  findInscriptionShardPda,
} from '@metaplex-foundation/mpl-inscription'

const shardKeys: Pda[]
for (let shardNumber = 0; shardNumber < 32; shardNumber += 1) {
  shardKeys.push(findInscriptionShardPda(umi, { shardNumber }))
}

const shards = await fetchAllInscriptionShard(umi, shardKeys)
let numInscriptions = 0
shards.forEach((shard) => {
  const rank = 32 * Number(shard.count) + shard.shardNumber
  numInscriptions = Math.max(numInscriptions, rank)
})

console.log(`Currently there are ${numInscriptions} Metaplex Inscriptions`)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
