# libra2-sdk

[![libra2-sdk on crates.io](https://img.shields.io/crates/v/libra2-sdk)](https://crates.io/crates/libra2-sdk)
[![Documentation (latest release)](https://docs.rs/libra2-sdk/badge.svg)](https://docs.rs/libra2-sdk/)
[![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://aptos.github.io/aptos/libra2_sdk/)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/finalverse/libra2-core/blob/main/LICENSE)

The official Rust SDK for Libra2.

## Usage

This SDK provides all the necessary components for building on top of the Libra2 Blockchain. Some of the important modules are:

* `client` - Includes a [REST client](https://aptos.dev/nodes/libra2-api-spec#/) implementation
* `crypto` - Types used for signing and verifying
* `transaction_builder` - Includes helpers for constructing transactions
* `types` - Includes types for Libra2 on-chain data structures

## Installing Rust SDK
Please refer to [Rust SDK Doc](https://aptos.dev/sdks/rust-sdk/) for details on how to install the Rust SDK.

## License

Libra2 Core is licensed as [Apache 2.0](https://github.com/finalverse/libra2-core/blob/main/LICENSE).
