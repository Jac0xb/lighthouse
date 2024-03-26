---
title: Public Key
metaTitle: Token Auth Rules - Public Key
description: The Public Key primitive rule
---

## Pubkey Match
Checks that the Pubkey specified matches a specific Pubkey. For example, this rule can be used when only a certain person should be granted access to perform operations on an NFT.

### Fields
* **pubkey** - The public key to be compared against
* **field** - The field specifying which Pubkey in the Payload to check

```js
// This Rule Set will only evaluate to true if transfer destination matches the Public Key.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    transfer: {
      type: 'PubkeyMatch',
      field: 'Destination',
      publicKey: publicKey('DhYCi6pvfhJkPRpt5RjYwsE1hZw84iu6twbRt9B6dYLV'),
    },
  },
}
```

## Pubkey List Match
The version of [PubkeyMatch](#pubkey-match) that checks that the Pubkey is contained in a the list of possible Pubkeys. For example, this rule can be used for building an allowlist of users who are allowed to interact with a token.

### Fields
* **pubkeys** - The list of public keys to be compared against
* **field** - The field specifying which Pubkey in the Payload to check

```js
// This Rule Set will only evaluate to true if transfer destination matches one of the Public Keys.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    transfer: {
      type: 'PubkeyListMatch',
      field: 'Destination',
      publicKeys: [publicKey('DhYCi6pvfhJkPRpt5RjYwsE1hZw84iu6twbRt9B6dYLV'), publicKey('6twkdkDaF3xANuvpUQvENSLhtNmPxzYAEu8qUKcVkWwy')],
    },
  },
}
```

## Pubkey Tree Match
The version of [PubkeyMatch](#pubkey-match) that checks that the Pubkey is contained in a merkle tree of possible Pubkeys. For example, this rule can be used for building a very large allowlist of users who are allowed to interact with a token.

### Fields
* **pubkey_field** - The field in the Payload which contains the pubkey to check
* **proof_field** - The field in the payload that contains the full merkle proof to be hashed
* **root** - The root of the merkle tree

```js
// This Rule Set will only evaluate to true if transfer destination and proof hash to the merkle root.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    transfer: {
      type: 'PubkeyTreeMatch',
      pubkeyField: 'Destination',
      proofField: 'DestinationProof',
      root: [229, 0, 134, 58, 163, 244, 192, 254, 190, 193, 110, 212, 193, 145, 147, 18, 171, 160 213, 18, 52, 155, 8, 51, 44, 55, 25, 245, 3, 47, 172, 111],
    },
  },
}
```