script {
    use libra2_framework::libra2_governance;
    use libra2_framework::coin;
    use libra2_framework::libra2_coin::Libra2Coin;
    use libra2_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = libra2_governance::resolve(proposal_id, @libra2_framework);
        let one_libra2_coin_with_decimals = 10 ** (coin::decimals<Libra2Coin>() as u64);
        // Change min to 1000 and max to 1M Aptos coins.
        let new_min_stake = 1000 * one_libra2_coin_with_decimals;
        let new_max_stake = 1000000 * one_libra2_coin_with_decimals;
        staking_config::update_required_stake(&framework_signer, new_min_stake, new_max_stake);
    }
}
