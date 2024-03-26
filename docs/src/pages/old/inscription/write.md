---
title: Write Inscription Data
metaTitle: Inscription - Write Inscription Data
description: Learn how to write Data to your Inscription
---

After [initializing](initialize) an inscription account data can be written to it. This is also the case for associated inscriptions.

{% dialect-switcher title="Write Inscription Data" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import { writeData } from '@metaplex-foundation/mpl-inscription';

await writeData(umi, {
  inscriptionAccount: inscriptionAccount.publicKey,
  inscriptionMetadataAccount,
  authority,
  value: Buffer.from(
    '{"description": "A bread! But on-chain!", "external_url": "https://breadheads.io"}'
  ),
  associatedTag: null,
  offset: 0,
})
```
{% /totem %}
{% /dialect %}
{% /dialect-switcher %}


For larger data it is recommended to first `allocate` the required space, wait for that transaction to finalize and then `writeData`. The following example allocates data in a associated Inscription account:

{% dialect-switcher title="Allocate space" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import { allocate } from '@metaplex-foundation/mpl-inscription';
const fs = require('fs');

// Open the image file to fetch the raw bytes.
const imageBytes: Buffer = await fs.promises.readFile('test/large_bread.png')
const resizes = Math.floor(imageBytes.length / 10240) + 1
for (let i = 0; i < resizes; i += 1) {
  await allocate(umi, {
    inscriptionAccount: associatedInscriptionAccount,
    inscriptionMetadataAccount,
    associatedTag: 'image/png',
    targetSize: imageBytes.length,
  }).sendAndConfirm(umi)
}
```
{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
