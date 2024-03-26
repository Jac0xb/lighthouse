---
title: FAQ
metaTitle: Inscription - FAQ
description: Frequently asked questions about Metaplex Inscriptions
---

## What's the point of Inscriptions?

Contrary to popular belief, Inscriptions can be used for a lot more than making Validators cry. The ability to write arbitrary data on-chain has huge benefits for Solana program integration. In the beginning, the primary use case for this will be NFTs, providing a way to store all NFT data on Solana. This will enable many use cases like on-chain trait-based gating for programs, a way to store additional NFT metadata without writing a custom program (e.g. game stat blocks, NFT history, additional information, etc.), and dynamic image generation directly in Solana programs.

## Where do I inscribe?

- The [Metaplex Inscription UI](https://inscriptions.metaplex.com) is a no-code reference implementation for Inscribing existing NFTs on Solana. This UI allows creators to view all of the NFTs they have update authority over and walk them through the Inscription flow to store the NFT JSON and Images on Solana.

  {% callout type="note" %}

  Due to the limitations of browser wallets, it is not recommended to use the UI for bulk Inscriptions. Please use the CLI instead to save you hundreds of transaction approvals.

  {% /callout %}

- The [Inscription CLI](https://github.com/metaplex-foundation/mpl-inscription/tree/main/clients/cli) is a command line tool to handle bulk Inscribing of NFTs.

## How much does it cost?

Inscription cost fundamentally comes down to 0.003306 SOL overhead for account rent plus the 0.00000696 SOL / byte of space for the actual data being inscribed. Several tools exist to make calculating this cost easier:

- An [Inscription calculator](https://www.sackerberg.dev/tools/inscriptionCalculator) that allows you to put in the Image and JSON sizes to calculate total cost.
- The Inscription UI includes an advanced compression suite, allowing you to dynamically resize and compress each NFT to measure the quality x cost tradeoff.
- The Inscription CLI includes tooling to measure the total cost of bulk Inscriptions.

## How do I inscribe new NFTs?

New NFTs can be inscribed by first minting through a creation tool (recommended tools are [Creator Studio](https://studio.metaplex.com/) and [Sol Tools](https://sol-tools.io/)). After minting, these new NFTs will now be listed on the Inscription UI and via the CLI tool.
