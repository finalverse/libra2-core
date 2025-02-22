// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::{libra2::Libra2UpdateTool, revela::RevelaUpdateTool};
use crate::{
    common::types::{CliCommand, CliResult},
    update::{movefmt::FormatterUpdateTool, prover_dependencies::ProverDependencyInstaller},
};
use clap::Subcommand;

/// Update the CLI or other tools it depends on.
#[derive(Subcommand)]
pub enum UpdateTool {
    Libra2(Libra2UpdateTool),
    Revela(RevelaUpdateTool),
    Movefmt(FormatterUpdateTool),
    ProverDependencies(ProverDependencyInstaller),
}

impl UpdateTool {
    pub async fn execute(self) -> CliResult {
        match self {
            UpdateTool::Libra2(tool) => tool.execute_serialized().await,
            UpdateTool::Revela(tool) => tool.execute_serialized().await,
            UpdateTool::Movefmt(tool) => tool.execute_serialized().await,
            UpdateTool::ProverDependencies(tool) => tool.execute_serialized().await,
        }
    }
}
