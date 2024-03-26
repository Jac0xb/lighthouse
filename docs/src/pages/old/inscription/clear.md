---
title: Clear Inscription Data
metaTitle: Inscription - Clear Data
description: Learn how to clear Inscription data
---

The update authority of an inscription can clear its data and the data of associated inscriptions using the **ClearData** instruction as long as inscription has not been [engraved](/engraver/). The **ClearData** instruction requires one of the the **Authorites** to sign the transaction.

Clearing the data removes all existing data resizes the inscription account to 0.

Here is how you can use our SDKs to clear inscription data.

{% dialect-switcher title="Clear Inscription Data" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts

import { clearData, findInscriptionMetadataPda } from '@metaplex-foundation/mpl-inscription'

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
})

await clearData(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
  inscriptionMetadataAccount,
  associatedTag: null, //use the same tag here as you used on creation
})
```

The `associatedTag` is used to derive the associated inscription account correctly.

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
