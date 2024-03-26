---
title: Getting Started
metaTitle: Amman - Getting Started
description: Installation and setup of the Metaplex Amman local validator toolkit.
---

## Prerequisites.

Before running Amman your system will need to have a few things installed on your system.

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [NodeJs](https://nodejs.org/en/download)

## Installation

Once you have initiated a new or opened an existing project you can install Amman via a package manager.

```js
npm i @metaplex-foundation/amman
```

## Add to Scripts (optional)

For ease of use you may wish to add the execution of Amman to your package.json scripts.

{% dialect-switcher title="package.json" %}
{% dialect title="JavaScript" id="js" %}

```js
"scripts": {
    ...
    "amman:start": "npx amman start"
  },
```
{% /dialect %}
{% /dialect-switcher %}
