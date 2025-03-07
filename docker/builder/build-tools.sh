#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=cli

echo "Building tools and services docker images"
echo "PROFILE: $PROFILE"
echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p aptos \
    -p libra2-backup-cli \
    -p libra2-faucet-service \
    -p libra2-fn-check-client \
    -p libra2-node-checker \
    -p libra2-openapi-spec-generator \
    -p libra2-telemetry-service \
    -p libra2-keyless-pepper-service \
    -p libra2-debugger \
    -p libra2-transaction-emitter \
    -p libra2-api-tester \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    aptos
    libra2-faucet-service
    libra2-node-checker
    libra2-openapi-spec-generator
    libra2-telemetry-service
    libra2-keyless-pepper-service
    libra2-fn-check-client
    libra2-debugger
    libra2-transaction-emitter
    libra2-api-tester
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done

# Build the Aptos Move framework and place it in dist. It can be found afterwards in the current directory.
echo "MOVE_COMPILER_V2: ${MOVE_COMPILER_V2:-not set}"
echo "Building the Aptos Move framework..."
(cd dist && cargo run --locked --profile=$PROFILE --package libra2-framework -- release)
