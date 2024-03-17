# Lighthouse - The Assertion Solana Program

![0_0](https://github.com/Jac0xb/lighthouse/assets/5273873/d5bb94be-a994-424f-88e0-32a917f07129)

## Overview

Lighthouse is a Solana program that provides assertion instructions that can be added to transactions. If a bad actor spoofs simulation results, there is overspending during the transaction, or an oracle account is in an undesired state, the assertion will fail, and the transaction will also fail. Lighthouse makes it simple to append assertion instructions to existing transactions without needing to write new Solana programs.

Lighthouse is an open source, public utility Solana program with an emphasis on security (multisig, verifiable build, non-upgradable releases, etc _coming soon_), composability (program-agnostic use cases), and community (open source, assist in integration with open source projects, incentivize contributions).

## Example Usecases

Solana at its core is a decentralized database (accounts) and assertion/mutation of that permissioned data (programs). Programs generally make assertions about account state before allowing data to be mutated, one of Anchor's innovations for preventing "footguns and attacks" was that it has common account state assertions baked into the framework. Currently performing even trivial assertions such as account balance checks requires one to deploy a Solana program. Lighthouse at a high-level seeks to be an composable way to make assertions on onchain state and the instruction-level delta of these state changes without the need to deploy additional Solana programs.

**Guardrail Example**: A wallet simulates that a token account changes balance from `100` to `90` for a transaction. It appends a Lighthouse assertion instruction to the transaction which says the token account balance must be 90 at the end of the transaction (the assertion instruction is placed at the end of the transaction).

```rust
let tx = blackhat_program
   .drain_token_account(&user, &drainer.encodable_pubkey(), &mint.pubkey())
   .append(lighthouse_program.create_assert(
      &user,
      user_ata,
      Assertion::LegacyTokenAccountField(
         LegacyTokenAccountField::Amount(90),
         Operator::Equal,
      ),
   )
)
```

(From the testing library, blackhat program is a test program designed to emulate existing drainer programs)

The transaction is then sent to the Solana blockchain. The assertion fails because the token balance is found to be `0` instead of `90` during assertion instruction. Having failed the assertion, the Lighthouse program then fails the transaction.

**Guardrail Example**: A transaction builder puts a Lighthouse assertion instruction that consists of checking a BONK USD price oracle account and asserts that it needs to be above a certain value or the transaction will fail.

**Whitehat Example**: The game SVBonk involves sending 1000 BONK to their program, if one minute elapses the first user to interact with SVBonk wins the jackpot. Lighthouse could be used to demonstrate to the creators of SVBonk about a potential exploit (that has now been patched). The exploit involves failing the transaction if the transaction builder made a state assertion which checked that they were the winner of the game at the end of the transaction, if they weren't the transaction would fail, thus exploiting the game by spamming transactions which will either succeed (win the game) or fail (no BONK sent). Lighthouse can be seen as a new useful tool for Solana smart contract developers to reason about and test state change exploitable edge cases in their smart contract.

**Jito Bundle Guardrail Example**: A bad actor cosigns a drainer transaction so it cannot be altered. A wallet provider builds a bundle of transactions consisting of 1) the bad actor's transaction 2) a lighthouse assertion transaction built from the expected simulation changes. The lighthouse assertion transaction fails after catching the bad actor's unexpected state changes.

## Features

**Assertion Instructions (Primary)** - instructions which allow transaction builders to assert on data accessed at runtime during the assertion instruction.

_Current Assertion Data Model_

```rust
// Used in assertions during evaluation.
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

// Used in assertion as "expected value"
pub enum DataValue {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    Bytes(Vec<u8>),
    Pubkey(Pubkey),
}

// Used in AccountInfoField assertion to assert about account info.
pub enum AccountInfoField {
    Key(Pubkey),
    Lamports(u64),
    DataLength(u64),
    Owner(Pubkey),
    RentEpoch(u64),
    IsSigner(bool),
    IsWritable(bool),
    Executable(bool),
}

// Used in LegacyTokenAccountField assertion to assert on token account data.
pub enum LegacyTokenAccountField {
    Mint(Pubkey),
    Owner(Pubkey),
    Amount(u64),
    Delegate(Option<Pubkey>),
    State(u8),
    IsNative(Option<u64>),
    DelegatedAmount(u64),
    CloseAuthority(Option<Pubkey>),
}

pub enum Assertion {
    AccountInfoField(AccountInfoField, Operator),
    AccountData(u16, Operator, DataValue),
    LegacyTokenAccountField(LegacyTokenAccountField, Operator),
}
```

