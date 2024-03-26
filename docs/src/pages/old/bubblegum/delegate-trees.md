---
title: Delegating Trees
metaTitle: Bubblegum - Delegating Trees
description: Learn how to delegate Merkle Trees on Bubblegum
---

Similarly to how the owner of a Compressed NFT can approve a Delegate Authority, the creator of a Bubblegum Tree can also approve another account to perform actions on their behalf. {% .lead %}

Once a Delegate Authority is approved for a Bubblegum Tree, it will be able to [mint Compressed NFTs](/bubblegum/mint-cnfts) on behalf of the creator. Note that this is only relevant for private trees since anyone can mint on public trees.

## Approving a Delegate Authority for a Tree

To approve a new Delegate Authority on a Bubblegum Tree, its creator may use the **Set Tree Delegate** instruction which accepts the following parameters:

- **Merkle Tree**: The address of the Merkle Tree to delegate.
- **Tree Creator**: The creator of the Merkle Tree as a Signer.
- **New Tree Delegate**: The new Delegate Authority to approve.

{% dialect-switcher title="Delegate a Bubblegum Tree" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { setTreeDelegate } from '@metaplex-foundation/mpl-bubblegum'

await setTreeDelegate(umi, {
  merkleTree,
  treeCreator,
  newTreeDelegate,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Revoking a Delegate Authority for a Tree

To revoke an existing Delegate Authority, the creator of the tree simply needs to set themselves as the new Delegate Authority.

{% dialect-switcher title="Revoke the Delegate Authority of a Bubblegum Tree" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { setTreeDelegate } from '@metaplex-foundation/mpl-bubblegum'

await setTreeDelegate(umi, {
  merkleTree,
  treeCreator,
  newTreeDelegate: treeCreator.publicKey,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
