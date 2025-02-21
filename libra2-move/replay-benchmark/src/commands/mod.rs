// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_logger::{Level, Logger};
use libra2_move_debugger::libra2_debugger::Libra2Debugger;
use libra2_push_metrics::MetricsPusher;
use libra2_rest_client::{AptosBaseUrl, Client};
pub use benchmark::BenchmarkCommand;
use clap::Parser;
pub use diff::DiffCommand;
pub use download::DownloadCommand;
pub use initialize::InitializeCommand;
use url::Url;

mod benchmark;
mod diff;
mod download;
mod initialize;

pub(crate) fn init_logger_and_metrics(log_level: Level) {
    let mut logger = Logger::new();
    logger.level(log_level);
    logger.init();

    let _mp = MetricsPusher::start(vec![]);
}

pub(crate) fn build_debugger(
    rest_endpoint: String,
    api_key: Option<String>,
) -> anyhow::Result<Libra2Debugger> {
    let builder = Client::builder(AptosBaseUrl::Custom(Url::parse(&rest_endpoint)?));
    let client = if let Some(api_key) = api_key {
        builder.api_key(&api_key)?.build()
    } else {
        builder.build()
    };
    Libra2Debugger::rest_client(client)
}

#[derive(Parser)]
pub struct RestAPI {
    #[clap(
        long,
        help = "Fullnode's REST API query endpoint, e.g., https://api.mainnet.aptoslabs.com/v1 \
                for mainnet"
    )]
    rest_endpoint: String,

    #[clap(
        long,
        help = "Optional API key to increase HTTP request rate limit quota"
    )]
    api_key: Option<String>,
}
