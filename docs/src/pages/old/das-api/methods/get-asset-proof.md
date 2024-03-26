---
title: Get Asset Proof
metaTitle: DAS API - Get Asset Proof
description: Returns the merkle tree proof information for a compressed asset
---

Returns the merkle tree proof information for a compressed asset.

## Parameters

| Name            | Required | Description                                |
| --------------- | :------: | ------------------------------------------ |
| `id`            |    ✅    | The id of the asset.                       |

## Example

{% dialect-switcher title="getAssetProof Example" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import { publicKey } from '@metaplex-foundation/umi';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { dasApi } from '@metaplex-foundation/digital-asset-standard-api';

const umi = createUmi('<ENDPOINT>').use(dasApi());
const assetId = publicKey('Ez6ezCMkRaUkWS5v6WVcP7uuCWiKadr3W2dHFkoZmteW');

const proof = await umi.rpc.getAssetProof(assetId);
console.log(proof);
```

{% /totem %}
{% /dialect %}
{% dialect title="cURL" id="curl" %}
{% totem %}

```sh
curl --request POST --url "<ENDPOINT>" --header 'Content-Type: application/json' --data '{
    "jsonrpc": "2.0",
    "method": "getAssetProof",
    "params": [
      "Ez6ezCMkRaUkWS5v6WVcP7uuCWiKadr3W2dHFkoZmteW"
    ],
    "id": 0
}'
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}