_Current Assertion Instructions_

```rust
pub fn assert_v1<'info>(...) -> Result<()> {
   // Allows for logging (CU intensive but useful for debugging) and has more instruction data overhead (config struct)
}

pub fn assert_compact_v1<'info>(...) -> Result<()> {
   // Has less data instruction overhead, and suppresses CU intensive logging.
}

pub fn assert_multi_v1<'info>(...) -> Result<()> {
   // Allows for logging on assertion (CU intensive) and has more instruction data overhead (uses vector for assertions and has a config struct)
}

pub fn assert_multi_compact_v1<'info>(...) -> Result<()> {
   // Uses fixed-array defined in enum for assertions and suppresses CU intensive logging.
}
```

**Write Instructions (Secondary)** - write account data, account info, and other data available at runtime into memory accounts to allow for assertion of inter-transaction state changes rather than just instruction data and data accessed at runtime during assertion instruction.

**Memory Account** - PDA with seeds `[b"memory".as_ref(), signer.key.as_ref(), &[memory_idx]]`. The memory account is used to store runtime data into memory accounts derived by the signer. Useful for asserting on the difference of changes between instructions.

_Current Write Data Model_

```rust
pub enum WriteTypeParameter {
    // Memory offset, write type
    WriteU8(u8, WriteType),
    WriteU16(u16, WriteType),
    WriteU32(u32, WriteType),
}


pub enum WriteType {
    // Account data start index, end index, validations (owner, exists, etc)
    AccountData(u16, Option<u16>, Option<AccountValidation>),
    AccountInfo,
    DataValue(DataValue),
}
```

_Current Write Instructions_

```rust
pub fn create_memory_account_v1<'info>(..., memory_idx) -> Result<()> {
   // Create a memory account which can then be used to write data to.
}


pub fn write_v1<'info>(..., memory_idx, write_type) -> Result<()> {
   // Write data to a memory account using predefined write types.
}
```

## Getting Started

### Prerequisites

- Install pnpm (https://pnpm.io/7.x/installation or npm install -g pnpm)

### Installation

- pnpm install

1. Compile the program:
   ```bash
   pnpm run programs:build
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

## Todo

- Instruction protections (Allow for assertions on CPI, etc)
  - Assert address is signer
- Program data address check (Program data swap attack)
  - Program unchanged
- Turn config into bit flag to save instruction space
- Add security info
- Add verification of source
- Resize cache account
- Write account info support
- Token Account Assertions
  - State assertions
  - Has Delegate (https://twitter.com/kb24x7/status/1731261594141671824)
  - Has No Delegate
- Instruction Data Hotloader
  - Allow instruction data to be hot loaded from cache to allow for instruction data to react to state change
- Piece-wise instruction CPI constructor
  - Piece-wise write segments of instruction data in a cache and then cpi execute the instruction data
- Perform arithmetic on cached values
  - Look into creating expressions which can load values, evaluate expression, save back to cache
- Protected cache write spaces
  - Think about if it is necessary to assign/protect cache space to transactions/assertion ids
- Assert on blocktime, slot, etc.

- Write Multi V1
  - Save on tranasction space by only needing to call write once.
- Auto-increment validation
  - Check to make sure transactions are ran in sequence or they fail
- Decide on using CPI events vs program logs events
  - Extra account overhead for cpi events pda
  - program logs concat
  - Do we even need logs :P
- Assertion type: hash of account, for post-simulation account integrity
- POD / Bytemuck / zerocopy rewrite of instruction data
  - Maybe need to move to native implementation
- [] Look into memcpy syscall for memory write.
- [] Add multi-write to roadmap
- [x] .info() isnt a good name
- [] Look for more programs to add to KnownPrograms
- [] Use log level helpers
- [] Add additional merkle tree assertions to roadmap
- [] Probably move get_state to its own function not as a variable
- [] Write test plan for validations, multi instructions, utils
- [] Source account for memory write needs to be optional
- [] Add multi write to roadmap
- [] Should I add 128 bit support for account data delta
- [] Probably readd data is empty check
- [] Remove anchor for spl compression
