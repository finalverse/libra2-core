// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    manifest_parser::git_repo_cache_path,
    source_package::parsed_manifest::{Dependency, GitInfo},
};
use clap::ValueEnum;
use move_symbol_pool::symbol::Symbol;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

/// Represents a standard library.
pub enum StdLib {
    Libra2TokenObjects,
    Libra2Token,
    Libra2Framework,
    Libra2Stdlib,
    MoveStdlib,
}

impl StdLib {
    /// The well-known git URL for the standard library.
    const STD_GIT_URL: &'static str = "https://github.com/aptos-labs/libra2-framework.git";

    /// Returns the dependency for the standard library with the given version.
    pub fn dependency(&self, version: &StdVersion) -> Dependency {
        if let StdVersion::Local(path) = version {
            Dependency {
                local: PathBuf::from(path).join(self.sub_dir()),
                subst: None,
                version: None,
                digest: None,
                git_info: None,
                node_info: None,
            }
        } else {
            let rev = version.rev().expect("non-local version");
            let local = git_repo_cache_path(Self::STD_GIT_URL, rev);
            Dependency {
                local: local.join(self.sub_dir()),
                subst: None,
                version: None,
                digest: None,
                git_info: Some(GitInfo {
                    git_url: Symbol::from(StdLib::STD_GIT_URL),
                    git_rev: Symbol::from(rev),
                    subdir: PathBuf::from(self.sub_dir()),
                    download_to: local,
                }),
                node_info: None,
            }
        }
    }

    /// Returns the name of the standard library.
    pub fn as_str(&self) -> &'static str {
        match self {
            StdLib::Libra2Token => "Libra2Token",
            StdLib::Libra2TokenObjects => "Libra2TokenObjects",
            StdLib::Libra2Framework => "Libra2Framework",
            StdLib::Libra2Stdlib => "Libra2Stdlib",
            StdLib::MoveStdlib => "MoveStdlib",
        }
    }

    /// Returns the standard library from the given package name, or `None` if the package name is not a standard library.
    pub fn from_package_name(package_name: Symbol) -> Option<StdLib> {
        match package_name.as_str() {
            "Libra2Token" => Some(StdLib::Libra2Token),
            "Libra2TokenObjects" => Some(StdLib::Libra2TokenObjects),
            "Libra2Framework" => Some(StdLib::Libra2Framework),
            "Libra2Stdlib" => Some(StdLib::Libra2Stdlib),
            "MoveStdlib" => Some(StdLib::MoveStdlib),
            _ => None,
        }
    }

    /// Returns the subdirectory of the standard library in the git repository.
    fn sub_dir(&self) -> &'static str {
        match self {
            StdLib::Libra2Token => "libra2-token",
            StdLib::Libra2TokenObjects => "libra2-token-objects",
            StdLib::Libra2Framework => "libra2-framework",
            StdLib::Libra2Stdlib => "libra2-stdlib",
            StdLib::MoveStdlib => "move-stdlib",
        }
    }
}

/// Represents a standard library version.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum StdVersion {
    Mainnet,
    Testnet,
    Devnet,
    #[clap(skip)]
    Local(String),
}

impl StdVersion {
    const DEVNET: &'static str = "devnet";
    const MAINNET: &'static str = "mainnet";
    const TESTNET: &'static str = "testnet";

    /// Returns the rev name of the standard library version.
    /// Returns `None` for a local version.
    pub fn rev(&self) -> Option<&'static str> {
        match self {
            StdVersion::Mainnet => Some(StdVersion::MAINNET),
            StdVersion::Testnet => Some(StdVersion::TESTNET),
            StdVersion::Devnet => Some(StdVersion::DEVNET),
            StdVersion::Local(_) => None,
        }
    }

    /// Returns the standard library version from the given rev name,
    /// or `None` if the string is not a standard library version.
    pub fn from_rev(version: &str) -> Option<StdVersion> {
        match version {
            StdVersion::MAINNET => Some(Self::Mainnet),
            StdVersion::TESTNET => Some(Self::Testnet),
            StdVersion::DEVNET => Some(Self::Devnet),
            _ => None,
        }
    }
}

impl Display for StdVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let StdVersion::Local(path) = self {
            write!(f, "local={}", path)
        } else {
            write!(f, "{}", self.rev().expect("non-local"))
        }
    }
}
