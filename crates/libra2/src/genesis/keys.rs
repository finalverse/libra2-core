// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    common::{
        types::{CliError, CliTypedResult, OptionalPoolAddressArgs, PromptOptions, RngArgs},
        utils::{check_if_file_exists, create_dir_if_not_exist, dir_default_to_current, read_from_file, write_to_user_only_file},
    },
    genesis::git::{from_yaml, to_yaml, GitOptions, LAYOUT_FILE, OPERATOR_FILE, OWNER_FILE},
    governance::CompileScriptFunction,
    CliCommand,
};
use libra2_genesis::{
    config::{HostAndPort, Layout, OperatorConfiguration, OwnerConfiguration},
    keys::{generate_key_objects, PublicIdentity},
};
use libra2_keygen::KeyGen;
use libra2_types::{account_address::AccountAddress, transaction::authenticator::AuthenticationKey};
use async_trait::async_trait;
use clap::Parser;
use serde_yaml;
use serde_yaml::Value;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use libra2_crypto::ed25519::{Ed25519PrivateKey, Ed25519PublicKey};

const PRIVATE_KEYS_FILE: &str = "private-keys.yaml";
const PUBLIC_KEYS_FILE: &str = "public-keys.yaml";
const VALIDATOR_FILE: &str = "validator-identity.yaml";
const VFN_FILE: &str = "validator-full-node-identity.yaml";
const VALIDATOR_DIR: &str = "my-validator";

/// Generates and saves keys for a validator.
pub fn generate_and_save_keys(output_dir: &str) -> Result<()> {
    let mut keygen = KeyGen::from_seed([0; 32]);

    // Generate keys
    let private_key = keygen.generate_ed25519_private_key();
    let public_key = Ed25519PublicKey::from(&private_key);
    let account_address = AuthenticationKey::ed25519(&public_key).account_address();

    let private_identity = format!(
        "account_address: {}\naccount_private_key: {}",
        account_address, private_key
    );
    let public_identity = format!(
        "account_address: {}\npublic_key: {}",
        account_address, public_key
    );

    // Ensure validator directory exists
    let validator_path = PathBuf::from(output_dir).join(VALIDATOR_DIR);
    fs::create_dir_all(&validator_path).expect("Failed to create validator directory");

    // Define file paths
    let private_keys_file = validator_path.join(PRIVATE_KEYS_FILE);
    let public_keys_file = validator_path.join(PUBLIC_KEYS_FILE);
    let validator_identity_file = validator_path.join(VALIDATOR_FILE);
    let vfn_identity_file = validator_path.join(VFN_FILE);

    // Save private keys
    fs::write(&private_keys_file, private_identity).expect("Failed to write private-keys.yaml");

    // Save public keys
    fs::write(&public_keys_file, public_identity).expect("Failed to write public-keys.yaml");

    // Save dummy validator identity (to be updated later)
    let validator_identity = format!(
        "validator_account: {}\nvalidator_key: {}",
        account_address, public_key
    );
    fs::write(&validator_identity_file, validator_identity)
        .expect("Failed to write validator-identity.yaml");

    // Save dummy full-node identity (to be updated later)
    let vfn_identity = format!(
        "full_node_account: {}\nfull_node_key: {}",
        account_address, public_key
    );
    fs::write(&vfn_identity_file, vfn_identity)
        .expect("Failed to write validator-full-node-identity.yaml");

    println!("✅ Keys successfully generated and saved in {}", validator_path.display());

    Ok(())
}

/// Reads the root public key from `public-keys.yaml`
fn get_root_key_from_public_keys(output_dir: &str) -> String {
    let public_keys_file = format!("{}/my-validator/public-keys.yaml", output_dir);

    let public_keys_content = fs::read_to_string(&public_keys_file)
        .expect(&format!("Failed to read {}", public_keys_file));

    let public_keys_yaml: serde_yaml::Value = serde_yaml::from_str(&public_keys_content)
        .expect("Failed to parse public-keys.yaml");

    public_keys_yaml["public_key"]
        .as_str()
        .expect("Missing public_key field")
        .to_string()
}


