// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use libra2_metrics_core::{exponential_buckets, register_histogram_vec, HistogramVec};
use once_cell::sync::Lazy;

pub(crate) static TIMER: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "libra2_storage_interface_timer_seconds",
        "Various timers for performance analysis.",
        &["name"],
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});
