---
title: Hashing NFT Data
metaTitle: Bubblegum - Hashing NFT Data
description: Learn more about how NFT data is hashed on Bubblegum
---

In previous sections we stated that each leaf node in a Bubblegum Merkle tree is obtained by hashing the data of the compressed NFT (cNFT).  But how exactly is this done?  We start with the metadata for the cNFT.  Each cNFT is minted with the following metadata structure as an argument to the minting instruction:

```rust
pub struct MetadataArgs {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    /// Whether or not the data struct is mutable, default is not
    pub is_mutable: bool,
    /// nonce for easy calculation of editions, if present
    pub edition_nonce: Option<u8>,
    /// Since we cannot easily change Metadata, we add the new DataV2 fields here at the end.
    pub token_standard: Option<TokenStandard>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
    /// Which token program version (currently only `TokenProgramVersion::Original`` is supported).
    pub token_program_version: TokenProgramVersion,
    /// The array of creators of the cNFT.
    pub creators: Vec<Creator>,
}
```

The cNFT's metadata is hashed multiple times as shown in the diagram and described below:

{% diagram %}

{% node %}
{% node #metadata label="Metadata Args" theme="blue" /%}
{% node label="Name" /%}
{% node label="Symbol" /%}
{% node label="URI" /%}
{% node label="Seller Fee Basis Points" /%}
{% node label="Primary Sale Happened" /%}
{% node label="Is Mutable" /%}
{% node label="Edition Nonce" /%}
{% node label="Token Standard" /%}
{% node label="Collection" /%}
{% node label="Uses" /%}
{% node label="Token Program Version" /%}
{% node label="Creators" /%}
{% /node %}

{% node #seller-fee-basis-points parent="metadata" y="305" label="Seller Fee Basis Points" theme="blue" /%}

{% node #creators parent="metadata" y="370" label="Creators" theme="blue" /%}

{% node parent="metadata" x="300" y="150" %}
{% node #data-hash label="Data Hash" theme="mint" /%}
{% node theme="transparent" %}
Hash(Metadata Args, \
Seller Fee Basis Points)
{% /node %}
{% /node %}

{% node parent="creators" x="300" %}
{% node #creator-hash label="Creator Hash" theme="mint" /%}
{% node theme="transparent" label="Hash(Creators)" /%}
{% /node %}

{% node parent="data-hash" x="250" %}
{% node #leaf-schema label="Leaf Schema" theme="blue" /%}
{% node label="ID" /%}
{% node label="Owner" /%}
{% node label="Delegate" /%}
{% node label="Nonce" /%}
{% node label="Data Hash" /%}
{% node label="Creator Hash" /%}
{% /node %}

{% node parent="leaf-schema" x="200" %}
{% node #leaf-node label="Leaf Node" theme="mint" /%}
{% node theme="transparent" label="Hash(Leaf Schema)" /%}
{% /node %}

{% edge from="metadata" to="data-hash" /%}
{% edge from="seller-fee-basis-points" to="data-hash" /%}
{% edge from="creators" to="creator-hash" /%}

{% edge from="data-hash" to="leaf-schema" /%}
{% edge from="creator-hash" to="leaf-schema" /%}

{% edge from="leaf-schema" to="leaf-node" /%}

{% /diagram %}

First the metadata is hashed, using the keccak-256 hash function.  Keccak-256 is much stronger than SHA-256, and is used in Solana as well as other blockchains such as Ethereum.

Note that the metadata is hashed, and then hashed again with the `seller_fee_basis_points`.  This makes it easier for marketplaces to validate seller fee basis points, because they do not have to pass a full `MetadataArgs` struct around (which can be up to 457 bytes in length).  Instead, they can pass a 32-byte array of already-hashed `MetadataArgs`, and the `u16` (2 bytes) `seller_fee_basis_points`, and by hashing them together they can recreate the data hash.

```rust
/// Computes the hash of the metadata.
///
/// The hash is computed as the keccak256 hash of the metadata bytes, which is
/// then hashed with the `seller_fee_basis_points`.
pub fn hash_metadata(metadata: &MetadataArgs) -> Result<[u8; 32]> {
    let hash = keccak::hashv(&[metadata.try_to_vec()?.as_slice()]);
    // Calculate new data hash.
    Ok(keccak::hashv(&[
        &hash.to_bytes(),
        &metadata.seller_fee_basis_points.to_le_bytes(),
    ])
    .to_bytes())
}
```

Next, the creator array is hashed individually.

```rust
/// Computes the hash of the creators.
///
/// The hash is computed as the keccak256 hash of the creators bytes.
pub fn hash_creators(creators: &[Creator]) -> [u8; 32] {
    // convert creator Vec to bytes Vec
    let creator_data = creators
        .iter()
        .map(|c| [c.address.as_ref(), &[c.verified as u8], &[c.share]].concat())
        .collect::<Vec<_>>();
    // computes the hash
    keccak::hashv(
        creator_data
            .iter()
            .map(|c| c.as_slice())
            .collect::<Vec<&[u8]>>()
            .as_ref(),
    )
    .to_bytes()
}
```

The data hash and creator hash are added to a leaf schema along with other information needed to uniquely identify the leaf.

The separation of data and creator hashes is done for a similar reason as `seller_fee_basis_points` - if a marketplace wants to validate a creator array, it can pass around a 32-byte array of already-hashed `MetadataArgs` along with the creator array.  The values in the creator array can be evaluated, and then hashed into the `creator_hash` and combined with the other existing information into the leaf schema.

```rust
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum LeafSchema {
    V1 {
        id: Pubkey,
        owner: Pubkey,
        delegate: Pubkey,
        nonce: u64,
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
    },
}
```

Other than data and creator hashes, the leaf schema contains the following other items:
* nonce: This is a "number used once" value that is unique for each leaf on the tree.  It is needed to ensure Merkle tree leaves are unique.  In practice it retrieved from off-chain indexers, similar to asset proofs.
* id - This asset ID is a PDA derived from a fixed prefix, the Merkle tree Pubkey, and the nonce.
* owner - The Pubkey of the cNFT owner, typically a user's wallet.
* delegate - The delegate for the cNFT.  By default this is the user's wallet, but can be set by the `delegate` Bubblegum instruction.

To create the 32-byte leaf node that exists on the Merkle tree, the entire leaf schema is hashed as follows:

```rust
impl LeafSchema {
    pub fn to_node(&self) -> Node {
        let hashed_leaf = match self {
            LeafSchema::V1 {
                id,
                owner,
                delegate,
                nonce,
                data_hash,
                creator_hash,
            } => keccak::hashv(&[
                &[self.version().to_bytes()],
                id.as_ref(),
                owner.as_ref(),
                delegate.as_ref(),
                nonce.to_le_bytes().as_ref(),
                data_hash.as_ref(),
                creator_hash.as_ref(),
            ])
            .to_bytes(),
        };
        hashed_leaf
    }
}
```

Bubblegum operations that involve changing a leaf (`transfer`, `delegate`, `burn`, etc.) will send a "before" and "after" hashed leaf node to spl-account-compression in order to validate the Merkle tree change.
