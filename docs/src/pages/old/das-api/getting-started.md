---
title: Getting Started
metaTitle: DAS API - Getting Started
description: Installation and setup of the Metaplex DAS API client.
---

The `@metaplex-foundation/digital-asset-standard-api` package can be use to interact with Metaplex DAS API:

The DAS API client is a Umi plugin so you will have to install Umi in conjunction with the DAS API client.

You can install the plugin from the location below.

```js
npm install @metaplex-foundation/digital-asset-standard-api
```

Once installed you can register the library with your Umi instance.

```js
import { dasApi } from "@metaplex-foundation/digital-asset-standard-api"

const umi = createUmi("exampleDasProvider.com").use(dasApi());
```

The plugin can be used with any RPC that supports the Metaplex DAS API specification – RPCs that support the specification can be found on the [RPC Providers page](/rpc-providers).

Note You might need to contact your RPC provider to "enable" the DAS API on your endpoint.