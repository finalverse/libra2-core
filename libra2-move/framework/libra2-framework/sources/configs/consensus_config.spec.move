spec libra2_framework::consensus_config {
    /// <high-level-req>
    /// No.: 1
    /// Requirement: During genesis, the Libra2 framework account should be assigned the consensus config resource.
    /// Criticality: Medium
    /// Implementation: The consensus_config::initialize function calls the assert_libra2_framework function to ensure
    /// that the signer is the libra2_framework and then assigns the ConsensusConfig resource to it.
    /// Enforcement: Formally verified via [high-level-req-1](initialize).
    ///
    /// No.: 2
    /// Requirement: Only aptos framework account is allowed to update the consensus configuration.
    /// Criticality: Medium
    /// Implementation: The consensus_config::set function ensures that the signer is libra2_framework.
    /// Enforcement: Formally verified via [high-level-req-2](set).
    ///
    /// No.: 3
    /// Requirement: Only a valid configuration can be used during initialization and update.
    /// Criticality: Medium
    /// Implementation: Both the initialize and set functions validate the config by ensuring its length to be greater
    /// than 0.
    /// Enforcement: Formally verified via [high-level-req-3.1](initialize) and [high-level-req-3.2](set).
    /// </high-level-req>
    ///
    spec module {
        use libra2_framework::chain_status;
        pragma verify = true;
        pragma aborts_if_is_strict;
        invariant [suspendable] chain_status::is_operating() ==> exists<ConsensusConfig>(@libra2_framework);
    }

    /// Ensure caller is admin.
    /// Aborts if StateStorageUsage already exists.
    spec initialize(libra2_framework: &signer, config: vector<u8>) {
        use std::signer;
        let addr = signer::address_of(libra2_framework);
        /// [high-level-req-1]
        aborts_if !system_addresses::is_libra2_framework_address(addr);
        aborts_if exists<ConsensusConfig>(@libra2_framework);
        /// [high-level-req-3.1]
        aborts_if !(len(config) > 0);
        ensures global<ConsensusConfig>(addr) == ConsensusConfig { config };
    }

    /// Ensure the caller is admin and `ConsensusConfig` should be existed.
    /// When setting now time must be later than last_reconfiguration_time.
    spec set(account: &signer, config: vector<u8>) {
        use libra2_framework::chain_status;
        use libra2_framework::timestamp;
        use std::signer;
        use libra2_framework::coin::CoinInfo;
        use libra2_framework::libra2_coin::Libra2Coin;
        use libra2_framework::staking_config;

        // TODO: set because of timeout (property proved)
        pragma verify_duration_estimate = 600;
        include staking_config::StakingRewardsConfigRequirement;
        let addr = signer::address_of(account);
        /// [high-level-req-2]
        aborts_if !system_addresses::is_libra2_framework_address(addr);
        aborts_if !exists<ConsensusConfig>(@libra2_framework);
        /// [high-level-req-3.2]
        aborts_if !(len(config) > 0);

        requires chain_status::is_genesis();
        requires timestamp::spec_now_microseconds() >= reconfiguration::last_reconfiguration_time();
        requires exists<CoinInfo<Libra2Coin>>(@libra2_framework);
        ensures global<ConsensusConfig>(@libra2_framework).config == config;
    }

    spec set_for_next_epoch(account: &signer, config: vector<u8>) {
        include config_buffer::SetForNextEpochAbortsIf;
    }

    spec on_new_epoch(framework: &signer) {
        requires @libra2_framework == std::signer::address_of(framework);
        include config_buffer::OnNewEpochRequirement<ConsensusConfig>;
        aborts_if false;
    }

    spec validator_txn_enabled(): bool {
        pragma opaque;
        aborts_if !exists<ConsensusConfig>(@libra2_framework);
        ensures [abstract] result == spec_validator_txn_enabled_internal(global<ConsensusConfig>(@libra2_framework).config);
    }

    spec validator_txn_enabled_internal(config_bytes: vector<u8>): bool {
        pragma opaque;
        ensures [abstract] result == spec_validator_txn_enabled_internal(config_bytes);
    }

    spec fun spec_validator_txn_enabled_internal(config_bytes: vector<u8>): bool;

}
