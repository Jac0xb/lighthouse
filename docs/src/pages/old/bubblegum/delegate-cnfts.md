---
title: Delegating Compressed NFTs
metaTitle: Bubblegum - Delegating Compressed NFTs
description: Learn how to delegate compressed NFTs on Bubblegum
---

The owner of a Compressed NFT can delegate it to another account whilst keeping ownership of the cNFT. {% .lead %}

This allows the delegated account — which we also refer to as the **Delegate Authority** — to perform actions on behalf of the owner. These actions are:

- [Transferring the cNFT](/bubblegum/transfer-cnfts). The Delegate Authority will be reset — i.e. set to the new owner — after the transfer.
- [Burning the cNFT](/bubblegum/burn-cnfts).

Each of these actions provides examples of how to use the Delegate Authority to perform them, but usually, it is simply the case of providing the **Leaf Delegate** account as a Signer instead of passing the **Leaf Owner** account as a Signer.

Let's see how we can approve and revoke Delegate Authorities for our Compressed NFTs.

## Approving a Delegate Authority

To approve or replace a Delegate Authority, the owner must send a **Delegate** instruction. This instruction accepts the following parameters:

- **Leaf Owner**: The current owner of the Compressed NFT as a Signer.
- **Previous Leaf Delegate**: The previous Delegate Authority, if any. Otherwise, this should be set to the **Leaf Owner**.
- **New Leaf Delegate**: The new Delegate Authority to approve.

Additionally, more parameters must be provided to verify the integrity of the Compressed NFT since this instruction will end up replacing the leaf on the Bubblegum Tree. Since these parameters are common to all instructions that mutate leaves, they are documented [in the following FAQ](/bubblegum/faq#replace-leaf-instruction-arguments). Fortunately, we can use a helper method that will automatically fetch these parameters for us using the Metaplex DAS API.

{% dialect-switcher title="Delegate a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { getAssetWithProof, delegate } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await delegate(umi, {
  ...assetWithProof,
  leafOwner,
  previousLeafDelegate: leafOwner.publicKey,
  newLeafDelegate: newDelegate,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Revoking a Delegate Authority

To revoke an existing Delegate Authority, the owner simply needs to set themselves as the new Delegate Authority.

{% dialect-switcher title="Revoke the Delegate Authority of a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { getAssetWithProof, delegate } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await delegate(umi, {
  ...assetWithProof,
  leafOwner,
  previousLeafDelegate: currentDelegate,
  newLeafDelegate: leafOwner.publicKey,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
