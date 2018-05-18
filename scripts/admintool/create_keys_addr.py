#!/usr/bin/env python3
# coding=utf-8


import os
import sys
from subprocess import call


def main():
    """use to create public key and private key"""
    node_path = sys.argv[1]
    auth_path = sys.argv[2]
    command = sys.argv[3]
    secret_path = os.path.join(node_path, "privkey")
    cmd = "%s %s %s" % (command, secret_path, auth_path)
    call(cmd, shell=True)


if __name__ == '__main__':
    main()
