// Empty governance proposal to demonstrate functionality for including proposal in the release builder;
//
script {
    use libra2_framework::libra2_governance;
    use std::features;

    fun main(proposal_id: u64) {
         let _framework_signer = libra2_governance::resolve_multi_step_proposal(
            proposal_id,
            @0000000000000000000000000000000000000000000000000000000000000001,
            {{ script_hash }},
        );
    }
}
