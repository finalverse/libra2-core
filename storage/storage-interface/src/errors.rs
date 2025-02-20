// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module defines error types used by `Libra2DB`.
use libra2_types::state_store::errors::StateViewError;
use std::sync::mpsc::RecvError;
use thiserror::Error;

/// This enum defines errors commonly used among `Libra2DB` APIs.
#[derive(Debug, Error)]
pub enum Libra2DbError {
    /// A requested item is not found.
    #[error("{0} not found.")]
    NotFound(String),
    /// Requested too many items.
    #[error("Too many items requested: at least {0} requested, max is {1}")]
    TooManyRequested(u64, u64),
    #[error("Missing state root node at version {0}, probably pruned.")]
    MissingRootError(u64),
    /// Other non-classified error.
    #[error("Libra2DB Other Error: {0}")]
    Other(String),
    #[error("Libra2DB RocksDb Error: {0}")]
    RocksDbIncompleteResult(String),
    #[error("Libra2DB RocksDB Error: {0}")]
    OtherRocksDbError(String),
    #[error("Libra2DB bcs Error: {0}")]
    BcsError(String),
    #[error("Libra2DB IO Error: {0}")]
    IoError(String),
    #[error("Libra2DB Recv Error: {0}")]
    RecvError(String),
    #[error("Libra2DB ParseInt Error: {0}")]
    ParseIntError(String),
}

impl From<anyhow::Error> for Libra2DbError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<bcs::Error> for Libra2DbError {
    fn from(error: bcs::Error) -> Self {
        Self::BcsError(format!("{}", error))
    }
}

impl From<RecvError> for Libra2DbError {
    fn from(error: RecvError) -> Self {
        Self::RecvError(format!("{}", error))
    }
}

impl From<std::io::Error> for Libra2DbError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(format!("{}", error))
    }
}

impl From<std::num::ParseIntError> for Libra2DbError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<Libra2DbError> for StateViewError {
    fn from(error: Libra2DbError) -> Self {
        match error {
            Libra2DbError::NotFound(msg) => StateViewError::NotFound(msg),
            Libra2DbError::Other(msg) => StateViewError::Other(msg),
            _ => StateViewError::Other(format!("{}", error)),
        }
    }
}

impl From<StateViewError> for Libra2DbError {
    fn from(error: StateViewError) -> Self {
        match error {
            StateViewError::NotFound(msg) => Libra2DbError::NotFound(msg),
            StateViewError::Other(msg) => Libra2DbError::Other(msg),
            StateViewError::BcsError(err) => Libra2DbError::BcsError(err.to_string()),
        }
    }
}
