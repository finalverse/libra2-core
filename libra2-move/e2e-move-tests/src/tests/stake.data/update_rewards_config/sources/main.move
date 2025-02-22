script {
    use libra2_framework::libra2_governance;
    use libra2_framework::staking_config;
    use libra2_std::fixed_point64;

    fun main(core_resources: &signer) {
        let framework_signer = libra2_governance::get_signer_testnet_only(core_resources, @libra2_framework);
        staking_config::update_rewards_config(
            &framework_signer,
            fixed_point64::create_from_rational(1, 100),
            fixed_point64::create_from_rational(3, 1000),
            365 * 24 * 60 * 60,
            fixed_point64::create_from_rational(50, 100),
        );
        libra2_governance::force_end_epoch(&framework_signer);
    }
}
