spec libra2_framework::reconfiguration {
    /// <high-level-req>
    /// No.: 1
    /// Requirement: The Configuration resource is stored under the Libra2 framework account with initial values upon
    /// module's initialization.
    /// Criticality: Medium
    /// Implementation: The Configuration resource may only be initialized with specific values and published under the
    /// libra2_framework account.
    /// Enforcement: Formally verified via [high-level-req-1](initialize).
    ///
    /// No.: 2
    /// Requirement: The reconfiguration status may be determined at any time without causing an abort, indicating whether
    /// or not the system allows reconfiguration.
    /// Criticality: Low
    /// Implementation: The reconfiguration_enabled function will never abort and always returns a boolean value that
    /// accurately represents whether the system allows reconfiguration.
    /// Enforcement: Formally verified via [high-level-req-2](reconfiguration_enabled).
    ///
    /// No.: 3
    /// Requirement: For each reconfiguration, the epoch value (config_ref.epoch) increases by 1, and one 'NewEpochEvent'
    /// is emitted.
    /// Criticality: Critical
    /// Implementation: After reconfiguration, the reconfigure() function increases the epoch value of the configuration
    /// by one and increments the counter of the NewEpochEvent's EventHandle by one.
    /// Enforcement: Audited that these two values remain in sync.
    ///
    /// No.: 4
    /// Requirement: Reconfiguration is possible only if genesis has started and reconfiguration is enabled. Also, the last
    /// reconfiguration must not be the current time, returning early without further actions otherwise.
    /// Criticality: High
    /// Implementation: The reconfigure() function may only execute to perform successful reconfiguration when genesis
    /// has started and when reconfiguration is enabled. Without satisfying both conditions, the function returns early
    /// without executing any further actions.
    /// Enforcement: Formally verified via [high-level-req-4](reconfigure).
    ///
    /// No.: 5
    /// Requirement: Consecutive reconfigurations without the passage of time are not permitted.
    /// Criticality: High
    /// Implementation: The reconfigure() function enforces the restriction that reconfiguration may only be performed
    /// when the current time is not equal to the last_reconfiguration_time.
    /// Enforcement: Formally verified via [high-level-req-5](reconfigure).
    /// </high-level-req>
    ///

    spec module {
        pragma verify = true;
        pragma aborts_if_is_strict;

        // After genesis, `Configuration` exists.
        invariant [suspendable] chain_status::is_operating() ==> exists<Configuration>(@libra2_framework);
        invariant [suspendable] chain_status::is_operating() ==>
            (timestamp::spec_now_microseconds() >= last_reconfiguration_time());
    }

    /// Make sure the signer address is @libra2_framework.
    spec schema AbortsIfNotLibra2Framework {
        libra2_framework: &signer;

        let addr = signer::address_of(libra2_framework);
        aborts_if !system_addresses::is_libra2_framework_address(addr);
    }

    /// Address @libra2_framework must exist resource Account and Configuration.
    /// Already exists in framework account.
    /// Guid_creation_num should be 2 according to logic.
    spec initialize(libra2_framework: &signer) {
        use std::signer;
        use libra2_framework::account::{Account};
        use libra2_framework::guid;

        include AbortsIfNotLibra2Framework;
        let addr = signer::address_of(libra2_framework);
        let post config = global<Configuration>(@libra2_framework);
        requires exists<Account>(addr);
        aborts_if !(global<Account>(addr).guid_creation_num == 2);
        aborts_if exists<Configuration>(@libra2_framework);
        // property 1: During the module's initialization, it guarantees that the Configuration resource will move under
        // the Libra2 framework account with initial values.
        /// [high-level-req-1]
        ensures exists<Configuration>(@libra2_framework);
        ensures config.epoch == 0 && config.last_reconfiguration_time == 0;
        ensures config.events == event::EventHandle<NewEpochEvent> {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: 2,
                    addr: @libra2_framework
                }
            }
        };
    }

    spec current_epoch(): u64 {
        aborts_if !exists<Configuration>(@libra2_framework);
        ensures result == global<Configuration>(@libra2_framework).epoch;
    }

    spec disable_reconfiguration(libra2_framework: &signer) {
        include AbortsIfNotLibra2Framework;
        aborts_if exists<DisableReconfiguration>(@libra2_framework);
        ensures exists<DisableReconfiguration>(@libra2_framework);
    }

    /// Make sure the caller is admin and check the resource DisableReconfiguration.
    spec enable_reconfiguration(libra2_framework: &signer) {
        use libra2_framework::reconfiguration::{DisableReconfiguration};
        include AbortsIfNotLibra2Framework;
        aborts_if !exists<DisableReconfiguration>(@libra2_framework);
        ensures !exists<DisableReconfiguration>(@libra2_framework);
    }

    /// When genesis_event emit the epoch and the `last_reconfiguration_time` .
    /// Should equal to 0
    spec emit_genesis_reconfiguration_event {
        use libra2_framework::reconfiguration::{Configuration};

        aborts_if !exists<Configuration>(@libra2_framework);
        let config_ref = global<Configuration>(@libra2_framework);
        aborts_if !(config_ref.epoch == 0 && config_ref.last_reconfiguration_time == 0);
        ensures global<Configuration>(@libra2_framework).epoch == 1;
    }

    spec last_reconfiguration_time {
        aborts_if !exists<Configuration>(@libra2_framework);
        ensures result == global<Configuration>(@libra2_framework).last_reconfiguration_time;
    }

    spec reconfigure {
        use libra2_framework::aptos_coin;
        use libra2_framework::staking_config;

        // TODO: set because of timeout (property proved)
        pragma verify = true;
        pragma verify_duration_estimate = 600;

        let success = !(chain_status::is_genesis() || timestamp::spec_now_microseconds() == 0 || !reconfiguration_enabled())
            && timestamp::spec_now_microseconds() != global<Configuration>(@libra2_framework).last_reconfiguration_time;
        include features::spec_periodical_reward_rate_decrease_enabled() ==> staking_config::StakingRewardsConfigEnabledRequirement;
        include success ==> aptos_coin::ExistsAptosCoin;
        aborts_if false;
        // The ensure conditions of the reconfigure function are not fully written, because there is a new cycle in it,
        // but its existing ensure conditions satisfy hp.
        // The property below is not proved within 500s and still cause an timeout
        // property 3: Synchronization of NewEpochEvent counter with configuration epoch.
        ensures success ==> global<Configuration>(@libra2_framework).epoch == old(global<Configuration>(@libra2_framework).epoch) + 1;
        ensures success ==> global<Configuration>(@libra2_framework).last_reconfiguration_time == timestamp::spec_now_microseconds();
        // We remove the ensures of event increment due to inconsisency
        // TODO: property 4: Only performs reconfiguration if genesis has started and reconfiguration is enabled.
        // Also, the last reconfiguration must not be the current time, returning early without further actions otherwise.
        // property 5: Consecutive reconfigurations without the passage of time are not permitted.
        /// [high-level-req-4]
        /// [high-level-req-5]
        ensures !success ==> global<Configuration>(@libra2_framework).epoch == old(global<Configuration>(@libra2_framework).epoch);
    }

    spec reconfiguration_enabled {
        // property 2: The reconfiguration status may be determined at any time without causing an abort, indicating
        // whether or not the system allows reconfiguration.
        /// [high-level-req-2]
        aborts_if false;
        ensures result == !exists<DisableReconfiguration>(@libra2_framework);
    }
}
