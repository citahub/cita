#!/usr/bin/env python
# coding=utf-8

import os
import sys


def main():
    nid = int(sys.argv[2])
    path = os.path.join(sys.argv[1])
    ip_list = (sys.argv[4]).split(',')
    port = ip_list[nid].split(':')[1]
    net_config_name = "network.toml"
    size = int(sys.argv[3])
    dump_path = os.path.join(path, net_config_name)
    with open(dump_path, "w") as f:
        f.write("id_card = " + str(nid) + "\n")
        f.write("port = " + port + "\n")
        f.write("max_peer = " + str(size - 1) + "\n")
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


if __name__ == '__main__':
    main()
