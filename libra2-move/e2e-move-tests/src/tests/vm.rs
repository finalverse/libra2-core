// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::MoveHarness;
use libra2_cached_packages::libra2_stdlib::libra2_account_transfer;
use libra2_types::{
    state_store::state_key::StateKey, transaction::ExecutionStatus, write_set::WriteOp,
};
use libra2_vm::Libra2VM;
use libra2_vm_environment::environment::Libra2Environment;
use claims::{assert_ok_eq, assert_some};
use move_core_types::vm_status::{StatusCode, VMStatus};
use test_case::test_case;

// Make sure verification and invariant violation errors are kept.
#[test_case(StatusCode::TYPE_MISMATCH)]
#[test_case(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)]
fn failed_transaction_cleanup_charges_gas(status_code: StatusCode) {
    let mut h = MoveHarness::new();
    let sender = h.new_account_with_balance_and_sequence_number(1_000_000, 10);
    let receiver = h.new_account_with_balance_and_sequence_number(1_000_000, 10);

    let max_gas_amount = 100_000;
    let txn = sender
        .transaction()
        .sequence_number(10)
        .max_gas_amount(max_gas_amount)
        .payload(libra2_account_transfer(*receiver.address(), 1))
        .sign();

    let state_view = h.executor.get_state_view();
    let env = Libra2Environment::new(&state_view);
    let vm = Libra2VM::new(env, state_view);

    let balance = 10_000;
    let output = vm
        .test_failed_transaction_cleanup(
            VMStatus::error(status_code, None),
            &txn,
            state_view,
            balance,
        )
        .1;
    let write_set: Vec<(&StateKey, &WriteOp)> = output
        .concrete_write_set_iter()
        .map(|(k, v)| (k, assert_some!(v)))
        .collect();
    assert!(!write_set.is_empty());
    assert_eq!(output.gas_used(), max_gas_amount - balance);
    assert!(!output.status().is_discarded());
    assert_ok_eq!(
        output.status().as_kept_status(),
        ExecutionStatus::MiscellaneousError(Some(status_code))
    );
}
