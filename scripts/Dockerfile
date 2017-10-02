FROM cryptape/cita-run

COPY . /cita
ENV PATH $PATH:/cita/bin

WORKDIR /data

RUN rm -rf /cita/scripts/admintool/release \
    &&ln -srf /data /cita/scripts/admintool/release
