---
title: FAQ
metaTitle: Token Metadata - FAQ
description: Frequently asked questions about Token Metadata
---

## How can I filter Metadata accounts by fields located after the `creators` array using `getProgramAccounts`?

When using [the `getProgramAccounts` method from the RPC API](https://docs.solana.com/developing/clients/jsonrpc-api#getprogramaccounts), it is common to want to filter accounts by fields using `memcmp` filters.

Since the `memcmp` filter compares arrays of bytes, this approach requires knowledge of the data structure of the account. Additionally, it requires the length of that data structure to be fixed, so we can find the position of the field we're looking for, for every single account.

Unfortunately, the `creators` field of the [Metadata Account](./accounts#metadata) is a vector that can contain one to five creators. This means the position of every field after it depends on how many creators the account has.

Note that adding new fields to an account without adding breaking change requires appending optional fields to the accounts. This unfortunately means that any new features we may add to the Metadata Account will be after the `creators` field and therefore will be challenging to filter via `getProgramAccounts`.

There are several ways to solve this problem:

- If every single account we are trying to filter has **the same number of creators**, then we can figure out the offset of the next field. We can do this by adding `4 + 34 * n` to the `creators` offset, where `n` is the fixed number of creators and `4` is because 4 bytes are used to store the length of the vector. This unblocks us for every field of fixed length present after the `creators` field. Unfortunately, the problem reoccurs as soon as we reach another field of variable size such as another vector or an optional field. Therefore, this solution is only valid if we know the exact length of all variable fields before the field we are trying to filter with.
- Another solution is to **crawl transactions to find the accounts we're looking for**. This approach is a bit more complex and requires us to implement a custom procedure that fits our needs. For instance, we can use `getSignaturesForAddress` to get all transactions associated with an account and then use `getTransaction` on each of them to access their transaction data before filtering the ones that matter for our use case. It is also worth considering that this approach might not be the most future-proof solution since we might end up relying on instructions that could be deprecated in favor of new ones.
- Finally, **the most robust solution is to index the data we're looking for using a [Geyser Plugin](https://docs.solana.com/developing/plugins/geyser-plugins)**. This currently requires a significant setup, but we end up with a reliable data store that mirrors the data in the Solana blockchain. Not only does it fix our filtering issue, but it also provides a much more convenient and efficient way to access our data.

## How can I filter Metadata accounts by collection using `getProgramAccounts`?

As mentioned in the question above, filtering by fields present after the `creators` array is a challenging task because it is not a field of fixed size.

Since **the `collection` field is located after the `creators` field**, this makes filtering [Metadata accounts](./accounts#metadata) by collection somewhat difficult.

The solutions listed above apply here too but, because this is a common problem, we have written a more detailed guide on how to use transaction crawling to access Metadata accounts of a given collection: **[Get Collection Methods](https://metaplex.notion.site/Get-Collection-Methods-1ff0b118e4ce4605971df60e753a8559)**.

## Why are the mint and freeze authorities transferred to the Edition PDA?

One question we often receive is: Why does the Token Metadata program transfer the `Mint Authority` and the `Freeze Authority` of the Mint Account to the Edition PDA when creating NFTs? Why not just void them by setting them to `None`?

Let's take a look at why this is the case for both of these authorities separately.

### Mint Authority

Controlling the Mint Authority is a crucial step for ensuring the non-fungibility of a token. Without this protection, someone could mint more tokens for a given NFT and therefore make the NFT fungible.

One way to prevent this from happening is to set the Mint Authority to `None` to ensure no one will ever be able to mint any more tokens for that NFT. However, the Token Metadata program sets that authority to the Edition PDA — which links to a Master Edition account or an Edition account.

**But Why?** The short answer is: **it enables us to deploy upgrades to the Token Metadata program at a much lower cost**.

Losing the Mint Authority is an irreversible action which means we could never leverage it to migrate NFTs to newer versions. For instance, say we want to change the way Original and Printed NFTs are structured and, instead of using Edition accounts, we want to leverage tokens. Without the Mint Authority, migrating NFTs to the new version would simply be impossible.

**Losing this authority would limit the scope of features and changes we may want to implement in the future** and that's why we're not setting it to `None`.

However, that doesn't mean someone can use that Mint Authority to mint more tokens on your NFT. The Mint Authority isn't transferred to someone's public key, it is transferred to a PDA that belongs to the Token Metadata program. Therefore, only an instruction provided by the program could make use of it and such instruction does not exist on the program. It is important to note that the Token Metadata program is completely open-source so anyone can inspect it to ensure the Mint Authority is not used to mint more tokens.

### Freeze Authority

Controlling the Freeze Authority allows someone to freeze a Token account, making that account immutable until it is thawed.

One reason this authority is transferred to the Edition PDA of the Token Metadata program is, similarly to the Mint Authority, it increases the scope of potential new features and upgrades.

However, contrary to the Mint Authority, we actually make use of that authority in the program.

The [`FreezeDelegatedAccount`](./instructions#freeze-the-token-account-as-a-delegate) and [`ThawDelegatedAccount`](./instructions#thaw-the-token-account-as-a-delegate) instructions are the only instructions that make use of the Freeze Authority. They allow the Delegate of a Token account to freeze (and thaw) that Token account to make them what we call "**Non-Transferable NFTs**". This enables a variety of use-cases such as preventing someone from selling an NFT while it is listed in an escrowless marketplace.

## Why does the Metadata account have both on-chain and off-chain data?

The [Metadata account](./accounts#metadata) contains on-chain data, yet it also has a `URI` attribute which points to an off-chain JSON file which provides additional data. So why is that? Can't we just store everything on-chain? Well, there are several issues with that:

- We have to pay rent to store data on-chain. If we had to store everything within the Metadata account, which may include long texts such as the description of an NFT, it would require a lot more bytes and minting an NFT would suddenly be a lot more expensive.
- On-chain data is much less flexible. Once an account is created using a certain structure, it cannot easily be changed. Therefore, if we had to store everything on-chain, the NFT standard would be a lot harder to evolve with the demands of the ecosystem.

Therefore, splitting the data into on-chain and off-chain data allows us to get the best of both worlds where on-chain data can be used by the program **to create guarantees and expectations for its users** whereas off-chain data can be used **to provide standardized yet flexible information**.

## Are there any costs to using Token Metadata?

Token Metadata currently charges very small fees ranging between 0.001 SOL and 0.01 SOL to the caller of certain instructions. More details can be found on the Protocol Fees page.

## Where can I find the deprecated instructions?

Some of the instructions of the Token Metadata program have been through a few iterations and have been deprecated in favour of newer ones. The deprecated instructions are still available in the program but they are not documented on the Developer Hub as they are no longer the recommended way to interact with the program. That being said, if you are looking for the deprecated instructions, you can find them in the Token Metadata program repository. Here is a list of them:

- [CreateMetadataAccountV3](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L448)
- [UpdateMetadataAccountV2](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L241)
- [UpdatePrimarySaleHappenedViaToken](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L112)
- [SignMetadata](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L146)
- [RemoveCreatorVerification](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L388)
- [CreateMasterEditionV3](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L267)
- [MintNewEditionFromMasterEditionViaToken](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L202)
- [ConvertMasterEditionV1ToV2](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L210)
- [PuffMetadata](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L236)
- [VerifyCollection](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L278)
- [SetAndVerifyCollection](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L367)
- [UnverifyCollection](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L334)
- [Utilize](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L296)
- [ApproveUseAuthority](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L311)
- [RevokeUseAuthority](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L324)
- [ApproveCollectionAuthority](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L346)
- [RevokeCollectionAuthority](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L354)
- [FreezeDelegatedAccount](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L375)
- [ThawDelegatedAccount](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L383)
- [BurnNft](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L383)
- [BurnEditionNft](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L487)
- [VerifySizedCollectionItem](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L411)
- [SetAndVerifySizedCollectionItem](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L437)
- [UnverifySizedCollectionItem](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L423)
- [SetCollectionSize](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L456)
- [SetTokenStandard](https://github.com/metaplex-foundation/mpl-token-metadata/blob/d1a13273cb23c033bda97b4d47b9731b51ef5a2f/programs/token-metadata/program/src/instruction/mod.rs#L464)
