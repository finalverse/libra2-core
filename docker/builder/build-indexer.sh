#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}

echo "Building indexer and related binaries"
echo "PROFILE: $PROFILE"

echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p libra2-indexer-grpc-cache-worker \
    -p libra2-indexer-grpc-file-store \
    -p libra2-indexer-grpc-data-service \
    -p libra2-nft-metadata-crawler \
    -p libra2-indexer-grpc-file-checker \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    libra2-indexer-grpc-cache-worker
    libra2-indexer-grpc-file-store
    libra2-indexer-grpc-data-service
    libra2-nft-metadata-crawler
    libra2-indexer-grpc-file-checker
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done
