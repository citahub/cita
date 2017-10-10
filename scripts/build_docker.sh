#!/bin/bash
PROJECT_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
if [ -z ${PROJECT_DIR} ] ; then
    echo "failed to locate project directory"
fi
echo ${PROJECT_DIR}
cd ${PROJECT_DIR}

#docker build --tag cryptape/cita-run --file scripts/Dockerfile-run .
docker build --tag cryptape/cita --file scripts/Dockerfile .
