---
title: Get Asset
metaTitle: DAS API - Get Asset
description: Returns the information of a compressed/standard asset
---

Returns the information of a compressed/standard asset including metadata and owner.

## Parameters

| Name            | Required | Description                                |
| --------------- | :------: | ------------------------------------------ |
| `id`            |    ✅    | The id of the asset.                       |

## Example

{% dialect-switcher title="getAsset Example" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```js
import { publicKey } from '@metaplex-foundation/umi';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { dasApi } from '@metaplex-foundation/digital-asset-standard-api';

const umi = createUmi('<ENDPOINT>').use(dasApi());
const assetId = publicKey('8TrvJBRa6Pzb9BDadqroHhWTHxaxK8Ws8r91oZ2jxaVV');

const asset = await umi.rpc.getAsset(assetId);
console.log(asset);
```

{% /totem %}
{% /dialect %}
{% dialect title="cURL" id="curl" %}
{% totem %}

```sh
curl --request POST --url "<ENDPOINT>" --header 'Content-Type: application/json' --data '{
    "jsonrpc": "2.0",
    "method": "getAsset",
    "params": [
      "8vw7tdLGE3FBjaetsJrZAarwsbc8UESsegiLyvWXxs5A"
    ],
    "id": 0
}'
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}