# Lighthouse: Solana Assertion Program

---

## Overview

Lighthouse is a Solana program developed to make assertions on Solana state that will fail the transaction if not true. Designed to combat prevalent issues such as MEV (Miner Extractable Value) attacks, and more specifically sandwich attacks. Allows for whitehat probing of Solana programs. Lighthouse empowers developers by enabling assertions about account balances and account state. This ensures that transactions adhere to predefined conditions, with usecases like safeguarding signer assets and maintaining transaction integrity.

## Features

- **Balance Assertions**: Users can assert expected account balances at the end of transactions to prevent unauthorized asset extraction.
- **Instruction Integrity Checks**: Validates instruction data to protect against manipulation, including sandwich attacks.
- **Transaction Finality Assurance**: Ensures the immutability of transactions post-validation.
- **Probing Defense Mechanism**: Offers robust protection against probing by ethical hackers or malicious entities.

## Getting Started

### Prerequisites

TODO

### Installation

1. Clone the Lighthouse repository:
   ```bash
   git clone https://github.com/jac0xb/lighthouse.git
   ```
2. Change directory to Lighthouse:
   ```bash
   cd lighthouse
   ```
3. Compile the program:
   ```bash
   npm run programs:build
   ```

## Usage

Integrate Lighthouse in your Solana transactions by specifying your assertion criteria. Here's an example in Rust:

~COMING SOON~

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
