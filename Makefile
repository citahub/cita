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
	$(CARGO) test --all 2>&1 |tee target/test.log

bench:
	-rm target/bench.log
	cargo bench --all --no-run |tee target/bench.log
	cargo bench --all --jobs 1 |tee -a target/bench.log

fmt:
	cargo fmt --all  -- --write-mode diff

cov:
	cargo cov test --all
	cargo cov report --open

clean:
	rm -rf target/debug/
	rm -rf target/release/

clippy:
	cargo build --features clippy --all
