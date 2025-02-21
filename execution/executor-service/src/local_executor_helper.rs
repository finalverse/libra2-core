// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_infallible::Mutex;
use libra2_logger::info;
use libra2_storage_interface::state_store::state_view::cached_state_view::CachedStateView;
use libra2_vm::{
    sharded_block_executor::{local_executor_shard::LocalExecutorClient, ShardedBlockExecutor},
    Libra2VM,
};
use once_cell::sync::Lazy;
use std::sync::Arc;

pub static SHARDED_BLOCK_EXECUTOR: Lazy<
    Arc<Mutex<ShardedBlockExecutor<CachedStateView, LocalExecutorClient<CachedStateView>>>>,
> = Lazy::new(|| {
    info!("LOCAL_SHARDED_BLOCK_EXECUTOR created");
    Arc::new(Mutex::new(
        LocalExecutorClient::create_local_sharded_block_executor(Libra2VM::get_num_shards(), None),
    ))
});
