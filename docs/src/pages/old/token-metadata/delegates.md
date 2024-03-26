---
title: Delegated Authorities
metaTitle: Token Metadata - Delegated Authorities
description: Learn how to approve delegated authorities for your Assets on Token Metadata
---

Having a single authority on our assets is not always ideal. Sometimes we want to delegate some of these responsibilities to other wallets or programs so they can do things on our behalf. This is why Token Metadata offers a whole set of delegates with different scopes. {% .lead %}

## Metadata vs Token Delegates

The delegates offered by Token Metadata can be split into two categories: the **Metadata Delegates** and the **Token Delegates**. We'll go through each of them in more detail below but let's have a quick look at how they differ.

- **Metadata Delegates** are associated with the Mint account of the asset and allow the delegated authority to perform updates on the Metadata account. They are approved by the update authority of the asset and there can be as many as needed.
- **Token Delegates** are associated with the Token account of the asset and allow the delegated authority to transfer, burn and/or lock the token(s). They are approved by the owner of the asset and there can only be one per token account at a time.

## Metadata Delegates

Metadata Delegates are delegates that operate at the Metadata level. These delegates are stored using a **Metadata Delegate Record** PDA — whose seeds are `["metadata", program id, mint id, delegate role, update authority id, delegate id]`.

That account keeps track of the **Delegate** authority as well as the **Update Authority** that approved it.

