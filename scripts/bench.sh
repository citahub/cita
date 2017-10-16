#!/bin/bash
set -e


SOURCE_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
cd  ${SOURCE_DIR}
source ~/.cargo/env


echo "################################################################################"
echo "1) setup"
git status
git rev-parse HEAD
scripts/config_rabbitmq.sh
# For native machine, skip this step.
# scripts/install_develop.sh

echo "################################################################################"
echo "3) build"
time make release

echo "################################################################################"
echo "4) benchmark in develop"
time make bench

echo "################################################################################"
echo "5) benchmark in deploy"
cd tests/wrk_benchmark_test/
echo "5.1) chain_performance"
rm -rf ./data
echo "performance test for create"
time bash tests/wrk_benchmark_test/chain_performance.sh 1 10000 0 0
rm -rf ./data
echo "performance test for call"
time bash tests/wrk_benchmark_test/chain_performance.sh 2 10000 0 0
rm -rf ./data
echo "performance test for store"
time bash tests/wrk_benchmark_test/chain_performance.sh 3 10000 0 0

echo "################################################################################"
echo "6) archive result"
now=$(date --iso-8601=minutes)
mkdir -p ${SOURCE_DIR}/../${now}_${BASHPID}
cp -rf ${SOURCE_DIR}/target/install  ${SOURCE_DIR}/../${now}_${BASHPID}/
cp -rf ${SOURCE_DIR}/target/*.log  ${SOURCE_DIR}/../${now}_${BASHPID}/
