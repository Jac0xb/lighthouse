---
title: Overview
metaTitle: Fusion - Overview
description: Provides a high-level overview of composable NFTs using Fusion.
---

Fusion is an NFT composability feature powered by the Trifle Program. {% .lead %}

The Trifle Program is built upon the Escrow extension of Token Metadata. It uses a Creator Owned Escrow, or COE, using a Trifle PDA as the creator and manager of the COE. Its purpose is to add on-chain tracking and composability around NFT ownership. Additionally, the ability to specify rules and effects around token ownership allows for complex ownership models to be implemented by creators.

🔗 **Helpful links:**

- [Token Metadata Escrow](https://github.com/metaplex-foundation/mpl-token-metadata/tree/main/programs/token-metadata/program/src/processor/escrow)
- [Fusion Program](https://github.com/metaplex-foundation/mpl-trifle/tree/master/programs/trifle)

Let's dig into the Trifle program in more details by looking at the accounts and instructions it offers.

## Accounts

### Escrow Constraint Model

A Constraint Model is a set of restrictions and requirements that can be evaluated to allow for transmission into and out of the Trifle account. On transfer, the contract will check against the constraint model to determine what checks need to be performed against the token being transferred to or from the TOE. One Constraint Model can serve many different NFTs and their Trifle accounts.

The Constraint Model can be viewed as a set of Constraints, defined as slots. Each slot consists of a Slot Name, the type of constraint (None/Collection/TokenSet), and the number of allowable tokens in the slot. Constraints are stored as a `HashMap` with the Key being the Slot Name and the Value being the Constraint Type and Token Limit.

### Trifle

The Trifle account is what tracks tokens owned by the COE on-chain. It also links to the Constraint Model being used. The Trifle account manages tokens as an internal HashMap which reflects the slot semantics of the Constraint Model.

## Instructions

### Create Escrow Constraint Model Account

Creates a Constraint Model that can be used for Trifle accounts.

### Create Trifle Account

Creates a Trifle Account to be used on an NFT. A mandatory Constraint Model account must be passed in on creation for the Trifle account to check against.

### Transfer In

Transfer a token into the Creator Owned Escrow managed by the Trifle account. While it is possible to do a standard spl-token transfer to the COE, using this instruction is the only way for the Trifle account to manage and track the owned tokens. This instruction also performs checks against the Constraint Model to verify that the token being transferred in is valid.

### Transfer Out

Transfer a token out of the Creator Owned Escrow managed by the Trifle account. This instruction also performs checks against the Constraint Model to verify that the token being transferred out is allowed to be removed.

### Add None Constraint to Escrow Constraint Model

Create a None Constraint in the Constraint Model. Slot name and number of allowable tokens in the slot are defined at this time.

### Add Collection Constraint to Escrow Constraint Model

Create a Collection Constraint in the Constraint Model. Slot name, allowable Collection and number of allowable tokens in the slot are defined at this time.

### Add Tokens Constraint to Escrow Constraint Model

Create a Collection Constraint in the Constraint Model. Slot name, allowable tokens and number of allowable tokens in the slot are defined at this time.

### Remove Constraint from Escrow Constraint Model

Remove a Constraint from the Constraint Model by specifying which slot to clear by name.
