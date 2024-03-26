---
title: Fetching Assets
metaTitle: Token Metadata - Fetching Assets
description: Learn how to fetch the various on-chain accounts of your assets on Token Metadata
---

Now that we know how to create and mint the various on-chain accounts of our assets, let's learn how to fetch them. {% .lead %}

## Digital Assets

As mentioned in [the previous page](/token-metadata/mint#creating-accounts), an asset — fungible or not — requires multiple on-chain accounts to be created. Depending on the Token Standard of the asset, some accounts may not be required. Here's a quick overview of these accounts:

- **Mint** account (from the SPL Token program): It defines the core properties of the underlying SPL token. This is the entry point to any asset as all other accounts derive from it.
- **Metadata** account: It provides additional data and features to the underlying SPL token.
- **Master Edition** or **Edition** account (only for Non-Fungibles): It enables printing multiple copies of an original NFT. Even when an NFT does not allow printing editions, the **Master Edition** account is still created as it is used as the Mint authority and Freeze authority of the **Mint** account to ensure its non-fungibility.

In order to make fetching assets easier, our SDKs offer a set of helper methods that allow us to fetch all the relevant accounts of an asset in one go. We call the data type that stores all these accounts a **Digital Asset**. In the next sub-sections, we will go through the various ways to fetch **Digital Assets**.

{% dialect-switcher title="Digital Asset definition" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { PublicKey } from '@metaplex-foundation/umi'
import { Mint } from '@metaplex-foundation/mpl-toolbox'
import {
  Metadata,
  MasterEdition,
  Edition,
} from '@metaplex-foundation/mpl-token-metadata'

export type DigitalAsset = {
  publicKey: PublicKey
  mint: Mint
  metadata: Metadata
  edition?:
    | ({ isOriginal: true } & MasterEdition)
    | ({ isOriginal: false } & Edition)
}
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch By Mint

This helper fetches a single **Digital Asset** from the public key of its **Mint** account.

{% dialect-switcher title="Fetch Asset by Mint" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchDigitalAsset } from '@metaplex-foundation/mpl-token-metadata'

const asset = await fetchDigitalAsset(umi, mint)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch By Metadata

This helper fetches a single **Digital Asset** from the public key of its **Metadata** account. This is slightly less efficient than the previous helper as we first need to fetch the content of the **Metadata** account to find the **Mint** address but if you only have access to the **Metadata** public key, this can be helpful.

{% dialect-switcher title="Fetch Asset by Metadata" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchDigitalAssetByMetadata } from '@metaplex-foundation/mpl-token-metadata'

const asset = await fetchDigitalAssetByMetadata(umi, metadata)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch All By Mint List

This helper fetches as many **Digital Assets** as there are **Mint** public keys in the provided list.

{% dialect-switcher title="Fetch Assets by Mint List" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchAllDigitalAsset } from '@metaplex-foundation/mpl-token-metadata'

const [assetA, assetB] = await fetchAllDigitalAsset(umi, [mintA, mintB])
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch All By Creator

This helper fetches all **Digital Assets** by creator. Since creators can be located in five different positions in the **Metadata** account, we must also provide the creator position we are interested in. For instance, if we know that for a set of NFTs, the first creator is creator A and the second creator B, we will want to search for creator A in position 1 and creator B in position 2.

{% dialect-switcher title="Fetch Assets by Creator" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchAllDigitalAssetByCreator } from '@metaplex-foundation/mpl-token-metadata'

// Assets such that the creator is first in the Creator array.
const assetsA = await fetchAllDigitalAssetByCreator(umi, creator)

// Assets such that the creator is second in the Creator array.
const assetsB = await fetchAllDigitalAssetByCreator(umi, creator, {
  position: 2,
})
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch All By Owner

This helper fetches all **Digital Assets** by owner.

{% dialect-switcher title="Fetch Assets by Owner" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchAllDigitalAssetByOwner } from '@metaplex-foundation/mpl-token-metadata'

const assets = await fetchAllDigitalAssetByOwner(umi, owner)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch All By Update Authority

This helper fetches all **Digital Assets** from the public key of their update authority.

{% dialect-switcher title="Fetch Assets by Update Authority" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchAllDigitalAssetByUpdateAuthority } from '@metaplex-foundation/mpl-token-metadata'

const assets = await fetchAllDigitalAssetByUpdateAuthority(umi, owner)
```

{% /dialect %}
{% /dialect-switcher %}

## Digital Assets With Token

Note that the **Digital Asset** data structure mentioned above does not provide any information about the owner of the asset. This first definition only focuses on the on-chain accounts that are required regardless of their owners. However, in order to provide a more complete picture of an asset, we may also need to know who owns it. This is where the **Digital Asset With Token** data structure comes in. It is an extension of the Digital Asset data structure that also includes the following accounts:

- **Token** account (from the SPL Token program): It defines the relationship between a **Mint** account and its owner. It stores important data such as the amount of tokens owned by the owner. In the case of NFTs, the amount is always 1.
- **Token Record** account (for PNFTs only): It defines additional token-related information for [Programmable Non-Fungibles](/token-metadata/pnfts) such as its current [Token Delegate](/token-metadata/delegates#token-delegates) and its role.

Note that, for fungible assets, the same Digital Asset will likely be associated with multiple owners via multiple Token accounts. Therefore, there can be multiple Digital Asset With Token for the same Digital Asset.

Here as well, we offer a set of helpers to fetch Digital Assets With Token.

{% dialect-switcher title="Digital Asset With Token definition" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { Token } from '@metaplex-foundation/mpl-toolbox'
import {
  DigitalAsset,
  TokenRecord,
} from '@metaplex-foundation/mpl-token-metadata'

export type DigitalAssetWithToken = DigitalAsset & {
  token: Token
  tokenRecord?: TokenRecord
}
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch By Mint

This helper fetches a single **Digital Asset With Token** from the public key of its **Mint** account. This is mostly relevant for Non-Fungible assets since it will only return one Digital Asset With Token, regardless of how many exist for a Fungible asset.

{% dialect-switcher title="Fetch Asset with Token By Mint" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchDigitalAssetWithTokenByMint } from '@metaplex-foundation/mpl-token-metadata'

const asset = await fetchDigitalAssetWithTokenByMint(umi, owner)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch By Mint and Owner

This helper is more performant than the previous helper but requires that we know the owner of the asset.

{% dialect-switcher title="Fetch Asset with Token By Mint" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchDigitalAssetWithAssociatedToken } from '@metaplex-foundation/mpl-token-metadata'

const asset = await fetchDigitalAssetWithAssociatedToken(umi, mint, owner)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch All By Owner

This helper fetches all **Digital Assets With Token** from a given owner.

{% dialect-switcher title="Fetch Assets with Token By Owner" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchAllDigitalAssetWithTokenByOwner } from '@metaplex-foundation/mpl-token-metadata'

const assets = await fetchAllDigitalAssetWithTokenByOwner(umi, owner)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch All By Mint

This helper fetches all **Digital Assets With Token** from the public key of a **Mint** account. This is particularly relevant for Fungible assets since it fetches all **Token** accounts.

{% dialect-switcher title="Fetch Assets with Token By Owner" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchAllDigitalAssetWithTokenByMint } from '@metaplex-foundation/mpl-token-metadata'

const assets = await fetchAllDigitalAssetWithTokenByMint(umi, owner)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetch All By Owner and Mint

This helper fetches all **Digital Assets With Token** from both an owner and a **Mint** account. This can be useful for Fungible assets that have more than one **Token** account for a given owner.

{% dialect-switcher title="Fetch Assets with Token By Mint and Owner" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchAllDigitalAssetWithTokenByOwnerAndMint } from '@metaplex-foundation/mpl-token-metadata'

const assets = await fetchAllDigitalAssetWithTokenByOwnerAndMint(
  umi,
  owner,
  mint
)
```

{% /dialect %}
{% /dialect-switcher %}
