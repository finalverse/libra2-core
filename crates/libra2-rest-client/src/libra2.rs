// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_api_types::U64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AptosCoin {
    pub value: U64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub coin: AptosCoin,
}

impl Balance {
    pub fn get(&self) -> u64 {
        *self.coin.value.inner()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AptosVersion {
    pub major: U64,
}