/// Generates `layout.yaml` with the correct `root_key`
pub fn generate_layout_yaml(output_dir: &str) {
    let root_key = get_root_key_from_public_keys(output_dir);

    let layout_yaml = format!(
        r#"chain_id: 4
epoch_duration_secs: 600
min_stake: 1000000
max_stake: 100000000
recurring_lockup_duration_secs: 86400
allow_new_validators: true
min_voting_threshold: 500000
rewards_apy_percentage: 10
voting_duration_secs: 432000
voting_power_increase_limit: 1000000
required_proposer_stake: 1000000
root_key: "{}"
default_validator_operator: "my-validator"
validators:
  - my-validator
users:
  - my-validator
"#,
        root_key
    );

    let layout_path = format!("{}/layout.yaml", output_dir);
    fs::write(&layout_path, layout_yaml).expect("Failed to write layout.yaml");

    println!("✅ layout.yaml successfully created with root_key");
}

/// Generate keys for a new validator
#[derive(Parser)]
pub struct GenerateKeys {
    #[clap(long, value_parser)]
    pub(crate) output_dir: Option<PathBuf>,
    #[clap(flatten)]
    pub(crate) pool_address_args: OptionalPoolAddressArgs,
    #[clap(flatten)]
    pub(crate) prompt_options: PromptOptions,
    #[clap(flatten)]
    pub rng_args: RngArgs,
}

#[async_trait]
impl CliCommand<()> for GenerateKeys {
    fn command_name(&self) -> &'static str {
        "GenerateKeys"
    }

    async fn execute(self) -> CliTypedResult<()> {
        let output_dir = dir_default_to_current(self.output_dir.clone())?;

        // Step 1: Generate keys first
        generate_and_save_keys(output_dir.to_str().unwrap())?;

        // Step 2: Now ensure owner.yaml is created using correct keys
        let username = "my-validator";  // This should be dynamically fetched if multiple validators exist
        ensure_owner_file_exists(output_dir.to_str().unwrap(), username);

        Ok(())
    }
}

/// Generate a Layout template file
#[derive(Parser)]
pub struct GenerateLayoutTemplate {
    #[clap(long, value_parser, default_value = LAYOUT_FILE)]
    pub(crate) output_file: PathBuf,
    #[clap(flatten)]
    pub(crate) prompt_options: PromptOptions,
}

#[async_trait]
impl CliCommand<()> for GenerateLayoutTemplate {
    fn command_name(&self) -> &'static str {
        "GenerateLayoutTemplate"
    }

    async fn execute(self) -> CliTypedResult<()> {
        //let output_dir = dir_default_to_current(Some(self.output_file.clone()))?;
        let output_dir = Path::new(self.output_file.as_path())  // ✅ Extract the directory from output_file
            .parent()
            .expect("Failed to extract directory")
            .to_str()
            .unwrap();

        //generate_layout_yaml(output_dir.to_str().unwrap());
        generate_layout_yaml(output_dir);

        Ok(())
    }
}

fn generate_validator_keys(output_dir: &str) {
    generate_and_save_keys(output_dir).expect("Key generation failed");
}

fn ensure_owner_file_exists(local_repository_dir: &str, username: &str) {
    let user_dir = PathBuf::from(local_repository_dir).join(username);
    let owner_file = user_dir.join("owner.yaml");

    // Read actual values from public-keys.yaml
    let public_keys_file = user_dir.join("public-keys.yaml");
    let public_keys_content = fs::read_to_string(&public_keys_file)
        .expect("Failed to read public-keys.yaml");

    let public_keys_yaml: Value = serde_yaml::from_str(&public_keys_content)
        .expect("Failed to parse public-keys.yaml");

    let owner_account_address = public_keys_yaml["account_address"]
        .as_str()
        .expect("Missing account_address in public-keys.yaml");

    let owner_account_public_key = public_keys_yaml["public_key"]
        .as_str()
        .expect("Missing public_key in public-keys.yaml");

    let operator_account_address = owner_account_address; // ❗ Assuming operator is the same as owner
    let operator_account_public_key = owner_account_public_key; // ❗ Assuming the same public key

    if !owner_file.exists() {
        println!("Creating default owner.yaml...");

        let default_owner_config = format!(
            r#"
        owner_account_address: "{}"
        owner_account_public_key: "{}"
        operator_account_address: "{}"
        operator_account_public_key: "{}"
        stake_amount: 1000000
        join_during_genesis: true
        "#,
            owner_account_address,
            owner_account_public_key,
            operator_account_address,
            operator_account_public_key
        );

        fs::create_dir_all(&user_dir).expect("Failed to create validator directory");
        fs::write(&owner_file, default_owner_config).expect("Failed to write owner.yaml");
    }
}
