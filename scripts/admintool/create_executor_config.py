#!/usr/bin/env python
# coding=utf-8


import sys
import toml


def main():
    data = dict()
    data["prooftype"] = 2
    data["journaldb_type"] = "archive"
    data["grpc_port"] = int(sys.argv[2])
    config_path = sys.argv[1]
    with open(config_path, "w") as fil:
        toml.dump(data, fil)

if __name__ == '__main__':
    main()
