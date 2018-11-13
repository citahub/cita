#! /usr/bin/env python3
# coding=utf-8

import os
import subprocess
import time

import toml
from jsonrpcclient.http_client import HTTPClient


def block_number(host="127.0.0.1", port=1337):
    """
    url: str
    port: int
    """
    url = "http://" + host + ":" + str(port)
    try:
        response = HTTPClient(url).request("blockNumber", [])
        return int(response, 16)
    except:
        return None


def run_subprocess(cmd, shell=True):
    """
    cmd: str, style like "ls -al"
    """
    return subprocess.Popen(cmd, shell=shell, stdout=subprocess.PIPE)


def start(node_number, log_level=""):
    """
    node_number: int
    log_level: str
    """
    for i in range(node_number + 1):
        p = run_subprocess(f'bin/cita setup node/{i}')
        p.wait()
        run_subprocess(f'bin/cita start node/{i} {log_level}')


def stop(node_number):
    """
    node_number: int
    """
    for i in range(node_number + 1):
        p = run_subprocess(f'bin/cita stop node/{i}')
        p.wait()


def clean():
    run_subprocess("rm node/ -rf")


def modify_forever(node_number):
    """
    node_number: int
    """
    for i in range(node_number + 1):
        with open(f"./node/{i}/forever.toml", "r") as file:
            forever_conf = toml.load(file)
            forever_conf["process"][-1]["respawn"] = 10000
            forever_conf["process"][-2]["respawn"] = 10000
        with open(f"./node/{i}/forever.toml", "w") as file:
            toml.dump(forever_conf, file)


def remove_statedb(node_number):
    """
    node_number: int
    """
    for i in range(node_number + 1):
        run_subprocess(f'rm ./node/{i}/data/statedb/ -rf')


def kill_process(always, occasionally):
    """
    :param always: path, str
    :param occasionally: path, str
    :return: None
    """
    for i in range(50):
        if os.path.exists(always):
            with open(always, "r") as file:
                always_kill = file.read()
            run_subprocess(f"kill -9 {always_kill}")

        if i % 4 == 0 and os.path.exists(occasionally):
            with open(occasionally, "r") as file:
                occasionally_kill = file.read()
            run_subprocess(f"kill -9 {occasionally_kill}")
        time.sleep(0.3)


def prepare():
    p = run_subprocess(
        "python3 ./scripts/create_cita_config.py create --super_admin '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523' --nodes '127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003' --chain_name node > /dev/null")
    p.wait()
    modify_forever(3)
    start(3)
    time.sleep(30)


def test_chain_higher_than_executor():
    for i in range(10):
        point_number = block_number()
        print(f"point height is {point_number}")
        stop(0)
        remove_statedb(0)
        start(0)
        start_time = time.time()
        while True:
            new_node_block_height = block_number()
            if new_node_block_height and new_node_block_height > point_number + 2:
                print(f"Current height is {new_node_block_height}, finish {i}")
                break
            else:
                print(f"Current height is {new_node_block_height}, wait...")
                time.sleep(3)
                duration_time = time.time() - start_time
                if duration_time > 60:
                    raise Exception("robustness test failure")


def test_executor_higher_than_chain():
    kill_process('./node/0/.cita-executor.pid', "./node/0/.cita-chain.pid")
    kill_process("./node/0/.cita-chain.pid", './node/0/.cita-executor.pid')

    time.sleep(6)
    point_number = block_number(port=1339)
    print(f"point height is {point_number}")
    start_time = time.time()
    while True:
        new_node_block_height = block_number()
        if new_node_block_height and new_node_block_height > point_number + 10:
            print(f"Current height is {new_node_block_height}, finish")
            break
        else:
            print(f"Current height is {new_node_block_height}, wait...")
            time.sleep(3)
            duration_time = time.time() - start_time
            if duration_time > 60:
                raise Exception("robustness test failure")


if __name__ == "__main__":
    pwd = os.getcwd()
    os.chdir(f'{pwd}/target/install')

    print("step 0: prepare")
    clean()
    prepare()

    print("step 1: Chain higher than Executor")
    test_chain_higher_than_executor()

    print("step 2: Executor higher than Chain")
    test_executor_higher_than_chain()

    print("step 3: stop")
    stop(3)

    print("step 4: clean up")
    clean()
