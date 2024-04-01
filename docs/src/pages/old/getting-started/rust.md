---
title: Getting Started using Rust
metaTitle: Candy Machine - Getting Started - Rust
description: Get started with Candy Machines using Rust
---

If you are a Rust developer, you can also use a Rust crate to interact with the Candy Machine program. Since the program is written in Rust, this crate contains all the program's logic, including helper methods that prepare instructions for us.

This can be helpful if you are developing a Rust client or if you want to make [CPI calls](https://solanacookbook.com/references/programs.html#how-to-do-cross-program-invocation) to the Candy Machine program within your program.

Since candy machines are composed of two programs, you will need to install two libraries.

- **Candy Machine Core**
  - [GitHub Repository](https://github.com/metaplex-foundation/mpl-candy-machine/tree/main/programs/candy-machine-core)
  - [Crate Page](https://crates.io/crates/mpl-candy-machine-core)
  - [API References](https://docs.rs/mpl-candy-machine-core/0.1.0/mpl_candy_machine_core/)
- **Candy Guard**
  - [GitHub Repository](https://github.com/metaplex-foundation/mpl-candy-machine/tree/main/programs/candy-guard)
  - [Crate Page](https://crates.io/crates/mpl-candy-guard)
  - [API References](https://docs.rs/mpl-candy-guard/0.1.0/mpl_candy_guard/)
