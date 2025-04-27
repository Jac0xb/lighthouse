---
title: Getting Started
metaTitle: Lighthouse - Getting Started
description: How to Lighthouse.
---

## Installing

Lighthouse instructions can be built using either the rust or javascript sdk. For the rust sdk, you can install the sdk using cargo:

```bash
$ cargo install lighthouse-sdk
```

For the @solana/kit version of the javascript sdk, you can install the sdk using npm:

```bash
$ npm install lighthouse-sdk
```

For the web3.js legacy version of the javascript sdk, you can install the sdk using npm:

```bash
$ npm install lighthouse-sdk-legacy
```

For the legacy version you'll need to construct a umi context like so:

```typescript
import { createLighthouseProgram } from 'lighthouse-sdk-legacy'
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'

function main() {
  const umi = createUmi('https://api.mainnet-beta.solana.com')
  umi.programs.add(createLighthouseProgram())
}
```

{% callout %}
If you'd like to minimize umi usage you can use the conversion functions in `@metaplex-foundation/umi-web3js-adapters` to convert back and forth between umi and web3.js objects like `toWeb3JsTransaction`, `toWeb3JsInstruction`, and `fromWeb3JsPublicKey`.
{% /callout %}

## Usage

For examples of how to build all the assertion instructions see [here](/assert).
