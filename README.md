# Lighthouse - The Assertion Solana Program

![Lighthouse Logo](https://github.com/Jac0xb/lighthouse/assets/5273873/d5bb94be-a994-424f-88e0-32a917f07129)

[**Documentation at lighthouse.voyage**](https://lighthouse.voyage/)

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Use Cases](#use-cases)
  - [Guardrail Example](#guardrail-example)
  - [Additional Examples](#additional-examples)
- [Addresses](#addresses)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Building and Deploying a Verifiable Program](#building-and-deploying-a-verifiable-program)
- [Contributing](#contributing)
- [License](#license)
- [Disclaimer](#disclaimer)

## Introduction

Lighthouse is an open-source Solana program that provides assertion instructions to enhance transaction security. By appending assertion instructions to transactions, Lighthouse ensures that if a bad actor spoofs simulation results, there's overspending during the transaction, or an oracle account is in an undesired state, the assertion will fail, causing the entire transaction to fail. This eliminates the need to write new Solana programs for adding assertions to existing transactions.

Lighthouse emphasizes security (with upcoming features like multisig, verifiable build, non-upgradable releases), composability (program-agnostic use cases), and community involvement (open source contributions and integration support).

## Features

- **Assertion Instructions**: Primary instructions that allow transaction builders to assert on data accessed at runtime during the assertion instruction.
- **Write Instructions**: Secondary instructions to write account data, account info, and other runtime data into memory accounts, enabling assertion of inter-transaction state changes.
- **Memory Accounts**: Program Derived Accounts (PDAs) used to store runtime data, facilitating assertions on changes between instructions.

## Use Cases

### Guardrail Example

A wallet simulates that a token account balance changes from `100` to `90` during a transaction. It appends a Lighthouse assertion instruction to the transaction, asserting that the token account balance must be `90` at the end of the transaction. The assertion instruction is placed at the end of the transaction.

```rust
let tx = Transaction::new_signed_with_payer(
   &[
      blackhat_program
         .drain_token_account(
            user.encodable_pubkey(),
            drainer.encodable_pubkey(),
            mint.pubkey(),
         )
         .ix(),
      AssertTokenAccountBuilder::new()
         .target_account(user_ata)
         .assertion(TokenAccountAssertion::Amount {
            value: 90,
            operator: IntegerOperator::Equal,
         })
         .instruction(),
      AssertTokenAccountBuilder::new()
         .target_account(user_ata)
         .assertion(TokenAccountAssertion::Delegate {
            value: None,
            operator: EquatableOperator::Equal,
         })
         .instruction(),
   ],
   Some(&user.pubkey()),
   &[&user],
   context.get_blockhash().await,
);
```

_(From the testing library; `blackhat_program` is a test program designed to emulate existing drainer programs.)_

The transaction is then sent to the Solana blockchain. The assertion fails because the token balance is found to be `0` instead of `90` during the assertion instruction. Having failed the assertion, the Lighthouse program then fails the transaction.

### Additional Examples

- **Oracle Price Check**: A transaction builder includes a Lighthouse assertion to check a BONK USD price oracle account, asserting that the price must be above a certain value, or the transaction will fail.
- **Whitehat Exploit Demonstration**: In the game SVBonk, users send 1000 BONK to participate. Lighthouse can be used to demonstrate potential exploits by asserting the state after a transaction, helping developers identify and patch vulnerabilities.
- **Jito Bundle Guardrail**: A wallet provider builds a bundle of transactions that includes a Lighthouse assertion transaction built from expected simulation changes. The assertion transaction fails if unexpected state changes are detected, preventing bad actors from executing malicious transactions.

## Addresses

- **Devnet**: `L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95`
- **Mainnet Beta**: `L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95`

## Getting Started

### Prerequisites

- **pnpm**: Install via [pnpm installation guide](https://pnpm.io/7.x/installation) or use `npm install -g pnpm`.
- **Solana CLI**: Install version `1.18.8` from the [Solana documentation](https://docs.solana.com/cli/install-solana-cli-tools).

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/Jac0xb/lighthouse.git
   cd lighthouse
   ```

2. **Compile the Program**

   ```bash
   pnpm run programs:build
   ```

3. **Run Tests**

   ```bash
   pnpm run programs:test
   ```

4. **Generate Shank and Kinobi Clients**

   ```bash
   pnpm run generate
   ```

### Building and Deploying a Verifiable Program

#### Requirements

- **Solana Verify**: Install from [Solana Verifiable Build](https://github.com/Ellipsis-Labs/solana-verifiable-build).
- **Docker**: Install from [Docker's official site](https://www.docker.com/).
  - Ensure Docker has file system permissions to access the Lighthouse repository (enable in Docker settings and system settings).

#### Steps

1. **Build with Solana Verify**

   ```bash
   cd programs/lighthouse
   solana-verify build
   ```

2. **Get Executable Hash**

   ```bash
   solana-verify get-executable-hash target/deploy/lighthouse.so
   ```

3. **Deploy Program**

   ```bash
   solana program deploy target/deploy/lighthouse.so --with-compute-unit-price 500000
   ```

4. **Verify the Program**

   - **Normal Verification**

     ```bash
     solana-verify verify-from-repo -um \
     --program-id L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95 \
     https://github.com/jac0xb/lighthouse \
     --mount-path programs/lighthouse \
     --library-name lighthouse
     ```

   - **OtterSec Verification**

     ```bash
     solana-verify verify-from-repo -um \
     --program-id L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95 \
     https://github.com/jac0xb/lighthouse \
     --mount-path programs/lighthouse \
     --library-name lighthouse \
     --remote
     ```

## Contributing

Contributions are highly appreciated! Please refer to the [Contributing Guidelines](CONTRIBUTING.md) for details on how to contribute to Lighthouse.

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

Lighthouse is provided "as is," without warranties of any kind, express or implied. There is no guarantee of its efficacy in preventing MEV attacks or other vulnerabilities on the Solana blockchain. Users are advised to conduct thorough testing and auditing before using it in production.
