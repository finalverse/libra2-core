// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_tool::{ArgWithType, FunctionArgType},
    CliResult, Tool,
};
use clap::Parser;
use std::str::FromStr;

/// In order to ensure that there aren't duplicate input arguments for untested CLI commands,
/// we call help on every command to ensure it at least runs
#[tokio::test]
async fn ensure_every_command_args_work() {
    assert_cmd_not_panic(&["libra2"]).await;

    assert_cmd_not_panic(&["libra2", "account"]).await;
    assert_cmd_not_panic(&["libra2", "account", "create", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "account", "create-resource-account", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "account", "fund-with-faucet", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "account", "list", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "account", "lookup-address", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "account", "rotate-key", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "account", "transfer", "--help"]).await;

    assert_cmd_not_panic(&["libra2", "config"]).await;
    assert_cmd_not_panic(&["libra2", "config", "generate-shell-completions", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "config", "init", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "config", "set-global-config", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "config", "show-global-config"]).await;
    assert_cmd_not_panic(&["libra2", "config", "show-profiles"]).await;

    assert_cmd_not_panic(&["libra2", "genesis"]).await;
    assert_cmd_not_panic(&["libra2", "genesis", "generate-genesis", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "genesis", "generate-keys", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "genesis", "generate-layout-template", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "genesis", "set-validator-configuration", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "genesis", "setup-git", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "genesis", "generate-admin-write-set", "--help"]).await;

    assert_cmd_not_panic(&["libra2", "governance"]).await;
    assert_cmd_not_panic(&["libra2", "governance", "execute-proposal", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "governance", "generate-upgrade-proposal", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "governance", "propose", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "governance", "vote", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "governance", "delegation_pool", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "governance", "delegation_pool", "vote", "--help"]).await;
    assert_cmd_not_panic(&[
        "libra2",
        "governance",
        "delegation_pool",
        "propose",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["libra2", "info"]).await;

    assert_cmd_not_panic(&["libra2", "init", "--help"]).await;

    assert_cmd_not_panic(&["libra2", "key"]).await;
    assert_cmd_not_panic(&["libra2", "key", "generate", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "key", "extract-peer", "--help"]).await;

    assert_cmd_not_panic(&["libra2", "move"]).await;
    assert_cmd_not_panic(&["libra2", "move", "clean", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "compile", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "compile-script", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "decompile", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "disassemble", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "download", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "init", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "list", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "prove", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "publish", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "run", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "run-script", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "test", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "transactional-test", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "move", "view", "--help"]).await;

    assert_cmd_not_panic(&["libra2", "node"]).await;
    assert_cmd_not_panic(&["libra2", "node", "check-network-connectivity", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "get-stake-pool", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "analyze-validator-performance", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "bootstrap-db-from-backup", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "initialize-validator", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "join-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "leave-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "run-local-testnet", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "show-validator-config", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "show-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "show-validator-stake", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "node", "update-consensus-key", "--help"]).await;
    assert_cmd_not_panic(&[
        "libra2",
        "node",
        "update-validator-network-addresses",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["libra2", "stake"]).await;
    assert_cmd_not_panic(&["libra2", "stake", "add-stake", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "stake", "increase-lockup", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "stake", "initialize-stake-owner", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "stake", "set-delegated-voter", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "stake", "set-operator", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "stake", "unlock-stake", "--help"]).await;
    assert_cmd_not_panic(&["libra2", "stake", "withdraw-stake", "--help"]).await;
}

/// Ensure we can parse URLs for args
#[tokio::test]
async fn ensure_can_parse_args_with_urls() {
    let result = ArgWithType::from_str("string:https://libra2.org").unwrap();
    matches!(result._ty, FunctionArgType::String);
    assert_eq!(
        result.arg,
        bcs::to_bytes(&"https://libra2.org".to_string()).unwrap()
    );
}

async fn assert_cmd_not_panic(args: &[&str]) {
    // When a command fails, it will have a panic in it due to an improperly setup command
    // thread 'main' panicked at 'Command propose: Argument names must be unique, but 'assume-yes' is
    // in use by more than one argument or group', ...

    match run_cmd(args).await {
        Ok(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
        Err(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
    }
}

async fn run_cmd(args: &[&str]) -> CliResult {
    let tool: Tool = Tool::try_parse_from(args).map_err(|msg| msg.to_string())?;
    tool.execute().await
}
