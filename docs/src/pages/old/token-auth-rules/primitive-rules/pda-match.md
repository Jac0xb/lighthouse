---
title: PDA Match
metaTitle: Token Auth Rules - PDA Match
description: The PDA Match primitive rule
---

## PDA Match
Performs a PDA derivation using `find_program_address()` and the associated Payload and Rule fields. This Rule evaluates to true if the PDA derivation matches the Payload address.

### Fields
* **program** - The Program from which the PDA is derived
* **pda_field** - The field in the Payload which the derived address much match for the Rule to evaluate to true
* **seeds_field** - The field in the Payload which stores an Array of PDA seeds to use for derivation

```js
// This Rule Set will only evaluate to true the derived PDA from the provided seeds matches the provided PDA.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    list: {
      type: 'PdaMatch',
      pdaField: "Escrow",
      program: publicKey("TSWAPaqyCSx2KABk68Shruf4rp7CxcNi8hAsbdwmHbN"),
      seedsField: "EscrowSeeds",
    },
  },
}
```