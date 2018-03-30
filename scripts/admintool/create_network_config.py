#!/usr/bin/env python
# coding=utf-8

import os
import sys
import toml


def main():
    """
    Argv1: usually is the work path `cita/targte/install`
    Argv2: node number, such as 0, 1, 2...
    Argv3: ip length, it means the total number of nodes
    Argv4: ip list, such as "127.0.0.1:1000,127.0.0.1:1000"
    Argv5: if append new node, it will be true
    """

    nid = int(sys.argv[2])
    path = os.path.join(sys.argv[1])
    ip_list = (sys.argv[4]).split(',')
    port = ip_list[nid].split(':')[1]
    net_config_name = "network.toml"

    if len(sys.argv) == 6:
        insert_peer_config(nid, ip_list[nid].split(":")[0], port, path)

    size = int(sys.argv[3])
    dump_path = os.path.join(path, net_config_name)
    with open(dump_path, "w") as f:
        f.write("id_card = " + str(nid) + "\n")
        f.write("port = " + port + "\n")
        ids = range(size)
        ip_list = zip(ids, ip_list)
        del ip_list[nid]
        for (id, addr) in ip_list:
            addr_list = addr.split(':')
            f.write("[[peers]]" + "\n")
            f.write("id_card = " + str(id) + "\n")
            ip = addr_list[0]
            f.write("ip = \"" + ip + "\"\n")
            port = addr_list[1]
            f.write("port = " + port + "\n")


def insert_peer_config(new_id, ip, port, path):
    """
    Insert new node ip, port, id to network configuration of existing nodes

    :param new_id: new node id
    :param ip: new node ip
    :param port: new node port
    :param path: work path, usually is `cita/targte/install`
    """

    for n in range(new_id):
        network_file = os.path.join(path, 'node{}'.format(n), 'network.toml')
        if os.path.exists(network_file):
            with open(network_file) as f:
                data = toml.load(f)
            if len(data["peers"]) < new_id:
                with open(network_file, 'w') as f:
                    data['peers'].append({
                        'id_card': new_id,
                        'ip': ip,
                        'port': int(port)
                    })
                    toml.dump(data, f)


if __name__ == '__main__':
    main()
