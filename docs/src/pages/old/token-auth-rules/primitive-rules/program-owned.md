---
title: Program Owned
metaTitle: Token Auth Rules - Program Owned
description: The Program Owned primitive rule
---

## Program Owned

Checks that the Program indicated owns the account. This is useful for PDAs which are typically always owned by the program they are derived from (e.g. marketplaces and utility programs).

### Fields

- **program** - The Program that must own the account specified in the field
- **field** - The field in the Payload to check the owner of

```js
// This Rule Set will only evaluate to true if PDA in the specified field is owned by the program indicated.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    list: {
      type: 'ProgramOwned',
      field: 'Escrow',
      program: publicKey('TSWAPaqyCSx2KABk68Shruf4rp7CxcNi8hAsbdwmHbN'),
    },
  },
}
```

## Program Owned List

The version of [Program Owned](#program-owned) that compares against a list of possible owning Programs.

### Fields

- **programs** - A vector of Programs, one of which must own the account specified in the field
- **field** - The field in the Payload to check the owner of

```js
// This Rule Set will only evaluate to true if PDA in the specified field is owned by one of the programs indicated.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    list: {
      type: 'ProgramOwnedList',
      field: 'Escrow',
      programs: [
        publicKey('TSWAPaqyCSx2KABk68Shruf4rp7CxcNi8hAsbdwmHbN'),
        publicKey('M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K'),
      ],
    },
  },
}
```

## Program Owned Tree

The version of [Program Owned](#program-owned) that compares against a merkle tree of possible owning Programs.

### Fields

- **pubkey_field** - The field in the Payload to check the owner of
- **proof_field** - The field in the payload that contains the full merkle proof to be hashed
- **root** - The root of the merkle tree

```js
// This Rule Set will only evaluate to true if PDA and proof provided hash to the correct merkle root.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    list: {
      type: 'ProgramOwnedTree',
      pubkeyField: "Escrow",
      proofField: "EscrowProof",
      root: [229, 0, 134, 58, 163, 244, 192, 254, 190, 193, 110, 212, 193, 145, 147, 18, 171, 160 213, 18, 52, 155, 8, 51, 44, 55, 25, 245, 3, 47, 172, 111],
    },
  },
}
```
