FROM ubuntu:16.04

WORKDIR /cita
RUN	apt-get update -q											\
	&& apt-get install -y git make

ENV PATH=/root/.cargo/bin:$PATH
RUN	git clone https://github.com/urugang/cita build				\
	&& cd /cita/build											\
	&& make setup1												\
	&& make setup2												\
	&& make release												\
	&& pip install -r admintool/requirements.txt				\
	&& admintool/txtool/requirements_sudo.sh					\
	&& pip install -r admintool/txtool/requirements.txt			\
	&& cp -rf /cita/build/admintool/* /cita						\
	&& rm -rf /cita/build ~/.cargo ~/.rustup					\
	&& apt-get purge -y --auto-remove							\
 	&& rm -rf /var/lib/apt/lists
EXPOSE 1337 4000
CMD /cita/release/node0/cita setup 0; /cita/release/node0/cita start 0; /cita/release/node0/cita cpu;
