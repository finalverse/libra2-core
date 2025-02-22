spec libra2_framework::dkg {

    spec module {
        use libra2_framework::chain_status;
        invariant [suspendable] chain_status::is_operating() ==> exists<DKGState>(@libra2_framework);
    }

    spec initialize(libra2_framework: &signer) {
        use std::signer;
        let libra2_framework_addr = signer::address_of(libra2_framework);
        aborts_if libra2_framework_addr != @libra2_framework;
    }

    spec start(
        dealer_epoch: u64,
        randomness_config: RandomnessConfig,
        dealer_validator_set: vector<ValidatorConsensusInfo>,
        target_validator_set: vector<ValidatorConsensusInfo>,
    ) {
        aborts_if !exists<DKGState>(@libra2_framework);
        aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@libra2_framework);
    }

    spec finish(transcript: vector<u8>) {
        use std::option;
        requires exists<DKGState>(@libra2_framework);
        requires option::is_some(global<DKGState>(@libra2_framework).in_progress);
        aborts_if false;
    }

    spec fun has_incomplete_session(): bool {
        if (exists<DKGState>(@libra2_framework)) {
            option::spec_is_some(global<DKGState>(@libra2_framework).in_progress)
        } else {
            false
        }
    }

    spec try_clear_incomplete_session(fx: &signer) {
        use std::signer;
        let addr = signer::address_of(fx);
        aborts_if addr != @libra2_framework;
    }

    spec incomplete_session(): Option<DKGSessionState> {
        aborts_if false;
    }
}
