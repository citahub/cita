# 1) prerequirement
# ./scripts/install_develop.sh
# 2) development
# ./scripts/config_rabbitms.sh
# ./scripts/speedup.sh
# make fmt
# make debug
# make test
# make bench
# make release
################################################################################
CARGO=RUSTFLAGS='-F warnings' cargo

debug:
	$(CARGO) build --all
	scripts/release.sh debug

release:
	$(CARGO) build --all  --release
	scripts/release.sh release

test:
	$(CARGO) test --all  --no-fail-fast 2>&1 |tee target/test.log
	@grep 'test result' target/test.log |awk '\
		 BEGIN { passed=0; failed=0; ignored=0; measured=0; filter=0; } \
			   { passed+=$$4; failed+=$$6; ignored+=$$8;  measured+=$$10; filter+=$$12; } \
		 END   { printf "passed=%d; failed=%d; ignored=%d; measured=%d; filter=%d\n", passed, failed, ignored, measured, filter; }'
	@echo "################################################################################"
	@echo "test error:"
	@grep -A 2  'error\[' target/test.log || exit 0
	@echo "################################################################################"
	@echo "test result:"
	@grep '\.\.\. FAILED' target/test.log || true
	@grep 'error: aborting due to previous error' target/test.log; if [ $$? -eq 0 ] ; then exit 1; fi;
	@grep -q 'error\[' target/test.log; if [ $$? -eq 0 ] ; then exit 1; fi;
	@grep -q '\.\.\. FAILED' target/test.log; if [ $$? -eq 0 ] ; then exit 1; fi;

test_ed25519_blake2b:
	sed -i 's/\["secp256k1"\]/\["ed25519"\]/g' share_libs/crypto/Cargo.toml
	sed -i 's/\["sha3hash"\]/\["blake2bhash"\]/g' share_libs/util/Cargo.toml
	$(CARGO) test  --all  --no-fail-fast 2>&1 |tee target/test.log
	sed -i 's/\["ed25519"\]/\["secp256k1"\]/g' share_libs/crypto/Cargo.toml
	sed -i 's/\["blake2bhash"\]/\["sha3hash"\]/g' share_libs/util/Cargo.toml
	@grep 'test result' target/test.log |awk '\
		 BEGIN { passed=0; failed=0; ignored=0; measured=0; filter=0; } \
			   { passed+=$$4; failed+=$$6; ignored+=$$8;  measured+=$$10; filter+=$$12; } \
		 END   { printf "passed=%d; failed=%d; ignored=%d; measured=%d; filter=%d\n", passed, failed, ignored, measured, filter; }'
	@echo "################################################################################"
	@echo "test error:"
	@grep -A 2  'error\[' target/test.log || exit 0
	@echo "################################################################################"
	@echo "test result:"
	@grep '\.\.\. FAILED' target/test.log ||true
	@grep -q 'error\[' target/test.log; if [ $$? -eq 0 ] ; then exit 1; fi;
	@grep -q '\.\.\. FAILED' target/test.log; if [ $$? -eq 0 ] ; then exit 1; fi;

bench:
	@-rm -f target/bench.log
	@find chain  consensus  jsonrpc network share_libs tests              		  \
		  -name 'Cargo.toml'                                                      \
		  -exec cargo bench  --manifest-path {} 2>&1 \; |tee -a target/bench.log
	@echo "################################################################################"
	@echo "bench error:"
	@grep -A 2  'error\[' target/bench.log || exit 0
	@echo "################################################################################"
	@echo "bench result:"
	@grep '\.\.\. bench: ' target/bench.log||exit 0
	@grep -q 'error\[' target/bench.log; if [ $$? -eq 0 ] ; then exit 1; fi;

fmt:
	cargo fmt --all  -- --write-mode diff

cov:
	cargo cov test --all  --no-fail-fast
	cargo cov report --open

clean:
	rm -rf target/debug/
	rm -rf target/release/
