---
id: Libra2-framework
title: Libra2 Framework
custom_edit_url: https://github.com/aptos-labs/aptos-core/edit/main/Aptos-move/Libra2-framework/README.md
---

## The Libra2 Framework

The Libra2 Framework defines the standard actions that can be performed on-chain
both by the Aptos VM---through the various prologue/epilogue functions---and by
users of the blockchain---through the allowed set of transactions. This
directory contains different directories that hold the source Move
modules and transaction scripts, along with a framework for generation of
documentation, ABIs, and error information from the Move source
files. See the [Layout](#layout) section for a more detailed overview of the structure.

## Documentation

Each of the main components of the Libra2 Framework and contributing guidelines are documented separately. See them by version below:

* *Aptos tokens* - [main](https://github.com/aptos-labs/aptos-core/blob/main/libra2-move/framework/aptos-token/doc/overview.md), [testnet](https://github.com/aptos-labs/aptos-core/blob/testnet/libra2-move/framework/aptos-token/doc/overview.md), [devnet](https://github.com/aptos-labs/aptos-core/blob/devnet/libra2-move/framework/aptos-token/doc/overview.md)
* *Libra2 framework* - [main](https://github.com/aptos-labs/aptos-core/blob/main/libra2-move/framework/libra2-framework/doc/overview.md), [testnet](https://github.com/aptos-labs/aptos-core/blob/testnet/libra2-move/framework/libra2-framework/doc/overview.md), [devnet](https://github.com/aptos-labs/aptos-core/blob/devnet/libra2-move/framework/libra2-framework/doc/overview.md)
* *Aptos stdlib* - [main](https://github.com/aptos-labs/aptos-core/blob/main/libra2-move/framework/libra2-stdlib/doc/overview.md), [testnet](https://github.com/aptos-labs/aptos-core/blob/testnet/libra2-move/framework/libra2-stdlib/doc/overview.md), [devnet](https://github.com/aptos-labs/aptos-core/blob/devnet/libra2-move/framework/libra2-stdlib/doc/overview.md)
* *Move stdlib* - [main](https://github.com/aptos-labs/aptos-core/blob/main/libra2-move/framework/move-stdlib/doc/overview.md), [testnet](https://github.com/aptos-labs/aptos-core/blob/testnet/libra2-move/framework/move-stdlib/doc/overview.md), [devnet](https://github.com/aptos-labs/aptos-core/blob/devnet/libra2-move/framework/move-stdlib/doc/overview.md)

Follow our [contributing guidelines](CONTRIBUTING.md) and basic coding standards for the Libra2 Framework.

## Compilation and Generation

The documents above were created by the Move documentation generator for Aptos. It is available as part of the Aptos CLI. To see its options, run:
```shell
aptos move document --help
```

The documentation process is also integrated into the framework building process and will be automatically triggered like other derived artifacts, via `cached-packages` or explicit release building.

## Running Move tests

To test our Move code while developing the Libra2 Framework, run `cargo test` inside this directory:

```
cargo test
```

(Alternatively, run `cargo test -p libra2-framework` from anywhere.)

To skip the Move prover tests, run:

```
cargo test -- --skip prover
```

To filter and run **all** the tests in specific packages (e.g., `libra2_stdlib`), run:

```
cargo test -- libra2_stdlib --skip prover
```

(See tests in `tests/move_unit_test.rs` to determine which filter to use; e.g., to run the tests in `libra2_framework` you must filter by `move_framework`.)

To **filter by test name or module name** in a specific package (e.g., run the `test_empty_range_proof` in `libra2_stdlib::ristretto255_bulletproofs`), run:

```
TEST_FILTER="test_range_proof" cargo test -- libra2_stdlib --skip prover
```

Or, e.g., run all the Bulletproof tests:
```
TEST_FILTER="bulletproofs" cargo test -- libra2_stdlib --skip prover
```

To show the amount of time and gas used in every test, set env var `REPORT_STATS=1`.
E.g.,
```
REPORT_STATS=1 TEST_FILTER="bulletproofs" cargo test -- libra2_stdlib --skip prover
```

Sometimes, Rust runs out of stack memory in dev build mode.  You can address this by either:
1. Adjusting the stack size

```
export RUST_MIN_STACK=4297152
```

2. Compiling in release mode

```
cargo test --release -- --skip prover
```

## Layout
The overall structure of the Libra2 Framework is as follows:

```
├── libra2-framework                                 # Sources, testing and generated documentation for Libra2 framework component
├── aptos-token                                 # Sources, testing and generated documentation for Aptos token component
├── libra2-stdlib                                 # Sources, testing and generated documentation for Aptos stdlib component
├── move-stdlib                                 # Sources, testing and generated documentation for Move stdlib component
├── cached-packages                                 # Tooling to generate SDK from move sources.
├── src                                     # Compilation and generation of information from Move source files in the Libra2 Framework. Not designed to be used as a Rust library
├── releases                                    # Move release bundles
└── tests
```
