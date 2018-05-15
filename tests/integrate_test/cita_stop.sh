#!/bin/bash
set -e
kafka=$1
if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/../..)
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install

. ${SOURCE_DIR}/tests/integrate_test/util.sh
cd ${BINARY_DIR}

date
echo "###Stop CITA "
stop_all
if [ "$kafka" == "kafka" ]; then
    echo "###Stop kafka"
    $SOURCE_DIR/tests/integrate_test/kafka_stop.sh $BINARY_DIR
fi
date

exit 0

