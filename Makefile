# 1) native development
# 1.1) prerequirement
#sudo apt-get install --force-yes libsnappy1v5 libsnappy-dev  capnproto  libgoogle-perftools-dev  \
#    libssl-dev  libudev-dev  rabbitmq-server  google-perftools jq libsodium*  libzmq3-dev
# 1.2) make setup
# 1.3) make clean
# 1.4) make debug or make release
# 1.5) make test|bench|cov
# 1.6) refer to env.sh

################################################################################
CARGO=RUSTFLAGS='-F warnings' cargo

debug:
	$(CARGO) build --all
	mkdir -p admintool/release/bin
	cp -f .env admintool/release
	find target/debug -maxdepth 1 -perm -111 -type f -not \( -name "*-*" -prune \) -exec cp -f {} admintool/release/bin \;

release:
	$(CARGO) build --release --all
	mkdir -p admintool/release/bin
	cp -f .env admintool/release
	find target/release -maxdepth 1 -perm -111 -type f -not \( -name "*-*" -prune \) -exec cp -f {} admintool/release/bin \;


setup1:
	apt-get update -q
	apt-get install --allow-change-held-packages software-properties-common
	if [ $$(lsb_release -s -c) = "trusty" ]; then add-apt-repository ppa:chris-lea/libsodium -y ; fi;
	apt-get update -q
	apt-get build-dep build-essential
	apt-get install --allow-change-held-packages \
	 	pkg-config libsnappy-dev  capnproto  libgoogle-perftools-dev  libssl-dev libudev-dev  \
		rabbitmq-server  google-perftools jq libsodium* libzmq3-dev
	wget https://github.com/ethereum/solidity/releases/download/v0.4.15/solc-static-linux
	mv solc-static-linux /usr/local/bin/solc
	chmod +x /usr/local/bin/solc
	/etc/init.d/rabbitmq-server restart
	-rabbitmqctl add_vhost dev
	-rabbitmqctl set_permissions -p dev guest ".*" ".*" ".*"

setup2:
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2017-08-04
	- . ~/.cargo/env;cargo install --force --vers 0.9.0 rustfmt;
	pip install -r admintool/requirements.txt --user

test:
	$(CARGO) test --release --all --no-fail-fast 2>&1 |tee target/test.log
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

test_ed25519_blake2b:
	sed -i 's/\["secp256k1"\]/\["ed25519"\]/g' share_libs/crypto/Cargo.toml
	sed -i 's/\["sha3hash"\]/\["blake2bhash"\]/g' share_libs/util/Cargo.toml
	$(CARGO) test --release --all --no-fail-fast 2>&1 |tee target/test.log
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
	mkdir -p /tmp/cita/build
	cp Dockerfile-build /tmp/cita/build/Dockerfile
	docker build -t cita/build /tmp/cita/build
	docker run -ti -v ${PWD}:/sources -u cita cita/build bash -c  "make setup; make release"
	mkdir -p admintool/release/lib
	ldd admintool/release/bin/* |awk '{ if (match($$3, "/")) { print $$3; } }'|xargs -I {} cp  {} admintool/release/lib
	rm -f admintool/release/lib/libc.so*
	rm -f admintool/release/lib/libcom_err.so*
	rm -f admintool/release/lib/libcrypt.so*
	rm -f admintool/release/lib/libdl.so*
	rm -f admintool/release/lib/liblzma.so*
	rm -f admintool/release/lib/libm.so*
	rm -f admintool/release/lib/libpthread.so*
	rm -f admintool/release/lib/libresolv.so*
	rm -f admintool/release/lib/librt.so*
	rm -f admintool/release/lib/libz.so*
	cd admintool;./admintool.sh -c;cd -
	cp Dockerfile-run admintool/release/Dockerfile; docker build -t cita/run admintool/release
