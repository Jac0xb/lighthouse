---
title: Getting Started using JavaScript
metaTitle: Inscription - Getting Started - JavaScript
description: Get started with Inscription using JavaScript
---

Metaplex provides a JavaScript library that can be used to interact with Metaplex Inscriptions. Thanks to the [Umi framework](https://github.com/metaplex-foundation/umi), it ships without many opinionated dependencies and, thus, provides a lightweight library that can be used in any JavaScript project.

To get started, you'll need to [install the Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md) and the Inscriptions JavaScript library.

```sh
npm install \
  @metaplex-foundation/umi \
  @metaplex-foundation/umi-bundle-defaults \
  @solana/web3.js \
  @metaplex-foundation/mpl-inscription
```

Next, you may create your `Umi` instance and install the `mplInscription` plugin like so.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplInscription } from '@metaplex-foundation/mpl-inscription'

// Use the RPC endpoint of your choice.
const umi = createUmi('http://127.0.0.1:8899').use(mplInscription())
```

That's it, you can now interact with Inscriptions by using [the various functions provided by the library](https://mpl-inscription-js-docs.vercel.app/) and passing your `Umi` instance to them. Here's an example of how to mint a simple inscription with a small JSON file attached, fetching the data of the inscription and printing the inscription Rank.

```ts
// Step 1: Mint an NFT or pNFT
// See https://developers.metaplex.com/token-metadata/mint

// Step 2: Inscribe JSON

const inscriptionAccount = await findMintInscriptionPda(umi, {
  mint: mint.publicKey,
})
const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount[0],
})

await initializeFromMint(umi, {
  mintAccount: mint.publicKey,
})
  .add(
    writeData(umi, {
      inscriptionAccount,
      inscriptionMetadataAccount,
      value: Buffer.from(
        JSON.stringify(metadata) // your NFT's JSON to be inscribed
      ),
      associatedTag: null,
      offset: 0,
    })
  )
  .sendAndConfirm(umi)

const inscriptionMetadata = await fetchInscriptionMetadata(
  umi,
  inscriptionMetadataAccount
)
console.log(
  'Inscription number: ',
  inscriptionMetadata.inscriptionRank.toString()
)
```

🔗 **Helpful Links:**

- [Umi Framework](https://github.com/metaplex-foundation/umi)
- [GitHub Repository](https://github.com/metaplex-foundation/mpl-inscription)
- [NPM Package](https://www.npmjs.com/package/@metaplex-foundation/mpl-inscription)
- [API References](https://mpl-inscription-js-docs.vercel.app/)
