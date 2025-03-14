# gravex-swap

A revamped constant product AMM program optimized for straightforward pool deployment along with additional features and integrations:

- No Openbook market ID is required for pool creation
- Token22 is supported
- Built-in price oracle
- Optimized in Anchor

The program is a fork of Raydium which was audited by [MadShield](https://www.madshield.xyz/). The report can be found [here](https://github.com/raydium-io/raydium-docs/tree/master/audit/MadShield%20Q1%202024).

Token22 and Library extensions have been added to support Gravity Coin Contracts

## Environment Setup

1. Install `Rust`

   ```shell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default 1.79.0
   ```

2. Install `Solana `

   ```shell
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   ```

   then run `solana-keygen new` to create a keypair at the default location.

3. install `Anchor`

   ```shell
   # Installing using Anchor version manager (avm)
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   # Install anchor
   avm install 0.29.0
   ```

## Quickstart

Clone the repository and test the program.

```shell

git clone https://github.com/gravex-io/gravex-swap
cd gravex-swap && yarn && anchor test
```

## License

Gravex swap is licensed under the Apache License, Version 2.0.
