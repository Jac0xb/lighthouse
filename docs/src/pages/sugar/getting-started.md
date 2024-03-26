---
title: Getting Started
metaTitle: Candy Machine - Sugar - Getting Started
description: Getting started with Sugar.
---

To get started, first check that you have Sugar installed on your system:

```bash
sugar --version
```

The command above should print the Sugar version – e.g., `sugar-cli 2.5.0`.

By default, Sugar uses the keypair and RPC settings from `solana-cli`. You can check your current settings by running:

```bash
solana config get
```

And you can set different settings by running:

```bash
solana config set --url <rpc url> --keypair <path to keypair file>
```

{% callout %}

Sugar does not require `solana-cli` to be installed on the system. Every command in Sugar accept the flags `-k` (keypair) and `-r` (RPC) to configure the values to use.

{% /callout %}

## Preparing Your Files

Create a folder for your project and within it, create a folder named `assets` to store your json metadata and image file pairs with the naming convention `0.json`, `0.png`, `1.json`, `1.png`, and so on. The metadata extension is `.json` and the image files can be `.png`, `.gif`, `.jpg` and `.jpeg`. Additionally, you will need `collection.json` and `collection.png` files containing the information for your collection NFT.

Your project directory will then look like:
{% diagram %}
{% node %}
{% node #my-project label="my-project/" theme="blue" /%}
{% /node %}

{% node parent="my-project" y="50" x="100" %}
{% node #assets label="assets/" theme="indigo" /%}
{% /node %}

{% node #0-json parent="assets" y="50" x="100" label="0.json" theme="mint" /%}
{% node #0-png parent="assets" y="95" x="100" label="0.png" theme="mint" /%}
{% node #1-json parent="assets" y="140" x="100" label="1.json" theme="orange" /%}
{% node #1-png parent="assets" y="185" x="100" label="1.png" theme="orange" /%}
{% node #2-json parent="assets" y="230" x="100" label="2.json" theme="mint" /%}
{% node #2-png parent="assets" y="275" x="100" label="2.png" theme="mint" /%}
{% node #more parent="assets" y="320" x="100" label=". . ." theme="orange" /%}
{% node #collection-json parent="assets" y="365" x="100" label="collection.json" theme="purple" /%}
{% node #collection-png parent="assets" y="410" x="100" label="collection.png" theme="purple" /%}

{% edge from="my-project" to="assets" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="0-json" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="0-png" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="1-json" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="1-png" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="2-json" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="2-png" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="more" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="collection-json" fromPosition="bottom" toPosition="left" /%}
{% edge from="assets" to="collection-png" fromPosition="bottom" toPosition="left" /%}
{% /diagram %}

## Running Sugar

Within your project directory, use the `launch` command to start an interactive process of creating your config file and deploying a Candy Machine to Solana:

```bash
sugar launch
```

At the end of the execution of the launch command, a Candy Machine will be deployed on-chain. You can use the `mint` command to mint an NFT:

```bash
sugar mint
```

When all NFTs have been minted, you can close your Candy Machine and reclaim the account rent:

```bash
sugar withdraw
```

{% callout %}

The `withdraw` command will close the Candy Machine even if it is not empty, so use it with caution.

{% /callout %}
