module aa::test_functions {
    use libra2_framework::libra2_account;

    /// test function for multi-agent aa.
    public entry fun transfer_to_the_last(a: &signer, b: &signer, c: &signer, d: address) {
        libra2_account::transfer(a, d, 1);
        libra2_account::transfer(b, d, 1);
        libra2_account::transfer(c, d, 1);
    }
}
