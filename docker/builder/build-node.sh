#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}
FEATURES=${FEATURES:-""}

echo "Building libra2-node"
echo "PROFILE: $PROFILE"
echo "FEATURES: $FEATURES"
echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

PACKAGES=(
    libra2-node
    aptos-forge-cli
)

# We have to do these separately because we need to avoid feature unification
# between libra2-node and other binaries
for PACKAGE in "${PACKAGES[@]}"; do
    # Build and overwrite the libra2-node binary with features if specified
    if [ -n "$FEATURES" ] && [ "$PACKAGE" = "libra2-node" ]; then
        echo "Building libra2-node with features ${FEATURES}"
        cargo build --profile=$PROFILE --features=$FEATURES -p $PACKAGE "$@"
    else 
        # Build libra2-node separately
        cargo build --locked --profile=$PROFILE -p $PACKAGE "$@"
    fi
done

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    libra2-node
    forge
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done
