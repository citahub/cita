#!/bin/bash
set -e
SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
BINARY_DIR=${SOURCE_DIR}/target/install

. ${SOURCE_DIR}/tests/integrate_test/util.sh
cd ${BINARY_DIR}

date
echo "###Stop CITA "
stop_all
date

exit 0

