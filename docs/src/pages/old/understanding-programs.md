---
title: Understanding Programs
metaTitle: Metaplex — Understanding Programs
description: A quick overview of how programs work on Solana.
---

This page aims to provide a quick overview of how programs work in Solana and offer additional reading material for those who are interested in learning more about a particular subject. {% .lead %}

## Introduction

Unlike most blockchains, **Solana separates logic and data** into two separate components. These are, respectively, **Programs** and **Accounts**. What that means is that instead of storing data inside variables internally, Programs interact with external data stored in Accounts with the ability to mutate them.

This architecture is great for making Programs more modular since the data they interact with is not bound by the Program itself and can scale to new orders of magnitude. It is also great for making Programs more performant since it allows the blockchain to run the same Program in parallel with different Accounts.

To interact with a Program, we must use the **Instructions** defined by that Program. You can think of Instructions as API endpoints exposed by the Program. Each Instruction contains a set of parameters and constraints that must be fulfilled to execute it.

To recap: **In Solana, Programs define Instructions that can be used to interact with external data stores called Accounts**.

Note that, technically, Programs are special kinds of Accounts marked as `executable` whose entire purpose is to store the compiled code of the Program. However, for the sake of simplicity, we will distinguish these definitions and use the term "Account" to refer to non-executable Accounts.

In the rest of this guide, we will talk about Accounts and Instructions in more detail before explaining the visual representation that we will be using in diagrams throughout the documentation.

📚 **Additional reading**:

