// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::error::QuorumStoreError;
use libra2_consensus_types::{common::Payload, payload_pull_params::PayloadPullParameters};
use libra2_types::validator_txn::ValidatorTransaction;
use libra2_validator_transaction_pool::TransactionFilter;
use futures::future::BoxFuture;

pub mod mixed;
pub mod user;
pub mod validator;

#[async_trait::async_trait]
pub trait PayloadClient: Send + Sync {
    async fn pull_payload(
        &self,
        config: PayloadPullParameters,
        validator_txn_filter: TransactionFilter,
        wait_callback: BoxFuture<'static, ()>,
    ) -> anyhow::Result<(Vec<ValidatorTransaction>, Payload), QuorumStoreError>;
}
