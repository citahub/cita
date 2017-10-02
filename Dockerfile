FROM cryptape/cita-build


COPY . /source
WORKDIR /source
RUN cargo build --release --all \
    && scripts/release.sh \
    && cp target/install /cita

RUN apt-get remove -y libssl-dev build-essential pkg-config  libsnappy-dev  libgoogle-perftools-dev   libsodium-dev libzmq3-dev \
    && rm -rf ~/.rustup ~/.cargo

ENV PATH $PATH:/cita/bin
WORKDIR /data
