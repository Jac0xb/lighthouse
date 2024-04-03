---
title: Overview
metaTitle: Lighthouse - Overview
description:
---

## Assert Instruction Types

The assert instructions for Lighthouse are the main building blocks for asserting on runtime state within the SVM while a transaction is being processed.

There are assertions such as `AssertAccountInfo`, `AssertAccountData`, and `AssertAccountDelta` which are used to make generic assertions about onchciain state.

There are assertions that are more specific assertions which combine multiple generic assertions to save transaction space and compute such as `AssertMintAccount`, `AssertTokenAccount`, and `AssertStakeAccount`.

[AssertAccountInfo](/assert/account-info) (Make assertions about account info)

[AssertAccountData](/assert/account-data) (Make assertions about account data)

[AssertAccountDelta](/assert/account-delta) (Make assertions about two accounts)

[AssertMintAccount](/assert/mint-account) (Make assertions about mint accounts)

[AssertTokenAccount](/assert/token-account) (Make assertions about token accounts)

[AssertStakeAccount](/assert/stake-account) (Make assertions about stake accounts)

[AssertSysvarClock](/assert/sysvar-clock) (Make assertions about the sysvar clock)

## Assert Multi Instruction Types

To save on transaction space and compute there are instructions which have the same logic as the single assertion instructions but take a vector of assertions instead. This elimiates duplicating instruction data: program id (u8), target account (u8), instruction disciminator (u8), shortvecs for instruction accounts and data, and the compute unit overhead of program entry per instruction.

{% callout %}
The error code for a failed multi-assertion instruction is different than for a normal assertion instruction. To calculate the assertion there are helper methods in the sdks. The error code starts at `0x1900 + index of failed assertion`. This should make it easy to parse which assertion failed in the assertions vector.
{% /callout %}
