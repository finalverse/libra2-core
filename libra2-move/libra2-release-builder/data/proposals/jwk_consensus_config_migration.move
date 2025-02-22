// Initialize `libra2_framework::jwk_consensus_config::JWKConsensusConfig` with Google.
// Start to ignore `libra2_framework::jwks::SupportedOIDCProviders`.
// Start to ignore move feature flag `std::features::JWK_CONSENSUS`.
script {
    use libra2_framework::libra2_governance;
    use libra2_framework::jwk_consensus_config;
    use std::string::utf8;

    fun main(proposal_id: u64) {
        let framework = libra2_governance::resolve_multi_step_proposal(
            proposal_id,
            @0x1,
            {{ script_hash }},
        );
        let provider_google = jwk_consensus_config::new_oidc_provider(
            utf8(b"https://accounts.google.com"),
            utf8(b"https://accounts.google.com/.well-known/openid-configuration"),
        );
        let config = jwk_consensus_config::new_v1(vector[provider_google]);
        jwk_consensus_config::initialize(&framework, config);
        libra2_governance::reconfigure(&framework);
    }
}
