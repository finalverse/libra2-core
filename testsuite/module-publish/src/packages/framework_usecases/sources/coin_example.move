module 0xABCD::coin_example {
    use std::signer;

    struct ExampleCoin {}

    fun init_module(sender: &signer) {
        libra2_framework::managed_coin::initialize<ExampleCoin>(
            sender,
            b"Example Coin",
            b"Example",
            8,
            false,
        );
    }

    public entry fun mint_p(user: &signer, admin: &signer, amount: u64) {
        libra2_framework::managed_coin::register<ExampleCoin>(user);
        libra2_framework::managed_coin::mint<ExampleCoin>(admin, signer::address_of(user), amount);
    }
}
