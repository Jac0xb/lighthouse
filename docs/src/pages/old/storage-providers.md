---
title: Storage Providers
metaTitle: Metaplex — Storage Providers
description: A list of available Storage Providers on Solana.
---

NFTs are meant to live forever, being bought, sold, held and enjoyed for the
rest of time. As such, the storage on which your assets are written should be
_permanent_. NFTs created through Metaplex are written to scalable, durable,
permanent, censorship resistant storage by default, but there are other options
available too. Let's take a look.

## Storage options

### Arweave

[Arweave][] is a decentralized, trust-minimized, censorship-resistant data
storage network designed to retain data permanently, making it a great fit for
NFTs. To cover the cost of storing your media forever, storage and mining fees
are paid at the time of upload and distributed to storage providers
participating in the network.

#### Arweave storage fees

Storage fees are based on the total size of the files you upload to the network during NFT creation. Each NFT consists of three files:

1. The asset itself (image, video, audio, etc)
1. The accompanying metadata file (attributes etc.)
1. A generated [manifest which creates a logical grouping][arweave path manifest] or relationship between your files

The cumulative size of these files (in bytes) is submitted to the [Arweave
storage cost estimation service][arweave price service] which returns the real
time estimated fee for storage, priced in [winstons][]. We then convert the
winstons to SOL for payment.

### AWS S3

[Amazon Web Services S3][S3] is a global, affordable but centralized storage
provider. Because S3 is centralized, NFTs stored there are not censorship
resistant. If AWS receives legal threats, decides they no longer support NFTs,
goes out of business, or you stop making payments, they can remove your assets
from their service, potentially leaving holders of your NFTs without their
media. For NFTs which are meant to be censorship resistant and permanent, we do
not recommend using S3. However, it is an affordable option so depending on your
needs may be what you need.

#### S3 storage fees

Please visit [https://aws.amazon.com/s3/pricing/](https://aws.amazon.com/s3/pricing/) for details.

### IPFS

The [InterPlanetary File System][IPFS], or IPFS, is a decentralized,
trust-minimized, censorship-resistant, peer-to-peer hypermedia protocol designed
to preserve and grow humanity's knowledge by making the web upgradeable,
resilient, and more open. It's P2P design allows for file deduplication and
other efficiencies. IPFS is not designed to store files permanently and so is
therefore not the default storage option.

#### IPFS storage fees

Please visit [https://infura.io/docs/ipfs](https://infura.io/docs/ipfs) for details.

### NFT.Storage

[NFT.Storage](https://nft.storage) is a free service that provides long-term NFT data storage on the decentralized [Filecoin](https://filecoin.io) network with fast retrieval through [IPFS][IPFS].

NFT.Storage currently supports files up to 31 Gib and does not charge for storage.

### Shadow Drive

The [GenesysGo Shadow Drive](https://shdw.genesysgo.com/shadow-infrastructure-overview/shadow-drive-overview) is a decentralized storage network designed to run in parallel to, and integrate seamlessly with, Solana. Storage fees are paid in the SPL token, SHDW, and allow immutable or mutable storage of data.

[Arweave]: https://arweave.org
[arweave price service]: https://node1.bundlr.network/price/0
[repo]: https://github.com/metaplex-foundation/metaplex
[IPFS]: https://ipfs.io/
[winstons]: https://docs.arweave.org/developers/server/http-api#ar-and-winston
[S3]: https://aws.amazon.com/s3/
[arweave path manifest]: https://github.com/ArweaveTeam/arweave/wiki/Path-Manifests
[nft.storage metaplex doc]: https://nft.storage/docs/how-to/mint-solana
