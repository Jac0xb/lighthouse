---
title: CLI Commands
metaTitle: Amman - CLI Commands
description: CLI Commands of the Metaplex Amman local validator toolkit.
---

```sh
amman [command]

Commands:
  amman start    Launches a solana-test-validator and the amman relay and/or
                 mock storage if so configured
  amman stop     Stops the relay and storage and kills the running solana
                 test validator
  amman logs     Launches 'solana logs' and pipes them through a prettifier
  amman airdrop  Airdrops provided Sol to the payer
  amman label    Adds labels for accounts or transactions to amman
  amman account  Retrieves account information for a PublicKey or a label or
                 shows all labeled accounts
  amman run      Executes the provided command after expanding all address
                 labels

Options:
  --help     Show help                                                 [boolean]
  --version  Show version number                                       [boolean]
```

## Running Commands

```sh
npx amman start <config.js>
```

If no `config.js` is provided _amman_ looks for an `.ammanrc.js` file in the current directory.
If that isn't found either it uses a default config.

If you added Amman into your package.json scripts you can respectively run Amman from your package installer of choice.

```sh
// npm
npm run amman:start

// yarn
yarn amman:start

// pnpm
pnpm run amman:start
```
