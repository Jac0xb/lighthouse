---
title: Getting Started using JavaScript
metaTitle: Umi - Getting Started - JavaScript
description: Get started with essential programs using JavaScript
---

Metaplex provides a JavaScript library that can be used to interact with essential programs. Thanks to the [Umi framework](https://github.com/metaplex-foundation/umi), it ships without many opinionated dependencies and, thus, provides a lightweight library that can be used in any JavaScript project.

To get started, you'll need to [install the Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md) and the Toolbox JavaScript library.

```sh
npm install \
  @metaplex-foundation/umi \
  @metaplex-foundation/umi-bundle-defaults \
  @solana/web3.js \
  @metaplex-foundation/mpl-toolbox
```

Next, you may create your `Umi` instance and install the `mplToolbox` plugin like so.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplToolbox } from '@metaplex-foundation/mpl-toolbox'

// Use the RPC endpoint of your choice.
const umi = createUmi('http://127.0.0.1:8899').use(mplToolbox())
```

That's it, you can now interact with essential programs by using [the various functions provided by the library](https://mpl-toolbox-js-docs.vercel.app/) and passing your `Umi` instance to them. Here's an example of creating a new associated token account.

```ts
import { createAssociatedToken } from '@metaplex-foundation/mpl-toolbox'

await createAssociatedToken(umi, { mint, owner }).sendAndConfirm(umi)
```

🔗 **Helpful Links:**

- [Umi Framework](https://github.com/metaplex-foundation/umi)
- [GitHub Repository](https://github.com/metaplex-foundation/mpl-toolbox)
- [NPM Package](https://www.npmjs.com/package/@metaplex-foundation/mpl-toolbox)
- [API References](https://mpl-toolbox-js-docs.vercel.app/)
