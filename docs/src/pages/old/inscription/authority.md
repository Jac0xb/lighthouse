---
title: Inscription Authority
metaTitle: Inscriptions - Inscription Authority
description: Learn what a Inscription Authority is and where it's stored
---


Metaplex Inscriptions can have **multiple** update authorities. This is different to Metaplex NFT which can just have one update Authority plus delegates.

Authorities can be _added_ and _removed_ by each authority. An Inscription is seen as **immutable** as soon as no more update authorities exist.

## Add Authorities

Additional Authorities can be added with a simple instruction call. One of the current Authorities has to sign the transaction.

{% dialect-switcher title="Add an Authority" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import {
  addAuthority,
  findInscriptionMetadataPda,
} from '@metaplex-foundation/mpl-inscription'

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
})

await addAuthority(umi, {
  inscriptionMetadataAccount,
  newAuthority: authority.publicKey,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Remove Authority

To remove an authority there also is a instruction. `removeAuthority` allows you to remove yourself from the authority array. **Be careful**, as soon as you removed all authorities no authorities can be added anymore!

{% dialect-switcher title="Remove yourself as authority" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import {
  addAuthority,
  findInscriptionMetadataPda,
} from '@metaplex-foundation/mpl-inscription'

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
})

await removeAuthority(umi, {
  inscriptionMetadataAccount,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
