// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use libra2_config::config::{NodeConfig, DEFAULT_EXECUTION_CONCURRENCY_LEVEL};
#[cfg(unix)]
use libra2_logger::prelude::*;
use libra2_storage_interface::{
    state_store::state_view::db_state_view::LatestDbStateCheckpointView, DbReaderWriter,
};
use libra2_types::{
    account_config::ChainIdResource, chain_id::ChainId, on_chain_config::OnChainConfig,
};
use libra2_vm::Libra2VM;
use libra2_vm_environment::prod_configs::set_paranoid_type_checks;
use std::cmp::min;

/// Error message to display when non-production features are enabled
pub const ERROR_MSG_BAD_FEATURE_FLAGS: &str = r#"
libra2-node was compiled with feature flags that shouldn't be enabled.

This is caused by cargo's feature unification.
When you compile two crates with a shared dependency, if one enables a feature flag for the dependency, then it is also enabled for the other crate.

PLEASE RECOMPILE LIBRA2-NODE SEPARATELY using the following command:
    cargo build --package libra2-node

"#;

/// Initializes a global rayon thread pool iff `create_global_rayon_pool` is true
pub fn create_global_rayon_pool(create_global_rayon_pool: bool) {
    if create_global_rayon_pool {
        rayon::ThreadPoolBuilder::new()
            .thread_name(|index| format!("rayon-global-{}", index))
            .build_global()
            .expect("Failed to build rayon global thread pool.");
    }
}

/// Fetches the chain ID from on-chain resources
pub fn fetch_chain_id(db: &DbReaderWriter) -> anyhow::Result<ChainId> {
    let db_state_view = db
        .reader
        .latest_state_checkpoint_view()
        .map_err(|err| anyhow!("[libra2-node] failed to create db state view {}", err))?;
    Ok(ChainIdResource::fetch_config(&db_state_view)
        .expect("[libra2-node] missing chain ID resource")
        .chain_id())
}

/// Sets the Libra2 VM configuration based on the node configurations
pub fn set_libra2_vm_configurations(node_config: &NodeConfig) {
    set_paranoid_type_checks(node_config.execution.paranoid_type_verification);
    let effective_concurrency_level = if node_config.execution.concurrency_level == 0 {
        min(
            DEFAULT_EXECUTION_CONCURRENCY_LEVEL,
            (num_cpus::get() / 2) as u16,
        )
    } else {
        node_config.execution.concurrency_level
    };
    Libra2VM::set_concurrency_level_once(effective_concurrency_level as usize);
    Libra2VM::set_discard_failed_blocks(node_config.execution.discard_failed_blocks);
    Libra2VM::set_num_proof_reading_threads_once(
        node_config.execution.num_proof_reading_threads as usize,
    );

    if node_config
        .execution
        .processed_transactions_detailed_counters
    {
        Libra2VM::set_processed_transactions_detailed_counters();
    }
}

pub fn ensure_max_open_files_limit(required: u64) {
    if required == 0 {
        return;
    }

    // Only works on Unix environments
    #[cfg(unix)]
    {
        if !rlimit::Resource::NOFILE.is_supported() {
            warn!(
                required = required,
                "rlimit setting not supported on this platform. Won't ensure."
            );
            return;
        }

        let (soft, mut hard) = match rlimit::Resource::NOFILE.get() {
            Ok((soft, hard)) => (soft, hard),
            Err(err) => {
                warn!(
                    error = ?err,
                    required = required,
                    "Failed getting RLIMIT_NOFILE. Won't ensure."
                );
                return;
            },
        };

        if soft >= required {
            return;
        }

        if required > hard {
            warn!(
                hard_limit = hard,
                required = required,
                "System RLIMIT_NOFILE hard limit too small."
            );
            // Not panicking right away -- user can be root
            hard = required;
        }

        rlimit::Resource::NOFILE
            .set(required, hard)
            .unwrap_or_else(|err| {
                panic!("Failed raising RLIMIT_NOFILE soft limit to {required}. Error: {err}")
            });
    }
}
