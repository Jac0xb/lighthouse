---
title: Create or Update Rule Sets
metaTitle: Token Auth Rules - Create/Update
description: How to Create and Update Rule Sets
---

## Introduction

A Token Authorization Rules Rule Set is a collection of [Composite Rules](/token-auth-rules/composite-rules) and [Primitive Rules](/token-auth-rules/primitive-rules) stored in a PDA owned by the Token Auth Rules program.

## Creating or Updating a Rule Set

A Rule Set is created and updated through a call to the same instruction, **CreateOrUpdate**. If the passed in PDA is uninitialized the program will create it, otherwise it will update the Rule Set with the passed in Rule Set data as a new revision. The following parameters must be passed in:

- **payer** - The authority of the Rule Set and payer of the rent fees.
- **ruleSetPda** - The PDA in which the Rule Set will be stored. The PDA uses "rule_set_state", **payer**, and **rule_set_name** as derivation seeds. The **rule_set_name** can be any string under 32 characters.
- **systemProgram** - The system program.
- **ruleSetRevision** - The serialized data for the Rule Set.

```ts
import {
  RuleSetRevisionV2,
  createOrUpdateV1,
  findRuleSetPda,
  programOwnedV2,
} from '@metaplex-foundation/mpl-token-auth-rules';

const owner = umi.identity;
const program = generateSigner(umi).publicKey;
const name = 'transfer_test';
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name,
  owner: owner.publicKey,
  operations: {
    Transfer: programOwnedV2('Destination', program),
  },
};

// When we create a new rule set account using this data.
const ruleSetPda = findRuleSetPda(umi, { owner: owner.publicKey, name });
await createOrUpdateWithBufferV1(umi, {
  payer: owner,
  ruleSetPda,
  ruleSetRevision: some(revision),
}).sendAndConfirm(umi);
```

## Resources

- [Token Auth Rule GitHub repository](https://github.com/metaplex-foundation/mpl-token-auth-rules)
- [TypeScript references for the JS client](https://mpl-token-auth-rules-js-docs.vercel.app/)