- [Solana Documentation — On-chain Programs](https://docs.solana.com/developing/on-chain-programs/overview)
- [Solana Cookbook — Programs](https://solanacookbook.com/core-concepts/programs.html)
- [The Anchor Book — Intro to Programming on Solana](https://book.anchor-lang.com/prerequisites/intro_to_solana.html)

## Accounts

In Solana, **Accounts are used to store data**. In essence, they are simple arrays of bytes stored at a particular address. The address of an Account is the **public key** of a cryptographic key pair.

Anyone that has access to the **private key** of that key pair, can sign on behalf of that Account which, depending on the program, may give them the ability to mutate the data stored in that Account.

Once an Account is created, it is usually immediately initialized by a Program which will be marked as its **Owner** and will define the data structure of the allocated array of bytes. The Program that owns the Account is then responsible for providing Instructions that can be used to interact with it.

📚 **Additional reading**:

- [Solana Documentation — Accounts](https://docs.solana.com/developing/programming-model/accounts)
- [Solana Cookbook — Accounts](https://solanacookbook.com/core-concepts/accounts.html)

### Program Derived Addresses (PDA)

There exist another type of Account, called **Program Derived Account**, whose address is not the public key of a cryptographic key pair but instead is algorithmically derived from the public key of the Program that owns the Account. We call that address a **Program Derived Address** or **PDA** for short.

Since the address is always derived from the public key of the Program, no other Program can algorithmically derive the same address. On top of that, additional **Seeds** can be provided to the algorithm to add more context to the address.

This has a variety of use cases such as enabling programs to sign [Cross-Program Invocations](#cross-program-invokations) or enabling the creation of accounts within an address that can be derived deterministically.

Note that, by design, Program Derived Addresses will never conflict with cryptographically generated public keys. All cryptographic public keys are part of what we call an [Elliptic-curve](https://en.wikipedia.org/wiki/Elliptic-curve_cryptography). If, when generating a PDA, the algorithm generated a key that falls on that curve, a **Bump** is added to the address and is incremented by one until the generated address no longer falls on the curve.

📚 **Additional reading**:

- [Solana Documentation — PDAs](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses)
- [Solana Cookbook — PDAs](https://solanacookbook.com/core-concepts/pdas.html)

### Account data

Whether we are dealing with a regular Account or a Program Derived Account, Accounts store data as a serialized array of bytes. Therefore, it is the responsibility of the Program to define a data structure for each of the Accounts it manages as well as provide a way of differentiating these Accounts, so we know which data structure to apply to them.

### Discriminators

Discriminators are used to differentiate between different types of Accounts within a Program. They can be implemented in many ways but here are the three most common ones:

- **Use a shared Enum as the first byte of every account**. By prefixing every Account with a shared Enum, we can use the first byte of the serialized data to identify the Account. This is a simple and efficient way to implement discriminators. Most of the programs maintained by Metaplex use this approach.
- **Use a deterministic hash as the first byte of every account**. This is very similar to the previous point, but it uses a hash instead of an Enum. Programs created using the Anchor framework end up using this approach implicitly because Anchor will automatically generate that hash based on the Account's name.
- **No discriminator, use the size of the Account**. If all the accounts managed by a Program have different sizes, then we can examine the length of that array of bytes to determine which Account we are dealing with. This is a performant approach since we don't need to add extra bytes to the data, but it limits how flexible a Program can be with its Accounts. The [SPL Token Program](https://spl.solana.com/token) by Solana uses this approach since it only maintains two accounts of different fixed sizes.

### Field types, sizes and offsets

Each Account defines its own data structure by using fields of different types. These types will affect the number of bytes required to store the field. For instance, an `i8` is an 8-bit integer that will require 1 byte to store whereas an `i64` is a 64-bit integer which will require 8 bytes to store.

Since, in the blockchain, Accounts are just arrays of bytes, it is important to understand the size of each field and where they start in this array, i.e. their offset. This can be useful when fetching multiple accounts from a given program [using a `memcmp` filter](https://solanacookbook.com/guides/get-program-accounts.html#memcmp).

Note that not all fields have a fixed size. For instance, a `Vec<i8>` is a vector of 8-bit integers that may contain none, one or many items. As such, it becomes a lot more complicated to filter accounts based on fields that are located after the first field of variable size.

### Optional fields

A field can also be defined as **optional**, meaning there exists a scenario where that field can be empty.

This field will use an additional byte as a prefix to indicate whether the field is empty or not.

Concretely, the value `None` will be assigned and the program will act accordingly when using that field.

### Indicative fields

Whilst this is not something that is explicitly defined in the data structure, the documentation will mark certain fields as **indicative**.

An indicative field means that the information provided by the field is not used by the program itself. Instead, it _indicates_ a piece of information to third parties. The program will still enforce the integrity of the data, but it will simply not use that information internally.

Let’s take the Metadata Account as an example.

The `Share` property of each creator in the `Creators` array is indicative. The Token Metadata program will ensure that the `Share` values of all creators add up to 100%, but it will not do anything with that information. Instead, it expects NFT marketplaces to use this information when distributing royalties.

On the other hand, the `Is Mutable` property is not indicative because the Token Metadata program will use that information internally to prevent immutable Metadata Accounts to be updated.

## Instructions

One can interact with a Program using the **Instructions** it provides. Multiple Instructions can be packed into a single **Transaction** that will be sent to the blockchain. Each Transaction is atomic meaning if any of its instructions fails, the whole Transaction will be reverted.

Similarly to Accounts, Instructions must be serialized into an array of bytes before they can be sent to the network. The data to be serialized must contain the following information for the Program to execute it.

- **Discriminator**: Similarly to Accounts, Instructions are usually prefixed with a discriminator so the Program can identify which Instruction is being executed.
- **Accounts**: An array of Account addresses that are affected by this instruction. This can either be because the Account will be read, mutated or both. Note that the order of this array is important since Programs will identify the type of Account provided based on its position.
- **Arguments**: An array of data fields required by the instruction. It is not uncommon for this array to be empty since Instructions can get most of their information directly from the Accounts. Note that these arguments are comparable to the fields of an Account and, therefore, they can have the same properties mentioned above such as "[optional](#optional-fields)" and "[indicative](#indicative-fields)".
- **Signers**: An array of signatures for a sub-set of the Accounts provided. This is only needed for Accounts that are required to sign the Instruction. The next section explains this in a bit more detail.

📚 **Additional reading**:

- [Solana Documentation — Transactions](https://docs.solana.com/developing/programming-model/transactions)
- [Solana Documentation — Instructions](https://docs.solana.com/developing/programming-model/transactions#instructions)
- [Solana Cookbook — Transactions](https://solanacookbook.com/core-concepts/transactions.html)

### Signer and/or Writable Accounts

A Program may require that the Accounts provided within an Instruction are **Signers** and/or **Writable**.

- **Signers**: A Signer Account is required to sign the Transaction for the Instruction to be successful. By attaching a signature, users can prove that they are the owner of the Account.
- **Writable**: A Writable Account will be mutated by the Instruction. This information is important for the blockchain to know which Transactions can be run in parallel and which ones can't.

Therefore, with these two booleans, we end up with the following four possible scenarios:

- **Non-Signer and Non-Writable**: This Account is only used to read data. We cannot mutate it and we cannot make any assumption about its ownership.
- **Signer and Non-Writable**: This Account can also not be mutated, but we know that the user who sent the Transaction owns its private key. This enables Programs to grant or deny access to certain actions.
- **Signer and Writable**: This Account has both signed the Transaction _and_ it can be mutated by the Instruction. This combination is pretty common since Programs will usually require the owner of an Account to prove who they are before mutating that account. Otherwise, anyone could mutate any Account they don't own.
- **Non-Signer and Writable**: This Account can be mutated, but we can't make any assumption about its ownership. That usually means that the Program is using other Signer Accounts to prove they can mutate that one. This is also the case for PDA Accounts since they are owned by the Program and, as such, they require the Program to keep track of Authorities that can mutate them. Also, note that certain actions like crediting lamports to an Account do not require the Account to sign the Transaction.

### Cross-Program Invocations (CPI)

Cross-Program Invocations allow Programs to execute nested Instructions within their Instructions. They can use Instructions from their own Program and/or from other Programs.

📚 **Additional reading**:

- [Solana Documentation — CPI](https://docs.solana.com/developing/programming-model/calling-between-programs#cross-program-invocations)
- [Using CPIs with Anchor Programs](https://book.anchor-lang.com/anchor_in_depth/CPIs.html)
