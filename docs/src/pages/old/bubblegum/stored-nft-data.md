---
title: Storing and Indexing NFT Data
metaTitle: Bubblegum - Storing and Indexing NFT Data
description: Learn more about how NFT data is stored on Bubblegum
---

As mentioned in the [Overview](/bubblegum#read-api), whenever compressed NFTs (cNFTs) are created or modified, the corresponding transactions are recorded on-chain in the ledger, but the cNFT state data is not stored in account space.  This is the reason for the massive cost savings of cNFTs, but for convenience and usability, the cNFT state data is indexed by RPC providers and available via the **the Metaplex DAS API**.

Metaplex has created a [reference implementation](https://github.com/metaplex-foundation/digital-asset-rpc-infrastructure) of the DAS API, and some RPC providers use some or all of this code for their particular implementation, while other RPC providers have written their own.  See the ["Metaplex DAS API RPCs"](/bubblegum/rpcs) page for a list of other RPC providers that support the Metaplex DAS API.

The Metaplex reference implementation of the DAS API includes the following key items:
* A Solana no-vote validator - This validator is configured to only have secure access to the validator ledger and account data under consensus.
* A Geyser plugin - The plugin is called "Plerkle" and runs on the validator.  The plugin is notified whenever there are account updates, slot status updates, transactions, or block metadata updates.  For the purpose of cNFT indexing, the plugin's `notify_transaction` method is used to provide transaction data whenever Bubblegum or spl-account-compression transactions are seen on the validator.  In reality, these transactions are coming from the spl-noop ("no operation") program, which is used by spl-account-compression and Bubblegum to avoid log truncation by turning events into spl-noop instruction data.
* A Redis cluster - Redis streams are used as queues for the each type of update (account, transaction, etc.).  The Geyser plugin is the producer of data going into these streams.  The Geyser plugin translates the data into the Plerkle serialization format, which uses the Flatbuffers protocol, and then puts the serialized record into the appropriate Redis data stream.
* An ingester process - This is the the consumer of the data from the Redis streams.  The ingester parses the serialized data, and then transforms it into SeaORM data objects that are stored in a Postgres database.
 * Postgres database - There are several database tables to represent assets, as well as a changelog table to store the state of Merkle trees it has seen.  The latter is used when requesting an asset proof to be used with Bubblegum instructions. Sequence numbers for Merkle tree changes are also used to enable the DAS API to process transactions out of order.
* API process - When end-users request asset data from RPC providers, the API process can retrieve the asset data from the database and serve it for the request.

{% diagram %}
{% node %}
{% node #validator label="Validator" theme="indigo" /%}
{% node theme="dimmed" %}
Runs Geyser plugin \
and is notified on \
transactions, account \
updates, etc.
{% /node %}
{% /node %}

{% node x="200" parent="validator" %}
{% node #messenger label="Message bus" theme="blue" /%}
{% node theme="dimmed" %}
Redis streams as queues \
for each type of update.
{% /node %}
{% /node %}

{% node x="200" parent="messenger" %}
{% node #ingester label="Ingester process" theme="indigo" /%}
{% node theme="dimmed" %}
Parses data and stores \
to database
{% /node %}
{% /node %}

{% node x="28" y="150" parent="ingester" %}
{% node #database label="Database" theme="blue" /%}
{% node theme="dimmed" %}
Postgres \
database
{% /node %}
{% /node %}

{% node x="-228" parent="database" %}
{% node #api label="API process" theme="indigo" /%}
{% node theme="dimmed" %}
RPC provider runs the API\
and serves asset data to \
end users.
{% /node %}
{% /node %}

{% node x="-200" parent="api" %}
{% node #end_user label="End user" theme="mint" /%}
{% node theme="dimmed" %}
Calls getAsset(), \
getAssetProof(), etc.
{% /node %}
{% /node %}

{% edge from="validator" to="messenger" /%}
{% edge from="messenger" to="ingester" /%}
{% edge from="ingester" to="database" /%}
{% edge from="database" to="api" /%}
{% edge from="api" to="end_user" /%}

{% /diagram %}
