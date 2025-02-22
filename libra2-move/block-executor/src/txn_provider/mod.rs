// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

mod blocking_txns_provider;
pub mod default;

use libra2_mvhashmap::types::TxnIndex;
use libra2_types::transaction::BlockExecutableTransaction as Transaction;

pub trait TxnProvider<T: Transaction> {
    /// Get total number of transactions
    fn num_txns(&self) -> usize;

    /// Get a reference of the txn object by its index.
    fn get_txn(&self, idx: TxnIndex) -> &T;
}
