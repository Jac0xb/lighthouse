---
title: Getting Started using JavaScript
metaTitle: Token Metadata - Getting Started - JavaScript
description: Get started with NFTs using JavaScript
---

Metaplex provides a JavaScript library that can be used to interact with NFTs. Thanks to the [Umi framework](https://github.com/metaplex-foundation/umi), it ships without many opinionated dependencies and, thus, provides a lightweight library that can be used in any JavaScript project.

To get started, you'll need to [install the Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md) and the Token Metadata JavaScript library.

```sh
npm install \
  @metaplex-foundation/umi \
  @metaplex-foundation/umi-bundle-defaults \
  @solana/web3.js \
  @metaplex-foundation/mpl-token-metadata
```

Next, you may create your `Umi` instance and install the `mplTokenMetadata` plugin like so.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata'

// Use the RPC endpoint of your choice.
const umi = createUmi('http://127.0.0.1:8899').use(mplTokenMetadata())
```

That's it, you can now interact with NFTs by using [the various functions provided by the library](https://mpl-token-metadata-js-docs.vercel.app/) and passing your `Umi` instance to them. Here's an example of creating an NFT and fetching the data of all of its on-chain accounts.

```ts
import { generateSigner, percentAmount } from '@metaplex-foundation/umi'
import {
  createNft,
  fetchDigitalAsset,
} from '@metaplex-foundation/mpl-token-metadata'

const mint = generateSigner(umi)
await createNft(umi, {
  mint,
  name: 'My NFT',
  uri: 'https://example.com/my-nft.json',
  sellerFeeBasisPoints: percentAmount(5.5),
}).sendAndConfirm(umi)

const asset = await fetchDigitalAsset(umi, mint.publicKey)
```

🔗 **Helpful Links:**

- [Umi Framework](https://github.com/metaplex-foundation/umi)
- [GitHub Repository](https://github.com/metaplex-foundation/mpl-token-metadata)
- [NPM Package](https://www.npmjs.com/package/@metaplex-foundation/mpl-token-metadata)
- [API References](https://mpl-token-metadata-js-docs.vercel.app/)
