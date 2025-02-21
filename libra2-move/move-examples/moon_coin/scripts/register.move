//:!:>moon
script {
    fun register(account: &signer) {
        libra2_framework::managed_coin::register<MoonCoin::moon_coin::MoonCoin>(account)
    }
}
//<:!:moon
