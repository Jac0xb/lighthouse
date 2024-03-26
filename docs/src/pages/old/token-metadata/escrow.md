---
title: NFT Escrow
metaTitle: Token Metadata - NFT Escrow
description: Learn more about the Token Owned Escrow feature of Token Metadata
---

This extension of the Token Metadata contract was created as a new feature primitive that could optionally be added to all NFTs. At its core it is simply an escrow account attached to an NFT, enabling NFTs to become owners of other tokens. {% .lead %}

Aside from the requisite security and ownership checks necessary, the functionality this feature affords has been left generic enough to allow users to implement whatever they desire on top of the composability of the token and its escrow account.

## Types of Escrow Accounts

It is currently possible to create two different types of escrow accounts on an NFT: A Token Owned Escrow (TOE) used for ownership and a Creator Owned Escrow (COE) used for associations. The existence of both types of escrow accounts is not mutually exclusive.

### Token Owned Escrow

A Token Owned Escrow account, or TOE, is an escrow account attached to the NFT that is managed by the holder of the NFT. Transferring a token out of this escrow account is only allowable by the NFT's holder and the permissions follow the NFT as it is transferred between wallets.

This means Alice can add a token to a TOE on her NFT, then sell her NFT to Bob. Bob would then be the only one allowed to transfer that token out of the TOE.

Due to the holder-based permissions, it is only possible to have one TOE on an NFT. A TOE follows traditional wallet action flow where anyone can transfer a token into the escrow account but the holder is the only one who can transfer the token out.

### Creator Owned Escrow

A Creator Owned Escrow, or COE, is an escrow account attached to the NFT that is managed by a specified creator. This escrow account allows creators to make associations between tokens that they themselves can manage, regardless of sales, transfers, and holders of the base NFT.

An example use case for this is Metaverse avatars. Rather than storing avatars on a Web2 server, the Metaverse team could mint the avatar as an NFT, then put it in a COE (that the Metaverse team manages) attached to the corresponding base NFT. Because usage of the COE is locked to the creator of the COE, a holder would be unable to transfer the avatar out of the escrow account and break the association.

A Creator Owned Escrow is seeded with the creator public key and therefore there is no limit to the number of COEs that can be attached to an NFT. This allows many associations to be built across many projects. A COE follows traditional wallet action flow where anyone can transfer a token into the escrow account but the creator of the escrow is the only one who can transfer the token out.
