module 0xcafe::test {
    use libra2_framework::coin::{Self, Coin};
    use libra2_framework::aptos_coin::AptosCoin;

    struct State has key {
        important_value: u64,
        coins: Coin<AptosCoin>,
    }

    fun init_module(s: &signer) {
        move_to(s, State {
            important_value: get_value(),
            coins: coin::zero<AptosCoin>(),
        })
    }

    fun get_value(): u64 {
        2
    }
}
