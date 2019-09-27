# Docker image for CITA release

## Build image

 Script is comming soon!

## Usage

### Using docker command

* Create a CITA chain config.

```shell
docker run -v `pwd`:/opt/cita-run cita/cita-release:1.0.0-secp256k1-sha3 cita create --super_admin "0x37d1c7449bfe76fe9c445e626da06265e9377601" --nodes "127.0.0.1:4000"
```

Tips: CITA image uses `/opt/cita-run` as its default work_dir, so you should mount `pwd` to it.

* Setup & run CITA

```shell
docker run -d -p 1337:1337 -v `pwd`:/opt/cita-run cita/cita-release:1.0.0-secp256k1-sha3 /bin/bash -c 'cita setup test-chain/0 && cita start test-chain/0 && sleep infinity'
```

Cool! Just two simple command, you have build a CITA blockchain in you computer.
Enjoy it!

### Using docker-compose

```shell
cd sample && docker-compose up
```