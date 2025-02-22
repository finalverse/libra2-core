module 0xcafe::test {
    use libra2_framework::coin::{Self, Coin};
    use libra2_framework::libra2_coin::Libra2Coin;

    struct State has key {
        important_value: u64,
        coins: Coin<Libra2Coin>,
    }

    fun init_module(s: &signer) {
        move_to(s, State {
            important_value: get_value(),
            coins: coin::zero<Libra2Coin>(),
        })
    }

    fun get_value(): u64 {
        2
    }
}
