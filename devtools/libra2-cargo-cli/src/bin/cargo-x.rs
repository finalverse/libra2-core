// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_cargo_cli::{Libra2CargoCommand, SelectedPackageArgs};
use clap::Parser;
use std::process::exit;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    #[command(name = "x")]
    Libra2CargoTool(Libra2CargoToolArgs),
}

#[derive(Parser)]
struct Libra2CargoToolArgs {
    #[command(subcommand)]
    cmd: Libra2CargoCommand,
    #[command(flatten)]
    package_args: SelectedPackageArgs,
}

fn main() {
    let CargoCli::Libra2CargoTool(args) = CargoCli::parse();
    let Libra2CargoToolArgs { cmd, package_args } = args;
    let result = cmd.execute(&package_args);

    // At this point, we'll want to print and determine whether to exit for an error code
    match result {
        Ok(_) => {},
        Err(inner) => {
            println!("{}", inner);
            exit(1);
        },
    }
}
