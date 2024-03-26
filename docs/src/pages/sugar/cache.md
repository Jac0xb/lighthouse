---
title: Cache File
metaTitle: Candy Machine - Sugar - Cache File
description: Sugar cache file.
---

Sugar keeps track of the Candy Machine and assets created using a cache file. This allows Sugar to resume the upload of assets without having the re-upload all assets. It also provides information regarding the Candy Machine account, such the collection and Candy Machine creator.

You should not need to manually modify the cache file directly – this file is manipulated by Sugar commands. There are specific circumstances that you might to so, as discussed above.

{% callout %}

Keep a copy of your cache file as it containts all asset information and addresses of the accounts created.

{% /callout %}

## Structure

The cache file is a JSON document with the following structure:

```json
{
  "program": {
    "candyMachine": "<PUBLIC KEY>",
    "candyGuard": "<PUBLIC KEY>",
    "candyMachineCreator": "<PUBLIC KEY>",
    "collectionMint": "<PUBLIC KEY>"
  },
  "items": {
    "-1": {
      "name": "My Collection",
      "image_hash": "6500707cb13044b7d133abb5ad68e0af660b154499229af49419c86a251a2b4d",
      "image_link": "https://arweave.net/KplI7R59EE24-mavSgai7WVJmkfvYQKhtTnqxXPlPdE?ext=png",
      "metadata_hash": "2009eda578d1196356abcfdfbba252ec3318fc6ffe42cc764a624b0c791d8471",
      "metadata_link": "https://arweave.net/K75J8IG1HcTYJyr1eC0KksYfpxuFMkPONJMpUNDmCuA",
      "onChain": true
    },
    "0": {
      "name": "My First NFT #1",
      "image_hash": "209a200ebea39be9e9e7882da2bc5e652fb690e612abecb094dc13e06db84e54",
      "image_link": "https://arweave.net/-qSoAFO7GWTm_js1eHDyoljgB3D_vszlXspVXBM7HyA?ext=png",
      "metadata_hash": "cfc45ba94da81c8d21f763ce8bb6bbb845ad598e23e44d5c8db1590672b7653f",
      "metadata_link": "https://arweave.net/6DRibEPNjLQKA90v3qa-JsYPPT5a6--VsgKumUnX3_0",
      "onChain": true
    },
    ...
  }
}
```

### `program`

The `"program"` section includes the information about the Candy Machine, Candy Guard accounts as well as the addresses of the Candy Machine creator and collection mint. These details are populated once the Candy Machine is deployed. The Candy Guard address is present only if you have enabled guards on your candy machine.

### `items`

The `"items"` section includes the information about the assets of the Candy Machine. This list is created once Sugar validates your assets folder. At this point, all the `"name"`, `"image_hash"` and `"metadata_hash"` are added to the cache file. Once the assets are uploaded, the information of the `"image_link"` and `"metadata_link"` are updated with their final values. Finally, once the Candy Machine is deployed, the `"onChain"` value is set to `true`.

Sugar `upload` will only upload assets that do not have the correspondent "link" value populated – e.g., running `sugar upload` with a cache file containing the following item:

```json
"0": {
      "name": "My First NFT #1",
      "image_hash": "209a200ebea39be9e9e7882da2bc5e652fb690e612abecb094dc13e06db84e54",
      "image_link": "https://arweave.net/-qSoAFO7GWTm_js1eHDyoljgB3D_vszlXspVXBM7HyA?ext=png",
      "metadata_hash": "cfc45ba94da81c8d21f763ce8bb6bbb845ad598e23e44d5c8db1590672b7653f",
      "metadata_link": "",
      "onChain": false
},
```

only the metadata file will be uploaded, since the image link is alredy present.

Sugar stores the "hash" of both image and metadata files, so when the hash value changes as a result of chaging the corresponding file, running `sugar upload` will upload the new file. At this point, the `"onChain"` value will be set to `false` and the change will only be effective (be on-chain) after running `sugar deploy`.

## "Advance" cache management

In most cases, you don't need to modify the cache file manually. But there are cases when you might want to do so.

### Deploying a new Candy Machine with the same items

If you want to deploy your Candy Machine to a new address, reusing the same items from the cache file, you can simply remove the `"candyMachine"` public key value from the cache file:

{% totem %}
{% totem-accordion title="Example" %}

```json
{
  "program": {
    "candyMachine": "",
    "candyGuard": "",
    "candyMachineCreator": "6DwuXCUnGEE2NktwQub22Ejt2EQUexGmGADZURN1RF6J",
    "collectionMint": "5TM8a74oX6HgyAtVnKaUaGuwu44hxMhWF5QT5i7PkuZY"
  },
  "items": {
    "-1": {
      "name": "My Collection",
      "image_hash": "6500707cb13044b7d133abb5ad68e0af660b154499229af49419c86a251a2b4d",
      "image_link": "https://arweave.net/KplI7R59EE24-mavSgai7WVJmkfvYQKhtTnqxXPlPdE?ext=png",
      "metadata_hash": "2009eda578d1196356abcfdfbba252ec3318fc6ffe42cc764a624b0c791d8471",
      "metadata_link": "https://arweave.net/K75J8IG1HcTYJyr1eC0KksYfpxuFMkPONJMpUNDmCuA",
      "onChain": true
    },
    "0": {
      "name": "My First NFT #1",
      "image_hash": "209a200ebea39be9e9e7882da2bc5e652fb690e612abecb094dc13e06db84e54",
      "image_link": "https://arweave.net/-qSoAFO7GWTm_js1eHDyoljgB3D_vszlXspVXBM7HyA?ext=png",
      "metadata_hash": "cfc45ba94da81c8d21f763ce8bb6bbb845ad598e23e44d5c8db1590672b7653f",
      "metadata_link": "https://arweave.net/6DRibEPNjLQKA90v3qa-JsYPPT5a6--VsgKumUnX3_0",
      "onChain": true
    },
    ...
  }
}
```

{% /totem-accordion %}
{% /totem %}

### Using pre-existing links

When you already have links to your assets, the information can be added to the cache file manually to avoid Sugar uploading them again. In this case, you should complete the `"image_link"` and `"metadata_link"` with the corresponding links.