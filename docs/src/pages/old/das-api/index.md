---
title: DAS API
metaTitle: DAS API - Overview
description: A DAS API client used to access the Metaplex Digital Asset Standard.
---

The state data of uncompressed NFTs is all stored in on-chain accounts. This is expensive at scale. Compressed NFTs save space by encoding the state data into an on-chain Merkle tree. The detailed account data is not stored on-chain, but in data stores managed by RPC providers. The Metaplex Digital Asset Standard (DAS) API represents a unified interface for interacting with digital assets on Solana, supporting both standard (Token Metadata) and compressed (Bubblegum) assets.

The API defines a set of methods that RPCs implement in order to provide asset data. In the majority of cases, the data is indexed using Metaplex Digital Asset RPC infrastructure.

{% quick-links %}

{% quick-link title="Getting Started" icon="InboxArrowDown" href="/das-api/getting-started" description="Find the language or library of your choice and get started essentials programs." /%}

{% quick-link title="Methods" icon="CodeBracketSquare" href="/das-api/methods" description="DAS API methods for fetching data." /%}

{% /quick-links %}
