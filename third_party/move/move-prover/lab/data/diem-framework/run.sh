#!/bin/bash

FRAMEWORK="../../../../../../libra2-move/framework/libra2-framework/sources"

# Benchmark per function
cargo run --release -p prover-lab -- bench -f -c prover.toml $FRAMEWORK/*.move

# Benchmark per module
cargo run --release -p prover-lab -- bench -c prover.toml $FRAMEWORK/*.move
