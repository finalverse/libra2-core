// Empty governance proposal to demonstrate functionality for including proposal in the release builder;
//
script {
    use libra2_framework::libra2_governance;

    fun main(proposal_id: u64) {
        let _framework_signer = libra2_governance::resolve(proposal_id, @0x1);
    }
}
