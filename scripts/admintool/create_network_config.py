#!/usr/bin/env python
# coding=utf-8

import os
import sys
import toml


def main():
    """Create a network configuration or update an existed one."""

    config_path = sys.argv[1]
    node_id = int(sys.argv[2])
    ip_list = (sys.argv[3]).split(',')
    update_existed = sys.argv[4] == 'true'

    size = len(ip_list)
    node_ip = ip_list[node_id].split(':')[0]
    node_port = int(ip_list[node_id].split(':')[1])

    if update_existed:
        update_existed_config(config_path, node_id, node_ip, node_port)
    else:
        # Create a new network configuration.
        with open(config_path, 'w') as fil:
            fil.write('id_card = {}\n'.format(node_id))
            fil.write('port = {}\n'.format(node_port))
            ip_list = filter(
                lambda x: x[0] != node_id, zip(range(size), ip_list))
            for (id_card, addr) in ip_list:
                addr_list = addr.split(':')
                fil.write('[[peers]]\n')
                fil.write('id_card = {}\n'.format(id_card))
                fil.write('ip = "{}"\n'.format(addr_list[0]))
                fil.write('port = {}\n'.format(addr_list[1]))


def update_existed_config(network_file, node_id, node_ip, node_port):
    """Update an existed network configuration."""
    if os.path.exists(network_file):
        with open(network_file, 'r') as fil:
            data = toml.load(fil)
        data['peers'].append({
            'id_card': node_id,
            'ip': node_ip,
            'port': node_port
        })
        with open(network_file, 'w') as fil:
            toml.dump(data, fil)


if __name__ == '__main__':
    main()
