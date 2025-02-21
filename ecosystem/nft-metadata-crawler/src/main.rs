// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_indexer_grpc_server_framework::ServerArgs;
use libra2_nft_metadata_crawler::config::NFTMetadataCrawlerConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = <ServerArgs as clap::Parser>::parse();
    args.run::<NFTMetadataCrawlerConfig>().await
}
