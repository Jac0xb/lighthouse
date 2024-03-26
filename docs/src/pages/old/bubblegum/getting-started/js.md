---
title: Getting Started using JavaScript
metaTitle: Bubblegum - Getting Started - JavaScript
description: Get started with compressed NFTs using JavaScript
---

Metaplex provides a JavaScript library that can be used to interact with Compressed NFTs. Thanks to the [Umi framework](https://github.com/metaplex-foundation/umi), it ships without many opinionated dependencies and, thus, provides a lightweight library that can be used in any JavaScript project.

To get started, you'll need to [install the Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md) and the Bubblegum JavaScript library.

```sh
npm install \
  @metaplex-foundation/umi \
  @metaplex-foundation/umi-bundle-defaults \
  @solana/web3.js \
  @metaplex-foundation/mpl-bubblegum
```

Next, you may create your `Umi` instance and install the `mplBubblegum` plugin like so.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplBubblegum } from '@metaplex-foundation/mpl-bubblegum'

// Use the RPC endpoint of your choice.
const umi = createUmi('http://127.0.0.1:8899').use(mplBubblegum())
```

That's it, you can now interact with Compressed NFTs by using [the various functions provided by the library](https://mpl-bubblegum-js-docs.vercel.app/) and passing your `Umi` instance to them. Here's an example of fetching a Merkle Tree account and its Bubblegum configurations.

```ts
import { publicKey } from '@metaplex-foundation/umi'
import {
  fetchMerkleTree,
  fetchTreeConfigFromSeeds,
} from '@metaplex-foundation/mpl-bubblegum'

const merkleTreeAddress = publicKey('...')
const merkleTree = await fetchMerkleTree(umi, merkleTreeAddress)
const treeConfig = await fetchTreeConfigFromSeeds(umi, {
  merkleTree: merkleTreeAddress,
})
```

🔗 **Helpful Links:**

- [Umi Framework](https://github.com/metaplex-foundation/umi)
- [GitHub Repository](https://github.com/metaplex-foundation/mpl-bubblegum)
- [NPM Package](https://www.npmjs.com/package/@metaplex-foundation/mpl-bubblegum)
- [API References](https://mpl-bubblegum-js-docs.vercel.app/)
