#Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

clean:
	- rm -rf go.mod go.sum aptosstdlib libra2types

test:
	cargo run -p libra2-framework release
	cargo run -p libra2-sdk-builder -- \
		--language go --module-name aptosstdlib \
		--package-name testing \
		--with-libra2-types "../../../../testsuite/generate-format/tests/staged/aptos.yaml" \
		--target-source-dir . \
		"../../../framework/libra2-framework/build/Libra2Framework"
	- go mod init testing
	go mod edit -replace testing=`pwd`
	go get github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/bcs
	go get github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/serde
	go run stdlib_demo.go

.PHONY: clean test
