#! /usr/bin/env python3
# coding=utf-8

from jsonrpcclient.http_client import HTTPClient
import subprocess
import toml
import time
import os

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
        run_subprocess(f'bin/cita setup node/{i}')
        run_subprocess(f'bin/cita start node/{i} {log_level}')

def stop(node_number):
    """
    node_number: int
    """
    for i in range(node_number + 1):
        run_subprocess(f'bin/cita stop node/{i}')

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

def test_chain_higher_than_executor():
    p = run_subprocess("python3 ./scripts/create_cita_config.py create --nodes '127.0.0.1:4000,127.0.0.2:4001,127.0.0.3:4002,127.0.0.4:4003' --chain_name node > /dev/null")
    p.wait()
    modify_forever(3)
    start(3)
    time.sleep(30)
    for i in range(10):
        point_number = block_number()
        stop(0)
        remove_statedb(0)
        start(0)
        start_time = time.time()
        while True:
            new_node_block_height = block_number()
            if new_node_block_height and new_node_block_height > point_number:
                print(f"Current height is {new_node_block_height}, finish {i}")
                break
            else:
                print(f"Current height is {new_node_block_height}, wait...")
                time.sleep(3)
                duration_time = time.time() - start_time
                if duration_time > 60:
                    raise Exception("robustness test failure")

def test_executor_higher_than_chain():
    point_number = block_number()
    for i in range(10):
        with open("./node/0/.cita-executor.pid", "r") as file:
            executor_pid = file.read()
        with open("./node/0/.cita-chain.pid", "r") as file:
            chain_pid = file.read()

        run_subprocess(f"kill -9 {executor_pid}")
        time.sleep(3)
        run_subprocess(f"kill -9 {chain_pid}")

    start_time = time.time()
    while True:
        new_node_block_height = block_number()
        if new_node_block_height and new_node_block_height > point_number:
            print(f"Current height is {new_node_block_height}, finish {i}")
            break
        else:
            print(f"Current height is {new_node_block_height}, wait...")
            time.sleep(3)
            duration_time = time.time() - start_time
            if duration_time > 60:
                raise Exception("robustness test failure")
    stop(3)

if __name__ == "__main__":
    pwd = os.getcwd()
    os.chdir(f'{pwd}/target/install')
    test_chain_higher_than_executor()
    test_executor_higher_than_chain()
    clean()
