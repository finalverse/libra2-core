script {
    use libra2_framework::aptos_governance;
    use libra2_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = aptos_governance::resolve(proposal_id, @libra2_framework);
        // Update voting power increase limit to 10%.
        staking_config::update_voting_power_increase_limit(&framework_signer, 10);
    }
}
