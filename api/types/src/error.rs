// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use libra2_types::vm_status::StatusCode;
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// This is the generic struct we use for all API errors, it contains a string
/// message and an Aptos API specific error code.
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct Libra2Error {
    /// A message describing the error
    pub message: String,
    pub error_code: Libra2ErrorCode,
    /// A code providing VM error details when submitting transactions to the VM
    pub vm_error_code: Option<u64>,
}

impl std::fmt::Display for Libra2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error({:?}): {:#}", self.error_code, self.message)
    }
}

impl std::error::Error for Libra2Error {}

impl Libra2Error {
    pub fn new_with_error_code<ErrorType: std::fmt::Display>(
        error: ErrorType,
        error_code: Libra2ErrorCode,
    ) -> Libra2Error {
        Self {
            message: format!("{:#}", error),
            error_code,
            vm_error_code: None,
        }
    }

    pub fn new_with_vm_status<ErrorType: std::fmt::Display>(
        error: ErrorType,
        error_code: Libra2ErrorCode,
        vm_error_code: StatusCode,
    ) -> Libra2Error {
        Self {
            message: format!("{:#}", error),
            error_code,
            vm_error_code: Some(vm_error_code as u64),
        }
    }
}

/// These codes provide more granular error information beyond just the HTTP
/// status code of the response.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[repr(u32)]
pub enum Libra2ErrorCode {
    /// Account not found at the requested version
    AccountNotFound = 101,
    /// Resource not found at the requested version
    ResourceNotFound = 102,
    /// Module not found at the requested version
    ModuleNotFound = 103,
    /// Struct field not found at the requested version
    StructFieldNotFound = 104,
    /// Ledger version not found at the requested version
    ///
    /// Usually means that the version is ahead of the latest version
    VersionNotFound = 105,
    /// Transaction not found at the requested version or with the requested hash
    TransactionNotFound = 106,
    /// Table item not found at the requested version
    TableItemNotFound = 107,
    /// Block not found at the requested version or height
    ///
    /// Usually means the block is fully or partially pruned or the height / version is ahead
    /// of the latest version
    BlockNotFound = 108,
    ///  StateValue not found at the requested version
    StateValueNotFound = 109,

    /// Ledger version is pruned
    VersionPruned = 200,
    /// Block is fully or partially pruned
    BlockPruned = 201,

    /// The API's inputs were invalid
    InvalidInput = 300,

    /// The transaction was an invalid update to an already submitted transaction.
    InvalidTransactionUpdate = 401,
    /// The sequence number for the transaction is behind the latest sequence number.
    SequenceNumberTooOld = 402,
    /// The submitted transaction failed VM checks.
    VmError = 403,

    /// Health check failed.
    HealthCheckFailed = 500,
    /// The mempool is full, no new transactions can be submitted.
    MempoolIsFull = 501,

    /// Internal server error
    InternalError = 600,
    /// Error from the web framework
    WebFrameworkError = 601,
    /// BCS format is not supported on this API.
    BcsNotSupported = 602,
    /// API Disabled
    ApiDisabled = 603,
}

impl Libra2ErrorCode {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

#[test]
fn test_serialize_deserialize() {
    let with_code = Libra2Error::new_with_vm_status(
        "Invalid transaction",
        Libra2ErrorCode::VmError,
        libra2_types::vm_status::StatusCode::UNKNOWN_MODULE,
    );
    let _: Libra2Error = bcs::from_bytes(&bcs::to_bytes(&with_code).unwrap()).unwrap();
    let _: Libra2Error = serde_json::from_str(&serde_json::to_string(&with_code).unwrap()).unwrap();

    let without_code =
        Libra2Error::new_with_error_code("some message", Libra2ErrorCode::MempoolIsFull);
    let _: Libra2Error = bcs::from_bytes(&bcs::to_bytes(&without_code).unwrap()).unwrap();
    let _: Libra2Error =
        serde_json::from_str(&serde_json::to_string(&without_code).unwrap()).unwrap();
}
