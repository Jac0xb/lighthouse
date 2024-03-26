---
title: Getting Started
metaTitle: Fusion - Getting Started
description: How to use Metaplex Fusion.
---

## What is Fusion?

Fusion is Metaplex's answer for composable NFTs. Fusion is itself a composition of several Metaplex programs that enable fully dynamic NFTs to be created by projects, artists, or collectors. At the contract level, Fusion is powered by Trifle which manages the on-chain tracking and rule-based fuse/defuse operations of an NFT.

## Steps for Setup

### Create a parent NFT

Fusion is structured as a single NFT (the Fusion Parent) that owns all of the attributes it is composed of. The Fusion Parent will dynamically have its Metadata and Image re-rendered to reflect the layering of all of the Attribute Tokens tracked in its on-chain Trifle account. To enable seamless recomposition of the metadata, a static URI is created using a deterministic format.

`https://shdw-drive.genesysgo.net/<METAPLEX_BUCKET>/<TRIFLE_ADDRESS>`

The dynamic metadata and image are hosted on GenesysGo's Shadow Drive technology to take advantage of their decentralized data hosting and updatable storage format. This static URI allows for backend updates of all data without requiring actual updates to the NFT's Metadata account, which is permissioned to only allow the Update Authority to make updates. This allows Fusion users to have dynamic metadata without sharing any private keys. An example of Fusion Parent creation is outlined below:

```tsx
const findTriflePda = async (mint: PublicKey, authority: PublicKey) => {
  return await PublicKey.findProgramAddress(
    [Buffer.from('trifle'), mint.toBuffer(), authority.toBuffer()],
    new PublicKey(PROGRAM_ADDRESS)
  )
}

const METAPLEX_BUCKET = 'Jf27xwhv6bH1aaPYtvJxvHvKRHoDe3DyQVqe4CJyxsP'
let nftMint = Keypair.generate()
let trifleAddress = await findTriflePda(nftMint.publicKey, updateAuthority)
let result
result = await metaplex!.nfts().create({
  uri:
    'https://shdw-drive.genesysgo.net/' +
    METAPLEX_BUCKET +
    '/' +
    trifleAddress[0].toString() +
    '.json',
  name: 'Fusion NFT',
  sellerFeeBasisPoints: 0,
  useNewMint: nftMint,
})
```

### Write Render Schema

Fusion utilizes the `schema` field of the Constraint Model account to determine the layer order to render the attributes in.

```json
{
  "type": "layering",
  "layers": ["base", "neck", "mouth", "nose"],
  "defaults": {
    "metadata": "https://shdw-drive.genesysgo.net/G6yhKwkApJr1YCCmrusFibbsvrXZa4Q3GRThSHFiRJQW/default.json"
  }
}
```

`type`: Defines what type of schema this represents and therefore how the backend server should render the Fusion Parent's image.
`layers`: An array of slot names on the Trifle account. The ordering of the array defines in what order the layers should be rendered. It is not a requirement to use all the layers, allowing for invisible attributes.
`defaults`: The default metadata to use as a baseline when combining the Fusion Parent's metadata. Metadata fields such `external_url` can then be included in the metadata in this way.

### Setup Trifle

Lastly, the Constraint Model and Trifle account should then be setup according to [these instructions](/programs/fusion/getting-started).

After the above steps, the Fusion Parent should be re-rendered after every `transfer_in` or `transfer_out` operation.
