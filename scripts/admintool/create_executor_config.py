#!/usr/bin/env python
# coding=utf-8


import toml
import os
import sys


def main():
    data = dict()
    data["prooftype"] = 2
    data["journaldb_type"] = "archive"
    data["grpc_port"] = int(sys.argv[1])
    path = sys.argv[2]
    dump_path = os.path.join(path, "executor.toml")
    f = open(dump_path, "w")
    toml.dump(data, f)
    f.close()

if __name__ == '__main__':
    main()
