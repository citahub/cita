#!/bin/bash
# usage: scripts/build_image_from_source [debug|release] [bootstrap]

set -e

sudo() {
	set -o noglob
	if [ "$(whoami)" == "root" ] ; then
		$*
	else
		/usr/bin/sudo $*
	fi
	set +o noglob
}

SOURCE_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
BINARY_DIR=${SOURCE_DIR}/target/install

build_type=$1
bootstrap=$2
echo "1) build image for build environment"
if [ "${bootstrap}" = "bootstrap" ] ; then
	cd ${SOURCE_DIR}/scripts
	sudo docker build -f Dockerfile-build -t cryptape/cita-build .
	sudo docker build -f Dockerfile-build-speedup -t cryptape/cita-build .
fi

echo "2) build target/install using image cita-build"
cd ${SOURCE_DIR}
if [ "${build_type}" = "release" ]; then
	(sudo docker ps -a |grep cita-build-release >/dev/null) || (
		sudo docker create --name cita-build-release \
			 -v $PWD:/source cryptape/cita-build \
			 bash  --verbose -c	'/source/scripts/bench.sh')
	sudo docker start -i cita-build-release
else
	(sudo docker ps -a |grep cita-build-debug >/dev/null) || (
		sudo docker create --name cita-build-debug \
             -v $PWD:/source cryptape/cita-build \
             bash  -c '/source/scripts/ci.sh' )
    sudo docker start -a cita-build-debug
fi

echo "3) build deployable image from binary"
cd ${BINARY_DIR}
if [ "${bootstrap}" = "bootstrap" ] ; then
    ./scripts/build_image_from_binary.sh all
else
    ./scripts/build_image_from_binary.sh
fi
