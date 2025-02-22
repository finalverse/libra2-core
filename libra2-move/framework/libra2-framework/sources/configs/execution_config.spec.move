spec libra2_framework::execution_config {
    spec module {
        pragma verify = true;
        pragma aborts_if_is_strict;
    }

    /// Ensure the caller is admin
    /// When setting now time must be later than last_reconfiguration_time.
    spec set(account: &signer, config: vector<u8>) {
        use libra2_framework::timestamp;
        use std::signer;
        use std::features;
        use libra2_framework::chain_status;
        use libra2_framework::staking_config;
        use libra2_framework::libra2_coin;

        // TODO: set because of timeout (property proved)
        pragma verify_duration_estimate = 600;
        let addr = signer::address_of(account);
        requires chain_status::is_genesis();
        requires exists<staking_config::StakingRewardsConfig>(@libra2_framework);
        requires len(config) > 0;
        include features::spec_periodical_reward_rate_decrease_enabled() ==> staking_config::StakingRewardsConfigEnabledRequirement;
        include libra2_coin::ExistsLibra2Coin;
        requires system_addresses::is_libra2_framework_address(addr);
        requires timestamp::spec_now_microseconds() >= reconfiguration::last_reconfiguration_time();

        ensures exists<ExecutionConfig>(@libra2_framework);
    }

    spec set_for_next_epoch(account: &signer, config: vector<u8>) {
        include config_buffer::SetForNextEpochAbortsIf;
    }

    spec on_new_epoch(framework: &signer) {
        requires @libra2_framework == std::signer::address_of(framework);
        include config_buffer::OnNewEpochRequirement<ExecutionConfig>;
        aborts_if false;
    }
}
