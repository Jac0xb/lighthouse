---
title: Overview
metaTitle: Lighthouse - Overview
description:
---

## Assert Instruction Types

The assert instructions for Lighthouse are the main building blocks of asserting on runtime state within the SVM during a transaction's execution. There are composable assertions such as AssertAccountInfo, AssertAccountData, AssertAccountDelta. There are more specific "helper" assertions which combine multiple generic assertions to save transaction space such as `AssertMintAccount`, `AssertTokenAccount`, and `AssertStakeAccount`.

<!-- (AssertAccountInfo) -->
<!-- (AssertAccountData) -->

## Assert Multi Instruction Types

To save transaction space there instructions which have the same logic as the single assertion instructions but take a vector of assertions. This elimiates duplicating instruction data: program id (u8), target account (u8), instruction disciminator (u8), and the compute unit overhead of program entry per assertion (3 bytes per instruction - 4 bytes for vector, ~500 CU per instruction).
Note: The error code is different than for a normal assertion. To calculate the assertion there are helper methods in the sdks: `0x1900 + index of failed assertion`. This is for indexers who want to know easily determine which assertion failed.
