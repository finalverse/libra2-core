// Initialize AIP-28 parital governance voting.
// This script MUST be run before enabling the feature flag, otherwise emitting the fee statement will fail.
script {
    use libra2_framework::libra2_governance;
    use libra2_framework::transaction_fee;

    fun main(proposal_id: u64) {
        let framework_signer = libra2_governance::resolve_multi_step_proposal(
            proposal_id,
            @0x1,
            {{ script_hash }},
        );
        transaction_fee::initialize_storage_refund(&framework_signer);
    }
}
