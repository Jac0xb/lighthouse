---
title: Getting Started using JavaScript
metaTitle: Candy Machine - Getting Started - JavaScript
description: Get started with Candy Machines using JavaScript
---

Metaplex provides a JavaScript library that can be used to interact with Candy Machines. Thanks to the [Umi framework](https://github.com/metaplex-foundation/umi), it ships without many opinionated dependencies and, thus, provides a lightweight library that can be used in any JavaScript project.

To get started, you'll need to [install the Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md) and the Candy Machine JavaScript library.

```sh
npm install \
  @metaplex-foundation/umi \
  @metaplex-foundation/umi-bundle-defaults \
  @solana/web3.js \
  @metaplex-foundation/mpl-candy-machine
```

Next, you may create your `Umi` instance and install the `mplCandyMachine` plugin like so.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplCandyMachine } from '@metaplex-foundation/mpl-candy-machine'

// Use the RPC endpoint of your choice.
const umi = createUmi('http://127.0.0.1:8899').use(mplCandyMachine())
```

That's it, you can now interact with NFTs by using [the various functions provided by the library](https://mpl-candy-machine-js-docs.vercel.app/) and passing your `Umi` instance to them. Here's an example of fetching a candy machine account and its associated candy guard account.

```ts
import { publicKey } from '@metaplex-foundation/umi'
import {
  fetchCandyMachine,
  fetchCandyGuard,
} from '@metaplex-foundation/mpl-candy-machine'

const candyMachinePublicKey = publicKey('...')
const candyMachine = await fetchCandyMachine(umi, candyMachinePublicKey)
const candyGuard = await fetchCandyGuard(umi, candyMachine.mintAuthority)
```

🔗 **Helpful Links:**

- [Umi Framework](https://github.com/metaplex-foundation/umi)
- [GitHub Repository](https://github.com/metaplex-foundation/mpl-candy-machine)
- [NPM Package](https://www.npmjs.com/package/@metaplex-foundation/mpl-candy-machine)
- [API References](https://mpl-candy-machine-js-docs.vercel.app/)
