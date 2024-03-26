---
title: Verified Collections
metaTitle: Token Metadata - Verified Collections
description: Learn how to safely wrap Assets into collections on Token Metadata
---

Certified Collections enables NFTs – and tokens in general — **to be grouped together** and for that information to be **verified on-chain**. Additionally, it makes it easier to manage these collections by allocating data for them on-chain. {% .lead %}

This feature provides the following advantages:

- Easy to identify which collection any given NFT belongs to without making additional on-chain calls.
- Possible to find all NFTs that belong to a given collection ([Check the FAQ to see how](/token-metadata/faq#how-can-i-filter-metadata-accounts-by-collection-using-get-program-accounts)).
- Easy to manage the collection metadata such as its name, description and image.

## Collections are NFTs

In order to group NFTs — or any token — together, we must first create a Collection NFT whose purpose is to store any metadata related to that collection. That's right, **a collection of NFT is itself, an NFT**. It has the same data layout on-chain as any other NFT.

The difference between a Collection NFT and a Regular NFT is that the information provided by the former will be used to define the group of NFTs it contains whereas the latter will be used to define the NFT itself.

## Associating NFTs to Collection NFTs

Collection NFTs and Regular NFTs are **linked together using a "Belong To" relationship** on the Metadata account. The optional `Collection` field on the Metadata account has been created for that purpose.

- If the `Collection` field is set to `None`, it means the NFT is not part of a collection.
- If the `Collection` field is set, it means the NFT is part of the collection specified within that field.

As such, the `Collection` field contains two nested fields:

- `Key`: This field points to the Collection NFT the NFT belongs to. More precisely, it points to **the public key of the Mint Account** of the Collection NFT. This Mint Account must be owned by the SPL Token program.
- `Verified`: This boolean is very important as it is used to verify that the NFT is truly part of the collection it points to. More on that below.

{% diagram %}

{% node %}
{% node #mint-1 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-1" y=-180 %}
{% node #metadata-1 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node label="Collection = None" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-1" x=-10 y=-25 theme="transparent" %}
Collection NFT {% .font-bold %}
{% /node %}
{% node #metadata-pda-1 parent="metadata-1" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-1" x=360 %}
{% node #mint-2 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-2" y=-180 %}
{% node #metadata-2 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node #metadata-2-collection theme="orange" z=1 %}
Collection

\- Key \
\- Verified = **True**

{% /node %}
{% /node %}
{% node parent="metadata-2" x=-10 y=-40 theme="transparent" %}
Regular NFT {% .font-bold %}

Attached to a collection
{% /node %}
{% node #metadata-pda-2 parent="metadata-2" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-2" x=360 %}
{% node #mint-3 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-3" y=-180 %}
{% node #metadata-3 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node label="Collection = None" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-3" x=-10 y=-40 theme="transparent" %}
Regular NFT {% .font-bold %}

With no collection
{% /node %}
{% node #metadata-pda-3 parent="metadata-3" x="-100" label="PDA" theme="crimson" /%}

{% edge from="mint-1" to="metadata-pda-1" theme="dimmed" /%}
{% edge from="metadata-pda-1" to="metadata-1" path="straight" theme="dimmed" /%}
{% edge from="mint-2" to="metadata-pda-2" theme="dimmed" /%}
{% edge from="metadata-pda-2" to="metadata-2" path="straight" theme="dimmed" /%}
{% edge from="mint-3" to="metadata-pda-3" theme="dimmed" /%}
{% edge from="metadata-pda-3" to="metadata-3" path="straight" theme="dimmed" /%}
{% edge from="metadata-2-collection" to="mint-1" theme="orange" /%}

{% /diagram %}

## Differentiating NFTs from Collection NFTs

The `Collection` field alone allows us to link NFTs and Collections together but it doesn't help us identify if a given NFT is a Regular NFT or a Collection NFT. That's why the `CollectionDetails` field was created. It provides additional context on Collection NFTs and differentiates them from Regular NFTs.

- If the `CollectionDetails` field is set to `None`, it means the NFT is a **Regular NFT**.
- If the `CollectionDetails` field is set, it means the NFT is a **Collection NFT** and additional attributes can be found inside this field.

The `CollectionDetails` is an optional enum that currently contains only one option `V1`. This option is a struct that contains the following field:

- `Size`: The size of the collection, i.e. the number of NFTs that are directly linked to this Collection NFT. This number is automatically computed by the Token Metadata program but can also be set manually to facilitate the migration process. Note that there currently is [a MIP in place to deprecate this `Size` attribute](https://github.com/metaplex-foundation/mip/blob/main/mip-3.md).

{% diagram %}

{% node %}
{% node #mint-1 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-1" y=-230 %}
{% node #metadata-1 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node label="Collection = None" theme="orange" z=1 /%}
{% node label="Use" /%}
{% node theme="orange" z=1 %}
CollectionDetails = **Some**
{% /node %}
{% /node %}
{% node parent="metadata-1" x=-10 y=-25 theme="transparent" %}
Collection NFT {% .font-bold %}
{% /node %}
{% node #metadata-pda-1 parent="metadata-1" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-1" x=360 %}
{% node #mint-2 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-2" y=-230 %}
{% node #metadata-2 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node #metadata-2-collection theme="orange" z=1 %}
Collection

\- Key \
\- Verified = **True**

{% /node %}
{% node label="Use" /%}
{% node label="CollectionDetails = None" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-2" x=-10 y=-40 theme="transparent" %}
Regular NFT {% .font-bold %}

Attached to a collection
{% /node %}
{% node #metadata-pda-2 parent="metadata-2" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-2" x=360 %}
{% node #mint-3 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-3" y=-230 %}
{% node #metadata-3 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node label="Collection = None" theme="orange" z=1 /%}
{% node label="Use" /%}
{% node label="CollectionDetails = None" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-3" x=-10 y=-40 theme="transparent" %}
Regular NFT {% .font-bold %}

With no collection
{% /node %}
{% node #metadata-pda-3 parent="metadata-3" x="-100" label="PDA" theme="crimson" /%}

{% edge from="mint-1" to="metadata-pda-1" theme="dimmed" /%}
{% edge from="metadata-pda-1" to="metadata-1" path="straight" theme="dimmed" /%}
{% edge from="mint-2" to="metadata-pda-2" theme="dimmed" /%}
{% edge from="metadata-pda-2" to="metadata-2" path="straight" theme="dimmed" /%}
{% edge from="mint-3" to="metadata-pda-3" theme="dimmed" /%}
{% edge from="metadata-pda-3" to="metadata-3" path="straight" theme="dimmed" /%}
{% edge from="metadata-2-collection" to="mint-1" theme="orange" /%}

{% /diagram %}

## Creating Collection NFTs

Creating a Collection NFT is very similar to creating a Regular NFT. The only difference is that we must set the `CollectionDetails` field as seen in the previous section. Some of our SDKs encapsulate this by requesting a `isCollection` attribute when creating an NFT.

{% dialect-switcher title="Create a Collection NFT" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { generateSigner, percentAmount } from '@metaplex-foundation/umi'
import { createNft } from '@metaplex-foundation/mpl-token-metadata'

const collectionMint = generateSigner(umi)
await createNft(umi, {
  mint: collectionMint,
  name: 'My Collection',
  uri: 'https://example.com/my-collection.json',
  sellerFeeBasisPoints: percentAmount(5.5), // 5.5%
  isCollection: true,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

## Nested Collection NFTs

Because Collections and NFTs are linked together via a "Belong To" relationship, it is possible by design to define nested collections. In this scenario, the `Collection` and `CollectionDetails` fields can be used together to differentiate Root and Nested Collection NFTs.

{% diagram %}

{% node %}
{% node #mint-1 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-1" y=-230 %}
{% node #metadata-1 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node label="Collection = None" theme="orange" z=1 /%}
{% node label="Use" /%}
{% node label="CollectionDetails = Some" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-1" x=-10 y=-40 theme="transparent" %}
Collection NFT {% .font-bold %}

Root collection
{% /node %}
{% node #metadata-pda-1 parent="metadata-1" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-1" x=360 %}
{% node #mint-2 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-2" y=-230 %}
{% node #metadata-2 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node #metadata-2-collection theme="orange" z=1 %}
Collection

\- Key \
\- Verified = **True**

{% /node %}
{% node label="Use" /%}
{% node label="CollectionDetails = Some" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-2" x=-10 y=-40 theme="transparent" %}
Collection NFT {% .font-bold %}

Nested collection
{% /node %}
{% node #metadata-pda-2 parent="metadata-2" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-2" x=360 %}
{% node #mint-3 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-3" y=-230 %}
{% node #metadata-3 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node #metadata-3-collection theme="orange" z=1 %}
Collection

\- Key \
\- Verified = **True**

{% /node %}
{% node label="Use" /%}
{% node label="CollectionDetails = None" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-3" x=-10 y=-40 theme="transparent" %}
Regular NFT {% .font-bold %}

Attached to a collection
{% /node %}
{% node #metadata-pda-3 parent="metadata-3" x="-100" label="PDA" theme="crimson" /%}

{% edge from="mint-1" to="metadata-pda-1" theme="dimmed" /%}
{% edge from="metadata-pda-1" to="metadata-1" path="straight" theme="dimmed" /%}
{% edge from="mint-2" to="metadata-pda-2" theme="dimmed" /%}
{% edge from="metadata-pda-2" to="metadata-2" path="straight" theme="dimmed" /%}
{% edge from="mint-3" to="metadata-pda-3" theme="dimmed" /%}
{% edge from="metadata-pda-3" to="metadata-3" path="straight" theme="dimmed" /%}
{% edge from="metadata-2-collection" to="mint-1" theme="orange" /%}
{% edge from="metadata-3-collection" to="mint-2" theme="orange" /%}

{% /diagram %}

## Verifying Collection NFTs

As mentioned above, the `Collection` field contains a `Verified` boolean which is used to **determine if the NFT is truly part of the collection it points to**. Without this field, anyone could pretend their NFT to be part of any collection.

{% diagram %}

{% node %}
{% node #mint-1 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-1" y=-230 %}
{% node #metadata-1 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node label="Collection = None" theme="orange" z=1 /%}
{% node label="Use" /%}
{% node theme="orange" z=1 %}
CollectionDetails = **Some**
{% /node %}
{% /node %}
{% node parent="metadata-1" x=-10 y=-25 theme="transparent" %}
Collection NFT {% .font-bold %}
{% /node %}
{% node #metadata-pda-1 parent="metadata-1" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-1" x=360 %}
{% node #mint-2 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-2" y=-230 %}
{% node #metadata-2 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node #metadata-2-collection theme="mint" z=1 %}
Collection

\- Key \
\- Verified = **True**

{% /node %}
{% node label="Use" /%}
{% node label="CollectionDetails = None" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-2" x=-10 y=-55 theme="transparent" %}
Verified NFT {% .font-bold .text-emerald-600 %}

The Collection NFT verified this NFT \
so we know for sure it is part of it.
{% /node %}
{% node #metadata-pda-2 parent="metadata-2" x="-100" label="PDA" theme="crimson" /%}

{% node parent="mint-2" x=360 %}
{% node #mint-3 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node parent="mint-3" y=-230 %}
{% node #metadata-3 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node label="Token Standard" /%}
{% node #metadata-3-collection theme="red" z=1 %}
Collection

\- Key \
\- Verified = **False**

{% /node %}
{% node label="Use" /%}
{% node label="CollectionDetails = None" theme="orange" z=1 /%}
{% /node %}
{% node parent="metadata-3" x=-10 y=-55 theme="transparent" %}
Unverified NFT {% .font-bold .text-red-500 %}

This could be anyone's NFT pretending \
to be part of this collection.
{% /node %}
{% node #metadata-pda-3 parent="metadata-3" x="-100" label="PDA" theme="crimson" /%}

{% edge from="mint-1" to="metadata-pda-1" theme="dimmed" /%}
{% edge from="metadata-pda-1" to="metadata-1" path="straight" theme="dimmed" /%}
{% edge from="mint-2" to="metadata-pda-2" theme="dimmed" /%}
{% edge from="metadata-pda-2" to="metadata-2" path="straight" theme="dimmed" /%}
{% edge from="mint-3" to="metadata-pda-3" theme="dimmed" /%}
{% edge from="metadata-pda-3" to="metadata-3" path="straight" theme="dimmed" /%}
{% edge from="metadata-2-collection" to="mint-1" theme="mint" /%}
{% edge from="metadata-3-collection" to="mint-1" theme="red" path="straight" /%}

{% /diagram %}

In order to flip that `Verified` boolean to `True`, the Authority of the Collection NFT must sign the NFT to prove that it is allowed to be part of the collection.

{% callout title="EXTREMELY IMPORTANT" type="warning" %}

Explorers, Wallets and Marketplaces, **MUST CHECK** that `Verified` is `true`. Verified can only be set true if the Authority on the Collection NFT has run one of the Token Metadata `Verify` instructions over the NFT.

This is the same pattern as the `Creators` field where `Verified` must be true to validate the NFT.

In order to check if a collection is valid on an NFT, it **MUST** have a collection struct set with:

- The `key` field matching the mint address of the appropriate collection parent.
- The `verified` field set to `true`.

If those two steps are not followed you could be exposing fraudulent NFTs on real collections.
{% /callout %}

### Verify

Once the `Collection` attribute is set on an NFT, an authority of the Collection NFT can send a **Verify** instruction on the Token Metadata to flip its `verify` attribute from `false` to `true`. This instruction accepts the following attributes:

- **Metadata**: The address of the NFT's Metadata account. This is the NFT we want to verify inside the collection.
- **Collection Mint**: The address of the Collection NFT's Mint account. This is the Collection NFT that is already set on the Metadata account of the NFT but not yet verified.
- **Authority**: The authority of the Collection NFT as a Signer. This can be the Update Authority of the Collection NFT or an approved delegate with the appropriate role (See "[Delegated Authority](/token-metadata/delegates)" page).

Here is how you can use our SDKs to verify a Collection NFT on Token Metadata.

{% dialect-switcher title="Verify a Collection NFT" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { verifyCollectionV1 } from '@metaplex-foundation/mpl-token-metadata'

await verifyCollectionV1(umi, {
  metadata,
  collectionMint,
  authority: collectionAuthority,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

### Unverify

Reciprocally, the authority of a Collection NFT can unverify any NFTs that are part of its collection. This is done by sending an **Unverify** instruction to the Token Metadata program whose attributes are the same as the **Verify** instruction.

{% dialect-switcher title="Unverify a Collection NFT" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { unverifyCollectionV1 } from '@metaplex-foundation/mpl-token-metadata'

await unverifyCollectionV1(umi, {
  metadata,
  collectionMint,
  authority: collectionAuthority,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}
