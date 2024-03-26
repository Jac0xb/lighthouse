---
title: Pass
metaTitle: Token Auth Rules - Pass
description: The Pass primitive rule
---

## Pass
This Rule always evaluates as true during validation.

```js
// This Rule Set will always evaluate to true.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    deposit: {
      type: 'Pass',
    },
  },
}
```