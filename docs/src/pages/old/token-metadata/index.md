---
title: Overview
metaTitle: Token Metadata - Overview
description: Provides a high-level overview of the Solana NFT standard.
---

The Token Metadata program is a fundamental program when dealing NFTs and Fungible assets on the Solana blockchain. In this overview, we explain what this program does and how we can leverage its various features at a high level. {% .lead %}

{% quick-links %}

{% quick-link title="Getting Started" icon="InboxArrowDown" href="/token-metadata/getting-started" description="Find the language or library of your choice and get started with digital assets on Solana." /%}

{% quick-link title="API reference" icon="CodeBracketSquare" href="/token-metadata/references" description="Looking for something specific? Have a peak at our API References and find your answer." /%}

{% /quick-links %}

## Introduction

The Token Metadata program is one of the most important programs when dealing with NFTs on the Solana blockchain. Its main goal is to **attach additional data to [Fungible](https://en.wikipedia.org/wiki/Fungibility) or Non-Fungible [Tokens](https://spl.solana.com/token)** on Solana.

It achieves this using [Program Derived Addresses](/understanding-programs/#program-derived-addresses-pda) (PDAs) that are _derived_ from the address of Mint Accounts. If you’re not familiar with [Solana’s Token program](https://spl.solana.com/token), _Mint Accounts_ are responsible for storing the global information of a Token and _Token Accounts_ store the relationship between a wallet and a Mint Account.

{% diagram %}
{% node %}
{% node #wallet label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}
{% node y="70" parent="wallet" label="Someone's wallet." theme="transparent" /%}

{% node x="200" parent="wallet" %}
{% node #token label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node y="70" parent="token" theme="transparent" %}
Stores the number of \
tokens owned by the wallet, \
among other things.
{% /node %}

{% node x="200" parent="token" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}
{% node y="70" parent="mint" theme="transparent" %}
Stores information about the \
token itseft. E.g. its current \
supply and its authorities.
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}

{% /diagram %}

Whilst Mint Accounts contain a few data attributes such as its current supply, it doesn't offer the ability to inject standardized data that can be understood by apps and marketplaces.

This is why the Token Metadata program offers a **Metadata Account** that attaches itself to a Mint Account via a PDA.

{% diagram height="h-64 md:h-[500px]" %}
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

{% node #metadata-pda parent="mint" x="41" y="-100" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-240" y="-300" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Key = MetadataV1" /%}
{% node label="Update Authority" /%}
{% node label="Mint" /%}
{% node label="Name" /%}
{% node label="Symbol" /%}
{% node label="URI" /%}
{% node label="Seller Fee Basis Points" /%}
{% node label="Creators" /%}
{% node label="Primary Sale Happened" /%}
{% node label="Is Mutable" /%}
{% node label="Edition Nonce" /%}
{% node label="Token Standard" /%}
{% node label="Collection" /%}
{% node label="Uses" /%}
{% node label="Collection Details" /%}
{% node label="Programmable Configs" /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" path="straight" /%}
{% edge from="metadata-pda" to="metadata" /%}

{% /diagram %}

That Metadata Account holds a lot of valuable information that can be used throughout the ecosystem. For instance, it maintains a list of creators for the token. Each creator has a `Verified` attribute that, when `True`, guarantees the token was signed by that creator. Each creator also has a `Share` attribute that can be used by marketplaces to distribute royalties.

By attaching more data to the Mint Account, **the Token Metadata program is able to make Digital Assets** of regular on-chain Tokens.

## A JSON standard

One important attribute of the Metadata Account is the `URI` attribute that points to a JSON file off-chain. This is used to safely provide additional data whilst not being constrained by the fees involved in storing on-chain data. That JSON file [follows a certain standard](/token-metadata/token-standard) that anyone can use to find useful information on tokens.

{% diagram height="h-64 md:h-[500px]" %}
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

{% node #metadata-pda parent="mint" x="41" y="-100" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-240" y="-300" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Key = MetadataV1" /%}
{% node label="Update Authority" /%}
{% node label="Mint" /%}
{% node label="Name" /%}
{% node label="Symbol" /%}
{% node #uri label="URI" /%}
{% node label="Seller Fee Basis Points" /%}
{% node label="Creators" /%}
{% node label="Primary Sale Happened" /%}
{% node label="Is Mutable" /%}
{% node label="Edition Nonce" /%}
{% node label="Token Standard" /%}
{% node label="Collection" /%}
{% node label="Uses" /%}
{% node label="Collection Details" /%}
{% node label="Programmable Configs" /%}
{% /node %}

{% node parent="uri" x="-200" y="-23" %}
{% node #json theme="slate" %}
Off-chain \
JSON Metadata
{% /node %}
{% node label="Name" /%}
{% node label="Description" /%}
{% node label="Image" /%}
{% node label="Animated URL" /%}
{% node label="Attributes" /%}
{% node label="..." /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" path="straight" /%}
{% edge from="metadata-pda" to="metadata" /%}
{% edge from="uri" to="json" path="straight" /%}

{% /diagram %}

Note that, this JSON file can be stored using a permanent storage solution such as Arweave to ensure it cannot be updated. Additionally, one can use the `Is Mutable` attribute of the Metadata Account to make it immutable and, therefore, forbid the `URI` attribute — and other attributes such as `Name` and `Creators` — to ever be changed. Using this combination, we can guarantee the immutability of the off-chain JSON file.

## NFTs

You might be wondering: what has this got to do with NFTs? Well, NFTs are special tokens that are Non-Fungible.

More precisely, NFTs in Solana are Mint Accounts with the following characteristics:

- It has **a supply of 1**, meaning only one token is in circulation.
- It has **zero decimals**, meaning there cannot be such a thing as 0.5 tokens.
- It has **no mint authority**, meaning no one can ever mint additional tokens.

What we end up with is a token that cannot be traded with something of the same kind, which is the definition of a Non-Fungible Token (NFT).

{% diagram %}
{% node %}
{% node #wallet label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node x="200" parent="wallet" %}
{% node #token label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Amount = 1" /%}
{% /node %}

{% node x="200" parent="token" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Mint Authority = None" /%}
{% node label="Supply = 1" /%}
{% node label="Decimals = 0" /%}
{% /node %}

{% node #metadata-pda parent="mint" x="41" y="-80" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-240" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" path="straight" /%}
{% edge from="metadata-pda" to="metadata" path="straight" /%}
{% /diagram %}

In this particular yet popular case, the goal of the Metadata Account is to provide the actual data of that NFT to make it a useful Digital Asset.

Additionally, the Token Metadata program offers another account specifically for NFTs called the **Master Edition Account**. This account is also a PDA derived from the Mint Account.

Before creating this account, the Token Metadata program will ensure the special characteristics of Non-Fungible Tokens listed above are met. However, it is worth noting that, instead of voiding the Mint Authority, it will transfer both the Mint Authority and the Freeze Authority to the Master Edition PDA to ensure no one can mint or freeze tokens without going through the Token Metadata program. You can [read more about why this decision was made in the FAQ](/token-metadata/faq#why-are-the-mint-and-freeze-authorities-transferred-to-the-edition-pda).

Thus, **the existence of the Master Edition account acts as proof of Non-Fungibility** for that Mint Account.

{% diagram %}
{% node %}
{% node #wallet label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node x="200" parent="wallet" %}
{% node #token label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Amount = 1" /%}
{% /node %}

{% node x="200" parent="token" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node #mint-authority label="Mint Authority = Edition" /%}
{% node label="Supply = 1" /%}
{% node label="Decimals = 0" /%}
{% node #freeze-authority label="Freeze Authority = Edition" /%}
{% /node %}

{% node #metadata-pda parent="mint" x="-10" y="-80" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-240" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% node #master-edition-pda parent="mint" x="-10" y="-220" label="PDA" theme="crimson" /%}

{% node parent="master-edition-pda" x="-240" %}
{% node #master-edition label="Master Edition Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Key = MasterEditionV2" /%}
{% node label="Supply" /%}
{% node label="Max Supply" /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" /%}
{% edge from="mint" to="master-edition-pda" /%}
{% edge from="metadata-pda" to="metadata" path="straight" /%}
{% edge from="master-edition-pda" to="master-edition" path="straight" /%}
{% edge from="mint-authority" to="master-edition-pda" dashed=true arrow="none" fromPosition="right" toPosition="right" animated=true /%}
{% edge from="freeze-authority" to="master-edition-pda" dashed=true arrow="none" fromPosition="right" toPosition="right" animated=true /%}
{% /diagram %}

## Printing Editions

In addition to being Non-Fungibility evidence, the Master Edition account also allows users to print one or multiple copies of an NFT.

This feature is particularly helpful to creators that want to offer multiple copies of their 1/1 NFTs to their audience.

The Master Edition account contains an optional `Max Supply` attribute that dictates the maximum amount of NFTs that can be printed that way. If set to `0`, printing is disabled. If set to `None` an unlimited amount of copies can be printed.

The Master Edition NFT, a.k.a. Original NFT, acts as the master record that one can use to print copies, a.k.a. Print NFTs.

Each Print NFT is made of its own Mint Account and its own Metadata Account whose data is copied from the Original NFT. However, instead of having a Master Edition account attached to their Mint Account, Print NFTs use yet another PDA account called an **Edition Account**. This account keeps track of the edition number and the parent Master Edition it originated from.

Note that the Master Edition account and the Edition account share the same seeds for their PDA. That means an NFT can be one or the other but not both.

{% diagram %}
{% node %}
{% node #wallet label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node x="200" parent="wallet" %}
{% node #token label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Amount = 1" /%}
{% /node %}

{% node x="200" parent="token" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node #mint-authority label="Mint Authority = Edition" /%}
{% node label="Supply = 1" /%}
{% node label="Decimals = 0" /%}
{% node #freeze-authority label="Freeze Authority = Edition" /%}
{% /node %}

{% node #metadata-pda parent="mint" x="-10" y="-80" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-280" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% node #master-edition-pda parent="mint" x="-10" y="-160" label="PDA" theme="crimson" /%}

{% node parent="master-edition-pda" x="-280" %}
{% node #master-edition label="Master Edition Account" theme="crimson" /%}
{% node label="Owner: Token edition Program" theme="dimmed" /%}
{% /node %}

{% node parent="master-edition" y="-140" %}
{% node #edition label="Edition Account" theme="crimson" /%}
{% node label="Owner: Token edition Program" theme="dimmed" /%}
{% node label="Key = EditionV1" /%}
{% node #edition-parent label="Parent" /%}
{% node label="Edition" /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" /%}
{% edge from="mint" to="master-edition-pda" /%}
{% edge from="metadata-pda" to="metadata" path="straight" /%}
{% edge from="master-edition-pda" to="master-edition" path="straight" /%}
{% edge from="master-edition-pda" to="edition" fromPosition="left" label="OR" /%}
{% edge from="mint-authority" to="master-edition-pda" dashed=true arrow="none" fromPosition="right" toPosition="right" /%}
{% edge from="freeze-authority" to="master-edition-pda" dashed=true arrow="none" fromPosition="right" toPosition="right" /%}
{% edge from="edition-parent" to="master-edition" dashed=true arrow="none" fromPosition="left" toPosition="left" /%}
{% /diagram %}

## Semi-Fungible Tokens

Whilst NFTs are the biggest use case of the Token Metadata program, it’s important to notice that the program also works with Fungible Token and, what we call, Semi-Fungible Tokens or Fungible Assets.

At the end of the day, the Metadata account helps attach data to tokens regardless of their fungibility. However, the standard of the off-chain JSON file will vary slightly to accommodate their needs.

To safely identify the fungibility of a token — and, thus, the standard that we should use — the Metadata account keeps track of that information in its `Token Standard` attribute. This attribute is automatically computed by the program and cannot be manually updated. It can take the following values.

- `NonFungible`: The Mint account is associated with a Master Edition account and, therefore, is Non-Fungible. This is your typical NFT standard.
- `NonFungibleEdition`: This is the same as `NonFungible` but the NFT was printed from an Original NFT and, thus, is associated with an Edition account instead of a Master Edition account.
- `FungibleAsset`: The Mint account is Fungible but has zero decimal places. Having zero decimals means we can treat the token as an asset whose supply is not limited to one. For instance, Fungible Assets can be used in the gaming industry to store resources such as “Wood” or “Iron”.
- `Fungible`: The Mint account is Fungible and has more than one decimal place. This is more likely going to be a token used as a decentralised currency.
- `ProgrammableNonFungible`: A special `NonFungible` token that is frozen at all times to enforce custom authorization rules. See the next section for more information.

You can [read more about these standards here](/token-metadata/token-standard).

{% diagram height="h-64 md:h-[500px]" %}
{% node %}
{% node #mint-1 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Mint Authority = Edition" /%}
{% node label="Supply = 1" /%}
{% node label="Decimals = 0" /%}
{% node label="Freeze Authority = Edition" /%}
{% /node %}
{% node parent="mint-1" y="-20" x="-10" label="NonFungible" theme="transparent" /%}

{% node parent="mint-1" x="220" #metadata-1-pda label="PDA" theme="crimson" /%}
{% node parent="metadata-1-pda" x="140" %}
{% node #metadata-1 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Token Standard = NonFungible" /%}
{% /node %}

{% node parent="mint-1" x="220" y="100" #master-edition-pda label="PDA" theme="crimson" /%}
{% node parent="master-edition-pda" x="140" %}
{% node #master-edition label="Master Edition Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}
{% node parent="master-edition" y="80" %}
{% node #edition label="Edition Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% node parent="mint-1" y="260" %}
{% node #mint-2 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Decimals = 0" /%}
{% /node %}
{% node parent="mint-2" y="-20" x="-10" label="FungibleAsset" theme="transparent" /%}

{% node parent="mint-2" x="220" #metadata-2-pda label="PDA" theme="crimson" /%}
{% node parent="metadata-2-pda" x="140" %}
{% node #metadata-2 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Token Standard = FungibleAsset" /%}
{% /node %}

{% node parent="mint-2" y="120" %}
{% node #mint-3 label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Decimals > 0" /%}
{% /node %}
{% node parent="mint-3" y="-20" x="-10" label="Fungible" theme="transparent" /%}

{% node parent="mint-3" x="220" #metadata-3-pda label="PDA" theme="crimson" /%}
{% node parent="metadata-3-pda" x="140" %}
{% node #metadata-3 label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="Token Standard = Fungible" /%}
{% /node %}

{% edge from="mint-1" to="metadata-1-pda" path="straight" /%}
{% edge from="metadata-1-pda" to="metadata-1" path="straight" /%}
{% edge from="mint-1" to="master-edition-pda" /%}
{% edge from="master-edition-pda" to="master-edition" path="straight" /%}
{% edge from="master-edition-pda" to="edition" label="OR" /%}

{% edge from="mint-2" to="metadata-2-pda" path="straight" /%}
{% edge from="metadata-2-pda" to="metadata-2" path="straight" /%}
{% edge from="mint-3" to="metadata-3-pda" path="straight" /%}
{% edge from="metadata-3-pda" to="metadata-3" path="straight" /%}
{% /diagram %}

## Programmable NFTs {% #pnfts %}

Because the Token Metadata program is building on top of the Solana Token program, anyone can transfer tokens (fungible or not) without going through the Token Metadata program. Whilst this is great for program composibility, it also means that the Token Metadata program cannot enforce any rules on the tokens it is attached to.

A good example of why this can be problematic is that Token Metadata cannot enforce secondary sales royalties. Whilst there is **Seller Fee Basis Points** attribute on the Metadata account, it is purely [indicative](/understanding-programs#indicative-fields) and anyone could create a marketplace that does not honor royalties — which is exactly what happened.

**Programmable NFTs** were introduced to solve this problem. They are a new _opt-in_ Token Standard that **keeps the underlying token accounts frozen at all times**. That way, nobody can transfer, lock or burn Programmable NFTs without going through the Token Metadata program.

It is then up to the creators to define custom operation-specific authorization rules that will be enforced by the Token Metadata program. These are defined in a special **RuleSet** account which is attached to the Metadata account. An example of such RuleSet could be an allowlist of program addresses that honor royalties. RuleSets are part of a new Metaplex program called [Token Auth Rules](/token-auth-rules).

You can [read more about Programmable NFTs here](/token-metadata/pnfts).

{% diagram %}
{% node %}
{% node #wallet label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node x="200" parent="wallet" %}
{% node #token label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="State = Frozen" /%}
{% /node %}

{% node x="200" parent="token" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% /node %}

{% node #metadata-pda parent="mint" x="41" y="-120" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-230" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% node label="..." /%}
{% node #programmable-configs label="Programmable Configs" /%}
{% /node %}

{% node parent="metadata" x="-260" y="0" %}
{% node #ruleset label="RuleSet Account" theme="crimson" /%}
{% node label="Owner: Token Auth Rules Program" theme="dimmed" /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" path="straight" /%}
{% edge from="metadata-pda" to="metadata" path="straight" /%}
{% edge from="programmable-configs" to="ruleset" arrow="none" dashed=true /%}
{% /diagram %}

## And a lot more

Whilst this provides a good overview of the Token Metadata program and what it has to offer, there’s still a lot more that can be done with it.

The other pages of this documentation aim to document it further and explain significant features in their own individual pages.

- [Token Standards (Assets)](/token-metadata/token-standard)
- [Minting Assets](/token-metadata/mint)
- [Updating Assets](/token-metadata/update)
- [Transferring Assets](/token-metadata/transfer)
- [Burning Assets](/token-metadata/burn)
- [Printed Editions](/token-metadata/print)
- [Verified Collections](/token-metadata/collections)
- [Verified Creators](/token-metadata/creators)
- [Delegated Authorities](/token-metadata/delegates)
- [Locking Assets](/token-metadata/lock)
- [Programmable NFTs](/token-metadata/pnfts)
- [NFT Escrow](/token-metadata/escrow)