{% diagram %}
{% node %}
{% node #wallet label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node x="200" parent="wallet" %}
{% node #token label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}

{% node x="200" parent="token" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}

{% node #metadata-pda parent="mint" x="-15" y="-80" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-240" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% node #metadata-delegate-pda parent="mint" x="-15" y="-260" label="PDA" theme="crimson" /%}

{% node parent="metadata-delegate-pda" x="-283" %}
{% node #metadata-delegate label="Metadata Delegate Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Key = MetadataDelegate" /%}
{% node label="Bump" /%}
{% node label="Mint" /%}
{% node label="Delegate" theme="orange" z=1 /%}
{% node label="Update Authority" theme="orange" z=1 /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" /%}
{% edge from="mint" to="metadata-delegate-pda" /%}
{% edge from="metadata-pda" to="metadata" path="straight" /%}
{% edge from="metadata-delegate-pda" to="metadata-delegate" path="straight" /%}
{% /diagram %}

Here are some key properties of Metadata Delegates:

- There can be as many Metadata delegates as needed for a given asset.
- Metadata delegates are derived from the Mint account which means they exist regardless of the owner of the asset. Thus, transferring an asset does not affect the Metadata delegates.
- Metadata delegates are also derived from the current Update Authority of the asset. This means, whenever the Update Authority is updated on an asset, all Metadata delegates are voided and cannot be used by the new Update Authority. However, if the Update Authority was to be transferred back, all Metadata delegates associated with it would automatically reactivate.
- Metadata delegates can be revoked by the Update Authority that approved them.
- Metadata delegates can also revoke themselves.

There exist 7 different types of Metadata Delegates, each with a different scope of action. Here is a table summarizing the different types of Metadata Delegates:

| Delegate                  | Self-updates | Update items in collection | Update scope                                                              |
| ------------------------- | ------------ | -------------------------- | ------------------------------------------------------------------------- |
| Authority Item            | ✅           | ❌                         | `newUpdateAuthority` ,`primarySaleHappened` ,`isMutable` ,`tokenStandard` |
| Collection                | ✅           | ✅                         | `collection` + verify/unverify collection on items                        |
| Collection Item           | ✅           | ❌                         | `collection`                                                              |
| Data                      | ✅           | ✅                         | `data`                                                                    |
| Data Item                 | ✅           | ❌                         | `data`                                                                    |
| Programmable Configs      | ✅           | ✅                         | `programmableConfigs`                                                     |
| Programmable Configs Item | ✅           | ❌                         | `programmableConfigs`                                                     |

Notice that the Metadata delegates whose name ends with `Item` can only act on themselves, whereas the other ones can also act on the collection items of the delegate asset. For instance, say we have a Collection NFT A that includes NFTs B and C. When we approve a **Data** delegate on A, we can update the `data` object of NFTs A, B and C. However, when we approve a **Data Item** delegate on A, we can only update the `data` object of NFT A.

Additionally, the **Collection** delegate is a little special as it also allows us to verify/unverify the delegated NFT on the items of the collection. In the example above, when we approve a **Collection** delegate on A, we can verify/unverify that collection on NFTs B and C.

Let's go through each of these Metadata delegates in a bit more detail and provide code samples for approving, revoking and using them.

### Authority Item Delegate

- The Delegate Authority can update a sub-set of the asset. It can update the following properties of the Metadata account:
  - `newUpdateAuthority`: transfers the Update Authority to another account.
  - `primarySaleHappened`: toggles to `true` when the primary sale of the asset has happened.
  - `isMutable`: toggles to `false` to make the asset immutable.
  - `tokenStandard`: can set the token standard if the asset was created before it was mandatory to set it.

{% dialect-switcher title="Work with Authority Item delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateAuthorityItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateAuthorityItemV1(umi, {
  mint,
  authority: updateAuthority,
  delegate: authorityItemDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeAuthorityItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeAuthorityItemV1(umi, {
  mint,
  authority: updateAuthority, // Or pass the delegate authority as a Signer to self-revoke.
  delegate: authorityItemDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update" %}

```ts
import { updateAsAuthorityItemDelegateV2 } from '@metaplex-foundation/mpl-token-metadata'

await updateAsAuthorityItemDelegateV2(umi, {
  mint,
  authority: authorityItemDelegate,
  newUpdateAuthority,
  isMutable: false,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Collection Delegate

- The Delegate Authority can update a sub-set of the asset. It can set the `collection` attribute of the Metadata account.
- When applied to a Collection NFT, the Delegate Authority can perform the following actions on the items inside that Collection:
  - It can verify and unverify that Collection NFT on the item. It can only do this if the Collection NFT is already set on the item. Otherwise, there is no way of knowing that the item is part of the delegated Collection NFT.
  - It can clear the Collection NFT from the item.

{% dialect-switcher title="Work with Collection delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateCollectionV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateCollectionV1(umi, {
  mint,
  authority: updateAuthority,
  delegate: collectionDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeCollectionV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeCollectionV1(umi, {
  mint,
  authority: updateAuthority, // Or pass the delegate authority as a Signer to self-revoke.
  delegate: collectionDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Update collection on delegated asset" %}

```ts
import {
  updateAsCollectionDelegateV2,
  collectionToggle,
} from '@metaplex-foundation/mpl-token-metadata'

await updateAsCollectionDelegateV2(umi, {
  mint,
  authority: collectionDelegate,
  collection: collectionToggle('Set', [
    { key: collectionMint, verified: false },
  ]),
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Clear collection on item" %}

```ts
import {
  updateAsCollectionDelegateV2,
  collectionToggle,
} from '@metaplex-foundation/mpl-token-metadata'

await updateAsCollectionDelegateV2(umi, {
  mint,
  delegateMint: collectionMint,
  authority: collectionDelegate,
  collection: collectionToggle('Clear'),
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Verify collection on item" %}

```ts
import {
  verifyCollectionV1,
  findMetadataPda,
} from '@metaplex-foundation/mpl-token-metadata'

await verifyCollectionV1(umi, {
  metadata: findMetadataPda(umi, { mint }),
  collectionMint,
  authority: collectionDelegate,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Unverify collection on item" %}

```ts
import {
  unverifyCollectionV1,
  findMetadataPda,
} from '@metaplex-foundation/mpl-token-metadata'

await unverifyCollectionV1(umi, {
  metadata: findMetadataPda(umi, { mint }),
  collectionMint,
  authority: collectionDelegate,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Collection Item Delegate

- The Delegate Authority can update a sub-set of the asset. It can set the `collection` attribute of the Metadata account.
- Even if the asset is a Collection NFT, and contrary to the Collection Delegate, the Collection Item Delegate cannot affect the items of that collection.

{% dialect-switcher title="Work with Collection Item delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateCollectionItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateCollectionItemV1(umi, {
  mint,
  authority: updateAuthority,
  delegate: collectionItemDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeCollectionItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeCollectionItemV1(umi, {
  mint,
  authority: updateAuthority, // Or pass the delegate authority as a Signer to self-revoke.
  delegate: collectionItemDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update" %}

```ts
import { updateAsCollectionItemDelegateV2 } from '@metaplex-foundation/mpl-token-metadata'

await updateAsCollectionItemDelegateV2(umi, {
  mint,
  authority: collectionItemDelegate,
  collection: collectionToggle('Set', [
    { key: collectionMint, verified: false },
  ]),
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Data Delegate

- The Delegate Authority can update a sub-set of the asset. It can update the entire `data` object of the Metadata account but nothing else. This means it can update the `creators` of the asset.
- Note that when updating the `creators` array inside the `data` object, it can only add and/or remove unverified creators.
- When applied to a Collection NFT, the Delegate Authority can perform the same updates on the items inside that Collection.

{% dialect-switcher title="Work with Data delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateDataV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateDataV1(umi, {
  mint,
  authority: updateAuthority,
  delegate: dataDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeDataV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeDataV1(umi, {
  mint,
  authority: updateAuthority, // Or pass the delegate authority as a Signer to self-revoke.
  delegate: dataDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update" %}

```ts
import {
  updateAsDataDelegateV2,
  fetchMetadataFromSeeds,
} from '@metaplex-foundation/mpl-token-metadata'

const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })
await updateAsDataDelegateV2(umi, {
  mint,
  authority: dataDelegate,
  data: { ...initialMetadata, name: 'Updated Name' },
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update on item" %}

```ts
import {
  updateAsDataDelegateV2,
  fetchMetadataFromSeeds,
} from '@metaplex-foundation/mpl-token-metadata'

const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })
await updateAsDataDelegateV2(umi, {
  mint,
  delegateMint: collectionMint,
  authority: dataDelegate,
  data: { ...initialMetadata, name: 'Updated Name' },
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Data Item Delegate

- The Delegate Authority can update a sub-set of the asset. It can update the entire `data` object of the Metadata account but nothing else. This means it can update the `creators` of the asset.
- Note that when updating the `creators` array inside the `data` object, it can only add and/or remove unverified creators.
- Even if the asset is a Collection NFT, and contrary to the Data Delegate, the Data Item Delegate cannot affect the items of that collection.

{% dialect-switcher title="Work with Data Item delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateDataItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateDataItemV1(umi, {
  mint,
  authority: updateAuthority,
  delegate: dataItemDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeDataItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeDataItemV1(umi, {
  mint,
  authority: updateAuthority, // Or pass the delegate authority as a Signer to self-revoke.
  delegate: dataItemDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update" %}

```ts
import {
  updateAsDataItemDelegateV2,
  fetchMetadataFromSeeds,
} from '@metaplex-foundation/mpl-token-metadata'

const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })
await updateAsDataItemDelegateV2(umi, {
  mint,
  authority: dataItemDelegate,
  data: { ...initialMetadata, name: 'Updated Name' },
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Programmable Config Delegate

- The Programmable Config Delegate is only relevant for [Programmable Non-Fungibles](/token-metadata/pnfts).
- The Delegate Authority can update the `programmableConfigs` attribute of the Metadata account but nothing else. This means it can update the `ruleSet` of the PNFT.
- When applied to a Collection NFT, the Delegate Authority can perform the same updates on the items inside that Collection.

{% dialect-switcher title="Work with Programmable Config delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateProgrammableConfigV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateProgrammableConfigV1(umi, {
  mint,
  authority: updateAuthority,
  delegate: programmableConfigDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeProgrammableConfigV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeProgrammableConfigV1(umi, {
  mint,
  authority: updateAuthority, // Or pass the delegate authority as a Signer to self-revoke.
  delegate: programmableConfigDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update" %}

```ts
import {
  updateAsAuthorityItemDelegateV2,
  ruleSetToggle,
} from '@metaplex-foundation/mpl-token-metadata'
import { findAssociatedTokenPda } from '@metaplex-foundation/mpl-toolbox'

await updateAsProgrammableConfigDelegateV2(umi, {
  mint,
  token: findAssociatedTokenPda(umi, { mint, owner: assetOwner }),
  authority: programmableConfigDelegate,
  ruleSet: ruleSetToggle('Set', [ruleSet]),
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update on item" %}

```ts
import {
  updateAsAuthorityItemDelegateV2,
  ruleSetToggle,
} from '@metaplex-foundation/mpl-token-metadata'
import { findAssociatedTokenPda } from '@metaplex-foundation/mpl-toolbox'

await updateAsProgrammableConfigDelegateV2(umi, {
  mint,
  token: findAssociatedTokenPda(umi, { mint, owner: assetOwner }),
  delegateMint: collectionMint,
  authority: programmableConfigDelegate,
  ruleSet: ruleSetToggle('Set', [ruleSet]),
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Programmable Config Item Delegate

- The Programmable Config Delegate is only relevant for [Programmable Non-Fungibles](/token-metadata/pnfts).
- The Delegate Authority can update the `programmableConfigs` attribute of the Metadata account but nothing else. This means it can update the `ruleSet` of the PNFT.
- Even if the asset is a Collection NFT, and contrary to the Programmable Config Delegate, the Programmable Config Item Delegate cannot affect the items of that collection.

{% dialect-switcher title="Work with Programmable Config Item delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateProgrammableConfigItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateProgrammableConfigItemV1(umi, {
  mint,
  authority: updateAuthority,
  delegate: programmableConfigItemDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeProgrammableConfigItemV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeProgrammableConfigItemV1(umi, {
  mint,
  authority: updateAuthority, // Or pass the delegate authority as a Signer to self-revoke.
  delegate: programmableConfigItemDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated update" %}

```ts
import {
  updateAsProgrammableConfigItemDelegateV2,
  ruleSetToggle,
} from '@metaplex-foundation/mpl-token-metadata'
import { findAssociatedTokenPda } from '@metaplex-foundation/mpl-toolbox'

await updateAsProgrammableConfigItemDelegateV2(umi, {
  mint,
  token: findAssociatedTokenPda(umi, { mint, owner: assetOwner }),
  authority: programmableConfigItemDelegate,
  ruleSet: ruleSetToggle('Set', [ruleSet]),
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Token Delegates

Token Delegates are delegates that operate at the Token level. This means they are spl-token delegates that are stored directly on the Token account of the SPL Token program. As such Token Delegates allow delegates to **transfer and burn tokens** on behalf of the owner but also **lock and unlock tokens** to prevent the owner from transferring, burning or even revoking the delegate. These delegates are crucial for applications like escrowless marketplaces, staking, asset loans, etc.

Whilst there is only one type of delegate offered by the SPL Token program, [Programmable NFTs](/token-metadata/pnfts) (PNFTs) allowed the Token Metadata program to provide more granular delegates that can be selected on a per-case basis. This is because PNFTs are always frozen on the SPL Token program which means we can build a delegate system on top of it.

We store that delegate system on a PNFT-specific account called the **Token Record** PDA — whose seeds are `["metadata", program id, mint id, "token_record", token account id]`. We synchronise the delegated authority on the SPL Token program as well but the tokens are always frozen. It is the responsibility of the Token Record account to keep track of whether the asset is really locked or not.

{% diagram height="h-64 md:h-[600px]" %}
{% node %}
{% node #wallet-1 label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node parent="wallet-1" x=-10 y=-25 label="Non-Fungibles and Semi-Fungibles" theme="transparent" /%}

{% node x="200" parent="wallet-1" %}
{% node #token-1 label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Delegate Authority" theme="orange" z=1 /%}
{% node label="Delegate Amount" theme="orange" z=1 /%}
{% /node %}

{% node x="200" parent="token-1" %}
{% node #mint-1 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}

{% node parent="wallet-1" y=150 %}
{% node #wallet-2 label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node parent="wallet-2" x=-10 y=-25 label="Programmable Non-Fungibles" theme="transparent" /%}

{% node #token-2-wrapper x="200" parent="wallet-2" %}
{% node #token-2 label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Delegate Authority" theme="orange" z=1 /%}
{% node label="Delegate Amount = 1" /%}
{% node label="Token State = Frozen" theme="orange" z=1 /%}
{% /node %}

{% node #mint-2-wrapper x="200" parent="token-2" %}
{% node #mint-2 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}

{% node #token-record-pda parent="mint-2" x="-158" y="150" label="PDA" theme="crimson" /%}

{% node parent="token-record-pda" x="-240" %}
{% node #token-record label="Token Record Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Key = TokenRecord" /%}
{% node label="Bump" /%}
{% node label="State = Locked, Unlocked, Listed" theme="orange" z=1 /%}
{% node label="Rule Set Revision" /%}
{% node label="Delegate" theme="orange" z=1 /%}
{% node label="Delegate Role" theme="orange" z=1 /%}
{% node label="Locked Transfer" /%}
{% /node %}

{% edge from="wallet-1" to="token-1" /%}
{% edge from="mint-1" to="token-1" /%}

{% edge from="wallet-2" to="token-2" /%}
{% edge from="mint-2" to="token-2" /%}
{% edge from="token-2-wrapper" to="token-record-pda" fromPosition="bottom" path="straight" /%}
{% edge from="mint-2-wrapper" to="token-record-pda" fromPosition="bottom" /%}
{% edge from="token-record-pda" to="token-record" path="straight" /%}
{% /diagram %}

Here are some key properties of Token Delegates:

- There can only be one Token Delegate per token account. Setting a new Token Delegate on the same Token account will override the existing one.
- Token delegates can be revoked by the owner of the asset as long as the asset is not locked.
- Token delegates cannot revoke themselves as they are also set on the Token Program which does not allow the delegates to self-revoke.
- Token delegates are reset on transfer. When dealing with fungible assets, the Delegate Authority is reset when all delegated tokens are transferred.
- The Standard delegate can be used by all assets except Programmable Non-Fungibles. All other Token delegates can only be used by Programmable Non-Fungibles.
- All Token delegates that can be used by Programmable Non-Fungibles store the current Delegate Authority, its role and its state — locked or unlocked — on the Token Record account of the PNFT.

There exist 6 different types of Token Delegates, each with a different scope of action. Here is a table summarizing the different types of Token Delegates:

| Delegate        | Lock/Unlock | Transfer | Burn | For              | Note                                                      |
| --------------- | ----------- | -------- | ---- | ---------------- | --------------------------------------------------------- |
| Standard        | ✅          | ✅       | ✅   | All except PNFTs |                                                           |
| Sale            | ❌          | ✅       | ❌   | PNFTs only       | Owner cannot transfer/burn until they revoke the delegate |
| Transfer        | ❌          | ✅       | ❌   | PNFTs only       | Owner can transfer/burn even when a delegate is set       |
| Locked Transfer | ✅          | ✅       | ❌   | PNFTs only       |                                                           |
| Utility         | ✅          | ❌       | ✅   | PNFTs only       |                                                           |
| Staking         | ✅          | ❌       | ❌   | PNFTs only       |                                                           |

Notice that the **Standard** delegate has a lot more power than the other PNFT-specific delegates as we must simply defer to the spl-token delegate. However, the other delegates are more granular and can be used in more specific use cases. For instance, the **Sale** delegate is perfect for listing assets on marketplaces since they forbid the owner to burn or transfer as long as the delegate is set.

Let's go through each of these Token delegates in a bit more detail and provide code samples for approving, revoking and using them.

### Standard Delegate

As mentioned above, the Standard Delegate is a wrapper around spl-token delegates. Whilst we could simply send instructions to the Token program directly, this delegate aims to offer the same API on Token Metadata regardless of the Token Standard. Additionally, Standard Delegates are able to lock/unlock assets which is not possible with native spl-token delegates.

Here are some key properties of the Standard Delegate:

- This delegate does not work with Programmable Non-Fungibles.
- The Delegate Authority can transfer the asset to any address. Doing so will revoke the Delegate Authority.
- The Delegate Authority can burn the asset.
- The Delegate Authority can lock the asset — also known as "freezing" the asset on the Token program. Until the Delegate Authority unlocks (or "thaw") the asset, the owner cannot transfer it, burn it, or revoke the Delegate Authority. This is specific to the Standard Delegate and cannot be done with a native spl-token delegate.
- When used with fungible assets, an amount greater than 1 can be provided to specify the number of tokens to delegate to the Delegate Authority.

{% dialect-switcher title="Work with Standard delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateStandardV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateStandardV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: standardDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeStandardV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeStandardV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: standardDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated transfer" %}

```ts
import { transferV1 } from '@metaplex-foundation/mpl-token-metadata'

await transferV1(umi, {
  mint,
  authority: standardDelegate,
  tokenOwner: currentOwner,
  destinationOwner: newOwner,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated burn" %}

```ts
import { burnV1 } from '@metaplex-foundation/mpl-token-metadata'

await burnV1(umi, {
  mint,
  authority: standardDelegate,
  tokenOwner: currentOwner,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Lock (freeze)" %}

```ts
import { lockV1 } from '@metaplex-foundation/mpl-token-metadata'

await lockV1(umi, {
  mint,
  authority: standardDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Unlock (thaw)" %}

```ts
import { unlockV1 } from '@metaplex-foundation/mpl-token-metadata'

await unlockV1(umi, {
  mint,
  authority: standardDelegate,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Sale Delegate (PNFT only)

- This delegate only works with Programmable Non-Fungibles.
- The Delegate Authority can transfer the PNFT to any address. Doing so will revoke the Delegate Authority.
- As long as a Sale Delegate is set on a PNFT, the PNFT enters a special Token State called `Listed`. The `Listed` Token State is a softer variation of the `Locked` Token State. During that time, the owner cannot transfer or burn the PNFT. However, the owner can revoke the Sale Delegate at any time, which will remove the `Listed` Token State and make the PNFT transferable and burnable again.

{% dialect-switcher title="Work with Sale delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateSaleV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateSaleV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: saleDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeSaleV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeSaleV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: saleDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated transfer" %}

```ts
import { transferV1 } from '@metaplex-foundation/mpl-token-metadata'

await transferV1(umi, {
  mint,
  authority: saleDelegate,
  tokenOwner: currentOwner,
  destinationOwner: newOwner,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Transfer Delegate (PNFT only)

- This delegate only works with Programmable Non-Fungibles.
- The Delegate Authority can transfer the PNFT to any address. Doing so will revoke the Delegate Authority.
- Contrary to the Sale Delegate, when a Transfer Delegate is set, the owner can still transfer and burn the PNFT.

{% dialect-switcher title="Work with Transfer delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateTransferV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateTransferV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: transferDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeTransferV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeTransferV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: transferDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated transfer" %}

```ts
import { transferV1 } from '@metaplex-foundation/mpl-token-metadata'

await transferV1(umi, {
  mint,
  authority: transferDelegate,
  tokenOwner: currentOwner,
  destinationOwner: newOwner,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Locked Transfer Delegate (PNFT only)

- This delegate only works with Programmable Non-Fungibles.
- The Delegate Authority can lock the PNFT. Until the Delegate Authority unlocks the PNFT, the owner cannot transfer it, burn it, or revoke the Delegate Authority.
- The Delegate Authority can transfer the PNFT to any address. Doing so will revoke the Delegate Authority and unlock the PNFT if it was locked.

{% dialect-switcher title="Work with Locked Transfer delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateLockedTransferV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateLockedTransferV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: lockedTransferDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeLockedTransferV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeLockedTransferV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: lockedTransferDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated transfer" %}

```ts
import { transferV1 } from '@metaplex-foundation/mpl-token-metadata'

await transferV1(umi, {
  mint,
  authority: lockedTransferDelegate,
  tokenOwner: currentOwner,
  destinationOwner: newOwner,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Lock" %}

```ts
import { lockV1 } from '@metaplex-foundation/mpl-token-metadata'

await lockV1(umi, {
  mint,
  authority: lockedTransferDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Unlock" %}

```ts
import { unlockV1 } from '@metaplex-foundation/mpl-token-metadata'

await unlockV1(umi, {
  mint,
  authority: lockedTransferDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Utility Delegate (PNFT only)

- This delegate only works with Programmable Non-Fungibles.
- The Delegate Authority can lock the PNFT. Until the Delegate Authority unlocks the PNFT, the owner cannot transfer it, burn it, or revoke the Delegate Authority.
- The Delegate Authority can burn the PNFT.

{% dialect-switcher title="Work with Utility delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateUtilityV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateUtilityV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: utilityDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeUtilityV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeUtilityV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: utilityDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Delegated burn" %}

```ts
import { burnV1 } from '@metaplex-foundation/mpl-token-metadata'

await burnV1(umi, {
  mint,
  authority: utilityDelegate,
  tokenOwner: currentOwner,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Lock" %}

```ts
import { lockV1 } from '@metaplex-foundation/mpl-token-metadata'

await lockV1(umi, {
  mint,
  authority: utilityDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Unlock" %}

```ts
import { unlockV1 } from '@metaplex-foundation/mpl-token-metadata'

await unlockV1(umi, {
  mint,
  authority: utilityDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Staking Delegate (PNFT only)

- This delegate only works with Programmable Non-Fungibles.
- The Delegate Authority can lock the PNFT. Until the Delegate Authority unlocks the PNFT, the owner cannot transfer it, burn it, or revoke the Delegate Authority.

{% dialect-switcher title="Work with Staking delegates" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

{% totem-accordion title="Approve" %}

```ts
import { delegateStakingV1 } from '@metaplex-foundation/mpl-token-metadata'

await delegateStakingV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: stakingDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Revoke" %}

```ts
import { revokeStakingV1 } from '@metaplex-foundation/mpl-token-metadata'

await revokeStakingV1(umi, {
  mint,
  tokenOwner: owner.publicKey,
  authority: owner,
  delegate: stakingDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Lock" %}

```ts
import { lockV1 } from '@metaplex-foundation/mpl-token-metadata'

await lockV1(umi, {
  mint,
  authority: stakingDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% totem-accordion title="Unlock" %}

```ts
import { unlockV1 } from '@metaplex-foundation/mpl-token-metadata'

await unlockV1(umi, {
  mint,
  authority: stakingDelegate,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Legacy Delegates

Finally, it is worth noting that — before this delegate system — collection delegates used to be stored on a specific **Collection Authority Record** PDA. That PDA is similar to the **Metadata Delegate Record** except that it supports only one role: **Collection**. This legacy collection delegate is now deprecated and we recommend using the new delegate system instead.

That being said, the Token Metadata program still accepts these legacy collection delegates wherever a new Collection delegate is expected. This is done to ensure backward compatibility with assets that are still delegating to these legacy delegates.

You can learn more about them [in the Token Metadata program](https://github.com/metaplex-foundation/mpl-token-metadata/blob/main/programs/token-metadata/program/src/instruction/collection.rs) directly.
