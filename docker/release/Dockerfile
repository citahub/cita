FROM cita/cita-run:ubuntu-18.04-20190829
LABEL maintainer="Cryptape Technologies <contact@cryptape.com>"

## Install CITA release here
ARG ENCRYPTION_ALG=secp256k1
ARG HASH_ALG=sha3
RUN mkdir -p /opt/cita

WORKDIR /opt/cita
COPY cita_${ENCRYPTION_ALG}_${HASH_ALG}.tar.gz .

# Keep tar package here for md5 test
RUN tar -xf cita_${ENCRYPTION_ALG}_${HASH_ALG}.tar.gz \
    && mv cita_${ENCRYPTION_ALG}_${HASH_ALG}/* /opt/cita \
    && rm -rf cita_${ENCRYPTION_ALG}_${HASH_ALG}

# Set CITA_HOME
ENV CITA_HOME=/opt/cita
# Set PATH to cita binary
ENV PATH=/opt/cita/bin:${PATH}
## End of install CITA release

COPY entrypoint.sh /usr/bin/
RUN chmod +x /usr/bin/entrypoint.sh

WORKDIR /opt/cita-run

ENTRYPOINT ["/usr/bin/entrypoint.sh"]
CMD ["cita", "bebop", "help"]
