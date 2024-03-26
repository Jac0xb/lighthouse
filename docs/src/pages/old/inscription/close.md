---
title: Close Inscriptions
metaTitle: Inscription - Close
description: Learn how to close Inscription accounts
---

The Authority of an inscription can close inscription accounts. This will close all possible accounts associated with the inscription and return the various rent-exempt fees to the owner. Think of `close` being similar to `burn` for tokens.

To close the inscription account you have to be a **authority**. The inscription can not have associated Inscriptions at that point in time.

Here is how you can use our SDKs to close inscription accounts.

{% dialect-switcher title="Close Inscription Data" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { close, findInscriptionMetadataPda } from '@metaplex-foundation/mpl-inscription';

import { close, findInscriptionMetadataPda } from '@metaplex-foundation/mpl-inscription'

const inscriptionMetadataAccount = await findInscriptionMetadataPda(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
})

await close(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
  inscriptionMetadataAccount,
})
```
{% /totem %}
{% /dialect %}

{% /dialect-switcher %}
