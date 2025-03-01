// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_api::context::Context;
use libra2_config::config::NodeConfig;
use libra2_mempool::mocks::MockSharedMempool;
use libra2_storage_interface::mock::MockDbReaderWriter;
use libra2_types::chain_id::ChainId;
use std::sync::Arc;

// This is necessary for building the API with how the code is structured currently.
pub fn get_fake_context() -> Context {
    let mempool = MockSharedMempool::new_with_runtime();
    Context::new(
        ChainId::test(),
        Arc::new(MockDbReaderWriter),
        mempool.ac_client,
        NodeConfig::default(),
        None, /* table info reader */
    )
}
