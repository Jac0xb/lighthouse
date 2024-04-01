---
title: Inserting Items
metaTitle: Candy Machine - Inserting Items
description: Explains how to load items into Candy Machines.
---

So far we’ve learnt to create and configure Candy Machines but we’ve not seen how to insert items inside them that can then be minted into NFTs. Thus, let’s tackle that on this page. {% .lead %}

It is important to remember that **inserting items only applies to Candy Machines using [Config Line Settings](/candy-machine/settings#config-line-settings)**. This is because NFTs minted from Candy Machine using [Hidden Settings](/candy-machine/settings#hidden-settings) will all share the same “hidden” name and URI.

## Uploading JSON Metadata

To insert items in a Candy Machine, you will need the following two parameters for each item:

- Its **Name**: The name of the NFT that will be minted from this item. If a Name Prefix was provided in the Config Line Settings, you must only provide the part of the name that comes after that prefix.
- Its **URI**: The URI pointing to the JSON metadata of the NFT that will be minted from this item. Here also, it excludes the URI Prefix that might have been provided in the Config Line Settings.

If you do not have URIs for your items, you’ll first need to upload their JSON metadata one by one. This can either be using an off-chain solution — such as AWS or your own server — or an on-chain solution — such as Arweave or IPFS.

Fortunately, our SDKs can help you with that. They allow you to upload a JSON object and retrieve its URI.

Additionally, tools like [Sugar](/candy-machine/sugar) make uploading JSON metadata a breeze by uploading in parallel, caching the process and retrying failed uploads.

{% dialect-switcher title="Upload items" %}
{% dialect title="JavaScript" id="js" %}

Umi ships with an `uploader` interface that can be used to upload JSON data to the storage provider of your choice. For instance, this is how you'd select the NFT.Storage implementation of the uploader interface.

```ts
import { nftStorage } from '@metaplex-foundation/umi-uploader-nft-storage'
umi.use(nftStorageUploader({ token: 'YOUR_API_TOKEN' }))
```

You may then use the `upload` and `uploadJson` methods of the `uploader` interface to upload your assets and their JSON metadata.

```ts
import { createGenericFileFromBrowserFile } from '@metaplex-foundation/umi'

// Upload the asset.
const file = await createGenericFileFromBrowserFile(event.target.files[0])
const [fileUri] = await umi.uploader.upload([file])

// Upload the JSON metadata.
const uri = await umi.uploader.uploadJson({
  name: 'My NFT #1',
  description: 'My description',
  image: fileUri,
})
```

API References: [UploaderInterface](https://umi-docs.vercel.app/interfaces/umi.UploaderInterface.html), [createGenericFileFromBrowserFile](https://umi-docs.vercel.app/functions/umi.createGenericFileFromBrowserFile.html).

{% /dialect %}
{% /dialect-switcher %}

## Inserting Items

Now that we have a name and URI for all of our items, all we need to do is insert them into our Candy Machine account.

This is an important part of the process and, when using Config Line Settings, **minting will not be permitted until all items have been inserted**.

Note that the name and URI of each inserted item are respectively constraint by the **Name Length** and **URI Length** attributes of the Config Line Settings.

Additionally, because transactions are limited to a certain size, we cannot insert thousands of items within the same transaction. The number of items we can insert per transaction will depend on the **Name Length** and **URI Length** attributes defined in the Config Line Settings. The shorter our names and URIs are, the more we'll be able to fit into a transaction.

{% dialect-switcher title="Add config lines" %}
{% dialect title="JavaScript" id="js" %}

When using the Umi library, you may use the `addConfigLines` function to insert items into a Candy Machine. It requires the config lines to add as well as the index in which you want to insert them.

```ts
await addConfigLines(umi, {
  candyMachine: candyMachine.publicKey,
  index: 0,
  configLines: [
    { name: 'My NFT #1', uri: 'https://example.com/nft1.json' },
    { name: 'My NFT #2', uri: 'https://example.com/nft2.json' },
  ],
}).sendAndConfirm(umi)
```

To simply append items to the end of the currently loaded items, you may using the `candyMachine.itemsLoaded` property as the index like so.

```ts
await addConfigLines(umi, {
  candyMachine: candyMachine.publicKey,
  index: candyMachine.itemsLoaded,
  configLines: [
    { name: 'My NFT #3', uri: 'https://example.com/nft3.json' },
    { name: 'My NFT #4', uri: 'https://example.com/nft4.json' },
    { name: 'My NFT #5', uri: 'https://example.com/nft5.json' },
  ],
}).sendAndConfirm(umi)
```

API References: [addConfigLines](https://mpl-candy-machine-js-docs.vercel.app/functions/addConfigLines.html)

{% /dialect %}
{% /dialect-switcher %}

## Inserting Items Using Prefixes

When using name and/or URI prefixes, you only need to insert the part that comes after them.

Note that, since using prefixes can significantly reduce the Name Length and URI Length, it should help you fit a lot more items per transaction.

{% dialect-switcher title="Add config lines from a given index" %}
{% dialect title="JavaScript" id="js" %}

When adding config lines to a candy machine that uses prefixes, you may only provide the part of the name and URI that comes after the prefix when using the `addConfigLines` function.

For instance, say you had a candy machine with the following config line settings.

```ts
await create(umi, {
  // ...
  configLineSettings: some({
    prefixName: 'My NFT #',
    nameLength: 4,
    prefixUri: 'https://example.com/nft',
    uriLength: 9,
    isSequential: false,
  }),
}).sendAndConfirm(umi)
```

Then, you can insert config lines like so.

```ts
await addConfigLines(umi, {
  candyMachine: candyMachine.publicKey,
  index: candyMachine.itemsLoaded,
  configLines: [
    { name: '1', uri: '1.json' },
    { name: '2', uri: '2.json' },
    { name: '3', uri: '3.json' },
  ],
}).sendAndConfirm(umi)
```

API References: [addConfigLines](https://mpl-candy-machine-js-docs.vercel.app/functions/addConfigLines.html)

{% /dialect %}
{% /dialect-switcher %}

## Overriding Existing Items

When inserting items, you may provide the position in which these items should be inserted. This enables you to insert items in any order you want but also allows you to update items that have already been inserted.

{% dialect-switcher title="Override config lines" %}
{% dialect title="JavaScript" id="js" %}

The following examples show how you can insert three items and, later on, update the second item inserted.

```ts
await addConfigLines(umi, {
  candyMachine: candyMachine.publicKey,
  index: 0,
  configLines: [
    { name: 'My NFT #1', uri: 'https://example.com/nft1.json' },
    { name: 'My NFT #2', uri: 'https://example.com/nft2.json' },
    { name: 'My NFT #3', uri: 'https://example.com/nft3.json' },
  ],
}).sendAndConfirm(umi)

await addConfigLines(umi, {
  candyMachine: candyMachine.publicKey,
  index: 1,
  configLines: [{ name: 'My NFT #X', uri: 'https://example.com/nftX.json' }],
}).sendAndConfirm(umi)

candyMachine = await fetchCandyMachine(candyMachine.publicKey)
candyMachine.items[0].name // "My NFT #1"
candyMachine.items[1].name // "My NFT #X"
candyMachine.items[2].name // "My NFT #3"
```

API References: [addConfigLines](https://mpl-candy-machine-js-docs.vercel.app/functions/addConfigLines.html)

{% /dialect %}
{% /dialect-switcher %}

## Conclusion

And just like that, we have a loaded Candy Machine ready to mint NFTs! However, we've not created any requirements for our minting process. How can we configure the price of the mint? How can we ensure that buyers are holders of a specific token or an NFT from a specific collection? How can we set the start date of our mint? What about the end conditions?

[On the next page](/candy-machine/guards), we’ll talk about Candy Guards which make all of this possible.
