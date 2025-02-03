# On-Chain Development

## Overview

This is an on-chain development project built in Rust that uses `solana-program` crate. It goes through all the basic regarding programs (smart contracts).
This project is linked to the [off_chain_solana_typescript](https://github.com/benjaminPla/off_chain_solana_typescript) repository, which provides the off-chain interactions required to interact with these smart contracts.

## Structure

```
.
├── hello_world
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── README.md
└── receive_data
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── lib.rs
```

## Clusters

The project is built to run on any cluster, but I highly recommend to run it locally.

1. `solana-test-validator`
2. `cargo build-sbf`
3. `solana program deloy <path-to-file.so>`

## Images

