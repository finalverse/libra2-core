spec libra2_framework::jwk_consensus_config {
    spec on_new_epoch(framework: &signer) {
        requires @libra2_framework == std::signer::address_of(framework);
        include config_buffer::OnNewEpochRequirement<JWKConsensusConfig>;
        aborts_if false;
    }
}
