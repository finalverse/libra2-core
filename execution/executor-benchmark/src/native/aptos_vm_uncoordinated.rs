// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::native_config::NATIVE_EXECUTOR_POOL;
use libra2_block_executor::{
    counters::BLOCK_EXECUTOR_INNER_EXECUTE_BLOCK, txn_provider::default::DefaultTxnProvider,
};
use libra2_types::{
    block_executor::{
        config::BlockExecutorConfigFromOnchain,
        transaction_slice_metadata::TransactionSliceMetadata,
    },
    state_store::StateView,
    transaction::{
        signature_verified_transaction::SignatureVerifiedTransaction, BlockOutput,
        TransactionOutput,
    },
    vm_status::VMStatus,
};
use libra2_vm::{Libra2VM, VMBlockExecutor};
use libra2_vm_environment::environment::Libra2Environment;
use libra2_vm_logging::log_schema::AdapterLogSchema;
use libra2_vm_types::module_and_script_storage::AsLibra2CodeStorage;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub struct Libra2VMParallelUncoordinatedBlockExecutor;

impl VMBlockExecutor for Libra2VMParallelUncoordinatedBlockExecutor {
    fn new() -> Self {
        Self
    }

    fn execute_block(
        &self,
        txn_provider: &DefaultTxnProvider<SignatureVerifiedTransaction>,
        state_view: &(impl StateView + Sync),
        _onchain_config: BlockExecutorConfigFromOnchain,
        _transaction_slice_metadata: TransactionSliceMetadata,
    ) -> Result<BlockOutput<TransactionOutput>, VMStatus> {
        let _timer = BLOCK_EXECUTOR_INNER_EXECUTE_BLOCK.start_timer();

        // let features = Features::fetch_config(&state_view).unwrap_or_default();

        let env = Libra2Environment::new(state_view);
        let vm = Libra2VM::new(env.clone(), state_view);

        let transaction_outputs = NATIVE_EXECUTOR_POOL.install(|| {
            txn_provider
                .get_txns()
                .par_iter()
                .enumerate()
                .map(|(txn_idx, txn)| {
                    let log_context = AdapterLogSchema::new(state_view.id(), txn_idx);
                    let code_storage = state_view.as_aptos_code_storage(env.clone());

                    vm.execute_single_transaction(
                        txn,
                        &vm.as_move_resolver(state_view),
                        &code_storage,
                        &log_context,
                    )
                    .map(|(_vm_status, vm_output)| {
                        vm_output
                            .try_materialize_into_transaction_output(state_view)
                            .unwrap()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(BlockOutput::new(transaction_outputs, None))
    }
}
