// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_api_types::U64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Libra2Coin {
    pub value: U64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub coin: Libra2Coin,
}

impl Balance {
    pub fn get(&self) -> u64 {
        *self.coin.value.inner()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Libra2Version {
    pub major: U64,
}
