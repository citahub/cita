# 1) native development
# 1.1) sudo make setup1
# 1.2) make setup2
# 1.3) make clean
# 1.4) make debug or make release
# 1.5) make test|bench|cov|fmt
# 1.6) refer to admintool/cita

################################################################################
CARGO=RUSTFLAGS='-F warnings' cargo

debug:
	$(CARGO) build --all
	mkdir -p admintool/release/bin 
	find target/debug -maxdepth 1 -perm -111 -type f -not \( -name "*-*" -prune \) -exec cp -f {} admintool/release/bin \;

release:
	$(CARGO) build --release --all
	mkdir -p admintool/release/bin
	find target/release -maxdepth 1 -perm -111 -type f -not \( -name "*-*" -prune \) -exec cp -f {} admintool/release/bin \;


setup1:
	echo "deb http://ppa.launchpad.net/ethereum/ethereum/ubuntu xenial main" > /etc/apt/sources.list.d/ethereum-ubuntu-ethereum-xenial.list
	apt-get update -q
	apt-get install -y --allow-unauthenticated \
        libsnappy-dev  capnproto  libgoogle-perftools-dev  libssl-dev libudev-dev  \
        rabbitmq-server  google-perftools jq solc \
        openssl  libyaml-dev python-pip python
	/etc/init.d/rabbitmq-server restart
	-rabbitmqctl add_vhost dev
	-rabbitmqctl set_permissions -p dev guest ".*" ".*" ".*"

setup2:
	-rm -rf ~/.local/lib/python2.7
	pip install secp256k1 bitcoin ecdsa ethereum jsonrpcclient pathlib protobuf py_solc pyping pysha3 pysha3 rlp secp256k1 simplejson
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2017-08-04
	- . ~/.cargo/env;cargo install --force --vers 0.9.0 rustfmt; cargo install --force --vers 0.0.1 cov
	cp .env admintool


test:
	$(CARGO) test --release --all --no-fail-fast |tee target/test.log
	@grep 'test result' target/test.log |awk '\
         BEGIN { passed=0; failed=0; ignored=0; measured=0; filter=0; } \
               { passed+=$$4; failed+=$$6; ignored+=$$8;  measured+=$$10; filter+=$$12; } \
         END   { printf "passed=%d; failed=%d; ignored=%d; measured=%d; filter=%d\n", passed, failed, ignored, measured, filter; }'
	@echo "################################################################################"
	@echo "failed testcase:"
	@grep '\.\.\. FAILED' target/test.log ||true
	@grep -q '\.\.\. FAILED' target/bench.log; if [ $$? -eq 0 ] ; then exit 1; fi;

bench:
	-rm -f target/bench.log
	find chain  consensus  devtools jsonrpc network share_libs tests              \
          -name 'Cargo.toml'                                                      \
          -not -path 'share_libs/parity/*'                                        \
          -not -path 'consensus/raft/*'                                           \
          -not -path 'consensus/capnp_nonblock/*'                                 \
          -exec cargo bench --manifest-path {} 2>&1 \; |tee -a target/bench.log
	@echo "################################################################################"
	@echo "bench error:"
	@grep -A 2  'error\[' target/bench.log || exit 0
	@echo "################################################################################"
	@echo "bench result:"
	@grep '\.\.\. ' target/bench.log|grep -v 'ignored'|grep -v 'bench_execute_block' || exit 0
	@grep -A 4 'libchain::chain::tests::bench_execute_block' target/bench.log || exit 0
	grep -q 'error\[' target/bench.log; if [ $$? -eq 0 ] ; then exit 1; fi;

fmt:
	cargo fmt --all

cov:
	cargo cov test --all --no-fail-fast
	cargo cov report --open

clean:
	rm -rf target

docker:
	mkdir -p /tmp/cita/
	cp Dockerfile /tmp/cita/
	cd /tmp/cita; docker build -t cryptape/cita -f Dockerfile /tmp/cita/
