// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::*;
use libra2_crypto::HashValue;
use libra2_schemadb::{schema::fuzzing::assert_encode_decode, test_no_panic_decoding};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_encode_decode(
        state_key in any::<HashValue>(),
        version in any::<Version>(),
        v in any::<Option<StateValue>>(),
    ) {
        assert_encode_decode::<StateValueByKeyHashSchema>(&(state_key, version), &v);
    }
}

test_no_panic_decoding!(StateValueByKeyHashSchema);
