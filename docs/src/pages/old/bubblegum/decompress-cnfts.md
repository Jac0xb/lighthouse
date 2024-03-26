---
title: Decompressing Compressed NFTs
metaTitle: Bubblegum - Decompressing Compressed NFTs
description: Learn how to redeem and decompress compressed NFTs on Bubblegum
---

It is possible for the owner of a Compressed NFT to decompress it into a regular NFT. {% .lead %}

This means on-chain accounts such as the Mint account, the Metadata account and the Master Edition account will be created for the NFT. This enables the NFT to perform certain operations that cannot be done with Compressed NFTs, interact with platforms that do not support Compressed NFTs and increase its interoperability with the NFT ecosystem in general.

## The decompression process

Decompressing a Compressed NFT is a two-step process initiated by the owner of the NFT.

1. First, the owner must **Redeem** the Compressed NFT for a Voucher. This will remove the leaf from the Bubblegum tree and create a Voucher account that acts as proof that the leaf once existed on the tree.

2. Then, the owner must **Decompress** the Voucher into a regular NFT. At this point, all accounts of the regular NFT will be created with the same data as the Compressed NFT. Alternatively, the owner can revert the process by using the **Cancel Redeem** instruction which will restore the leaf on the Bubblegum tree and close the Voucher account. Note that once the cNFT is fully decompressed, the **Cancel Redeem** instruction can no longer be used and therefore the process can no longer be reverted.

{% diagram %}

{% node #merkle-tree-wrapper %}
{% node #merkle-tree label="Merkle Tree Account" theme="blue" /%}
{% node label="Owner: Account Compression Program" theme="dimmed" /%}
{% /node %}

{% node #tree-config-pda parent="merkle-tree" x="87" y="-60" label="PDA" theme="crimson" /%}

{% node #tree-config parent="tree-config-pda" x="-63" y="-80" %}
{% node label="Tree Config Account" theme="crimson" /%}
{% node label="Owner: Bubblegum Program" theme="dimmed" /%}
{% /node %}

{% node #voucher-wrapper parent="merkle-tree" x="350" %}
{% node #voucher label="Voucher Account" theme="crimson" /%}
{% node label="Owner: Bubblegum Program" theme="dimmed" /%}
{% /node %}

{% node parent="voucher" x="320" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}

{% node #edition-pda parent="mint" x="80" y="-100" label="PDA" theme="crimson" /%}
{% node #metadata-pda parent="mint" x="80" y="-200" label="PDA" theme="crimson" /%}

{% node parent="edition-pda" x="-250" %}
{% node #edition label="Master Edition Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% node parent="metadata-pda" x="-250" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% edge from="merkle-tree" to="tree-config-pda" path="straight" /%}
{% edge from="tree-config-pda" to="tree-config" path="straight" /%}
{% edge from="merkle-tree" to="voucher" animated=true label="1️⃣  Redeem" theme="mint" /%}
{% edge from="voucher" to="mint" animated=true label="2️⃣  Decompress" theme="mint" /%}
{% edge from="voucher-wrapper" to="merkle-tree-wrapper" animated=true label="2️⃣  Cancel Redeem" fromPosition="bottom" toPosition="bottom" theme="red" labelX=175 /%}
{% edge from="mint" to="edition-pda" fromPosition="right" toPosition="right" /%}
{% edge from="mint" to="metadata-pda" fromPosition="right" toPosition="right" /%}
{% edge from="edition-pda" to="edition" path="straight" /%}
{% edge from="metadata-pda" to="metadata" path="straight" /%}

{% /diagram %}

## Redeeming a Compressed NFT

To initiate the first step of the decompression process, the owner of the Compressed NFT must send a **Redeem** instruction and sign the transaction. This will create a Voucher account for the cNFT that will be used in the next step of the decompression process.

Note that this instruction removes a leaf from the Bubblegum Tree. Therefore, additional parameters must be provided to verify the integrity of the Compressed NFT to remove. Since these parameters are common to all instructions that mutate leaves, they are documented [in the following FAQ](/bubblegum/faq#replace-leaf-instruction-arguments). Fortunately, we can use a helper method that will automatically fetch these parameters for us using the Metaplex DAS API.

{% dialect-switcher title="Redeem a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { getAssetWithProof, redeem } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await redeem(umi, {
  ...assetWithProof,
  leafOwner: currentLeafOwner,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Decompressing a Redeemed NFT

The finalize the decompression process, the owner of cNFT must send a **Decompress** instruction which will transform the redeemed Voucher account into a regular NFT. The following parameters must be provided:

- **Mint**: The mint address of the NFT to create. This must be the **Asset ID** of the Compressed NFT, i.e. the PDA derived the Merkle Tree address and the index of the leaf.
- **Voucher**: The address of the Voucher account that was created in the previous step. This address is also derived from the Merkle Tree address and the index of the leaf.
- **Metadata**: The metadata object that contains all of the cNFT's data. This attribute must match exactly the data of the Compressed NFT, otherwise, the hashes won't match and decompression will fail.

Here again, a helper function provided by our SDKs can be used to fetch and parse most of these attributes from the Metaplex DAS API.

{% dialect-switcher title="Decompress a Redeemed Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  findVoucherPda,
  decompressV1,
} from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await decompressV1(umi, {
  ...assetWithProof,
  leafOwner: currentLeafOwner,
  mint: assetId,
  voucher: findVoucherPda(umi, assetWithProof),
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Cancelling a Redeemed NFT

Should the owner change their mind about decompressing the cNFT, they can cancel the decompression process by sending a **Cancel Redeem** instruction. This will add the leaf back to the tree and close the Voucher account. Similarly to the **Decompress** instruction, the **Voucher** address must be provided as well as other attributes that can be retrieved using the Metaplex DAS API.

{% dialect-switcher title="Cancel the decompression a Redeemed Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  findVoucherPda,
  cancelRedeem,
} from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await cancelRedeem(umi, {
  ...assetWithProof,
  leafOwner: currentLeafOwner,
  voucher: findVoucherPda(umi, assetWithProof),
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
