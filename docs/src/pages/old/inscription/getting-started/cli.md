---
title: Getting started using the Inscriptions CLI
metaTitle: Inscription - Getting Started - CLI
description: Get started using the Inscriptions CLI
---

## Set up your workspace

Clone the [mpl-inscription repo](https://github.com/metaplex-foundation/mpl-inscription/).

```bash
git clone https://github.com/metaplex-foundation/mpl-inscription.git
```

The CLI lives in the repo in the `clients/cli` subdirectory. The dependencies must first be installed before it can be run.

```bash
pnpm install
```

After that bulk Inscribing can be invoked using the following commands. Commands that are optional are indicated

## Download the NFTs

This command is used for initializing the assets that will be inscribed. The Download process will create a cache folder in the running directory and store the JSON (.json) and Media (.png, .jpg, .jpeg) files associated with the NFT there, along with a .metadata file which stores data for other CLI commands. The name of each file will be the mint address of the NFT being inscribed.

If you wish to manually override any of the JSON or media files being inscribed, replace the relevant file in the cache directory with the file you'd like to inscribe instead.

{% dialect-switcher title="Download your NFT assets." %}
{% dialect title="Bash (Hashlist)" id="bash" %}
{% totem %}

```bash
pnpm cli download hashlist -r <RPC_URL> -k <KEYPAIR_FILE> -h <HASHLIST_FILE>
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Estimate cost (Optional)

The total cost of inscribing an NFT can be determined using this command. It calculates the SOL rent costs for inscribing NFTs based on the account overhead and the file sizes in the cache directory.

{% dialect-switcher title="Estimate total NFT Inscription cost." %}
{% dialect title="Bash (Hashlist)" id="bash" %}
{% totem %}

```bash
pnpm cli cost hashlist -h <HASHLIST_FILE>
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Trim JSON fields (Optional)

This command can be used to trim JSON fields from the .json file associated with an NFT. Oftentimes NFT JSON data includes deprecated fields that can be removed for cost savings during the Inscription process. For example the 'seller_fee_basis_points', 'creators', and 'collection' fields are all deprecated in the JSON data and can be removed to save on rent cost. Additionally, the description field is often long and creators may want to remove this for cost savings. The default fields to be removed if the `--remove` option isn't provided are 'symbol', 'description', 'seller_fee_basis_points', and 'collection'.

{% dialect-switcher title="Trim JSON fields." %}
{% dialect title="Bash (Hashlist)" id="bash" %}
{% totem %}

```bash
pnpm cli compress json --fields symbol
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Compress Images (Optional)

The CLI also offers the ability to compress images before inscribing, to further save on rent cost. They can be compressed on three metrics:

- Quality (number 1-100, default: 80) (only applicable for jpegs) which reduces the overall clarity and colors available in the image.
- Size (number 1-100, default: 100) - Reducing the total image size with lower numbers being smaller images.
- Extension (png or jpg, default: jpg) - Change the image to the specified file type, with jpegs typically being smaller (but lossier) than pngs.

{% dialect-switcher title="Compress Images." %}
{% dialect title="Bash (Hashlist)" id="bash" %}
{% totem %}

```bash
pnpm cli compress images -q <QUALITY> -s <SIZE> -e <EXTENSION>
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Inscribe!

{% dialect-switcher title="Download your NFT assets." %}
{% dialect title="Bash (Hashlist)" id="bash" %}
{% totem %}

```bash
pnpm cli inscribe hashlist -r <RPC_URL> -k <KEYPAIR_FILE> -h <HASHLIST_FILE>
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
