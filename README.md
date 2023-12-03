# Lighthouse: Solana Assertion Program

---

## Overview

Lighthouse is a Solana program developed to make asertions on Solana state that will fail the transaction if not true. Designed to combat prevalent issues such as MEV (Miner Extractable Value) attacks, and more specifically sandwich attacks. Allows for whitehat probing of Solana programs. Lighthouse empowers developers by enabling assertions about account balances and account state. This ensures that transactions adhere to predefined conditions, with usecases like safeguarding signer assets and maintaining transaction integrity.

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
- Program data address check (Program data swap attack)
- Turn config into bit flag to save instruction space
- Add security info
- Add verification of source
- Resize cache account
