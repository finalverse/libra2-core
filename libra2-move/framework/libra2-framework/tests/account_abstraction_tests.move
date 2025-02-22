#[test_only]
module libra2_framework::account_abstraction_tests {
    use libra2_framework::auth_data::AbstractionAuthData;

    public fun test_auth(account: signer, _data: AbstractionAuthData): signer { account }
}
