// Initialize on-chain randomness resources.
script {
    use libra2_framework::aptos_governance;
    use libra2_framework::config_buffer;
    use libra2_framework::dkg;
    use libra2_framework::randomness;
    use libra2_framework::randomness_config;
    use libra2_framework::reconfiguration_state;

    fun main(proposal_id: u64) {
        let framework = aptos_governance::resolve_multi_step_proposal(
            proposal_id,
            @0x1,
            {{ script_hash }},
        );
        config_buffer::initialize(&framework); // on-chain config buffer
        dkg::initialize(&framework); // DKG state holder
        reconfiguration_state::initialize(&framework); // reconfiguration in progress global indicator
        randomness::initialize(&framework); // randomness seed holder

        let config = randomness_config::new_off();
        randomness_config::initialize(&framework, config);
    }
}
