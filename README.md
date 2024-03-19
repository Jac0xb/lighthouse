# Lighthouse - The Assertion Solana Program

![0_0](https://github.com/Jac0xb/lighthouse/assets/5273873/d5bb94be-a994-424f-88e0-32a917f07129)

## Overview

Lighthouse is a Solana program that provides assertion instructions that can be added to transactions. If a bad actor spoofs simulation results, there is overspending during the transaction, or an oracle account is in an undesired state, the assertion will fail, and the transaction will also fail. Lighthouse makes it simple to append assertion instructions to existing transactions without needing to write new Solana programs.

Lighthouse is an open source, public utility Solana program with an emphasis on security (multisig, verifiable build, non-upgradable releases, etc _coming soon_), composability (program-agnostic use cases), and community (open source, assist in integration with open source projects, incentivize contributions).

## Example Usecases

Solana at its core is a decentralized database (accounts) and assertion/mutation of that permissioned data (programs). Programs generally make assertions about account state before allowing data to be mutated, one of Anchor's innovations for preventing "footguns and attacks" was that it has common account state assertions baked into the framework. Currently performing even trivial assertions such as account balance checks requires one to deploy a Solana program. Lighthouse at a high-level seeks to be an composable way to make assertions on onchain state and the instruction-level delta of these state changes without the need to deploy additional Solana programs.

**Guardrail Example**: A wallet simulates that a token account changes balance from `100` to `90` for a transaction. It appends a Lighthouse assertion instruction to the transaction which says the token account balance must be 90 at the end of the transaction (the assertion instruction is placed at the end of the transaction).

````
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

(From the testing library, blackhat program is a test program designed to emulate existing drainer programs)

The transaction is then sent to the Solana blockchain. The assertion fails because the token balance is found to be `0` instead of `90` during assertion instruction. Having failed the assertion, the Lighthouse program then fails the transaction.

**Guardrail Example**: A transaction builder puts a Lighthouse assertion instruction that consists of checking a BONK USD price oracle account and asserts that it needs to be above a certain value or the transaction will fail.

**Whitehat Example**: The game SVBonk involves sending 1000 BONK to their program, if one minute elapses the first user to interact with SVBonk wins the jackpot. Lighthouse could be used to demonstrate to the creators of SVBonk about a potential exploit (that has now been patched). The exploit involves failing the transaction if the transaction builder made a state assertion which checked that they were the winner of the game at the end of the transaction, if they weren't the transaction would fail, thus exploiting the game by spamming transactions which will either succeed (win the game) or fail (no BONK sent). Lighthouse can be seen as a new useful tool for Solana smart contract developers to reason about and test state change exploitable edge cases in their smart contract.

**Jito Bundle Guardrail Example**: A bad actor cosigns a drainer transaction so it cannot be altered. A wallet provider builds a bundle of transactions consisting of 1) the bad actor's transaction 2) a lighthouse assertion transaction built from the expected simulation changes. The lighthouse assertion transaction fails after catching the bad actor's unexpected state changes.

## Features

**Assertion Instructions (Primary)** - instructions which allow transaction builders to assert on data accessed at runtime during the assertion instruction.

**Write Instructions (Secondary)** - write account data, account info, and other data available at runtime into memory accounts to allow for assertion of inter-transaction state changes rather than just instruction data and data accessed at runtime during assertion instruction.

**Memory Account** - PDA with seeds `[b"memory".as_ref(), signer.key.as_ref(), &[memory_idx]]`. The memory account is used to store runtime data into memory accounts derived by the signer. Useful for asserting on the difference of changes between instructions.

## Getting Started

TODO

### Prerequisites

- Install pnpm (https://pnpm.io/7.x/installation or npm install -g pnpm)

### Installation

- pnpm install

1. Compile the program:
   ```bash
   pnpm run programs:build
````

2. Run the tests:

   ```bash
   pnpm run programs:test
   ```

3. Generate shank and kinobi clients:
   ```bash
   pnpm run generate
   ```

## Usage

For experimental use only, program has yet to receive an audit and more tests need to be written!

devnet: L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK
mainnet: L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK

## Contributing

Your contributions are highly appreciated. For contributing guidelines, please refer to [Contributing to Lighthouse](CONTRIBUTING.md).

## License

This project is under the [MIT License](LICENSE).

## Disclaimer

Lighthouse is provided "as is", with no warranties regarding its efficacy in completely preventing MEV attacks or other vulnerabilities. Users are advised to conduct thorough testing and auditing.
