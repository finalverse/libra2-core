# The Libra2 Framework Repo

This repository serves as a mirror for the Libra2 Framework packages, including the Move standard library. The contents are synced from [aptos-core](https://github.com/aptos-labs/aptos-core) on an hourly basis.

By pulling dependencies from this mirror repository, developers can avoid downloading unnecessary data, reducing build time significantly.

## Usage
To use the packages in this repository as dependencies in your Move project, you can include them in your move.toml file by adding them as Git dependencies.

To add a dependency from this repository, include the following in your `move.toml` file:
```
[dependencies]
<package_name> = { git = "https://github.com/aptos-labs/libra2-framework.git", subdir = "<path_to_directory_containing_Move.toml>", rev = "<commit_hash_or_branch_name>" }
```
For example, to add `Libra2Framework` from the `mainnet` branch, you would use:
```
Libra2Framework = { git = "https://github.com/aptos-labs/libra2-framework.git", subdir = "libra2-framework", rev = "mainnet" }
```
Make sure to replace `subdir` with the appropriate path if you are referencing a different package within the framework.

## Contributing
If you want to contribute to the development of the framework, please submit issues and pull requests to the [aptos-core](https://github.com/aptos-labs/aptos-core) repository, where active development happens.

Bugs, feature requests, or discussions of enhancements will be tracked in the issue section there as well. This repository is a mirror, and issues will not be tracked here.
