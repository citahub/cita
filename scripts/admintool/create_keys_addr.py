#!/usr/bin/env python
# coding=utf-8


import os
import sys
from subprocess import call

# use to create public key and private key
# Argv1: usually is the work path `cita/targte/install`
# Argv2: node number, such as 0, 1, 2...
# Argv2: the executable file name. "create_key_addr"
def main():
    if len(sys.argv) == 2:
        path = sys.argv[1]
    else:
        path = os.path.join(sys.argv[1], "node" + sys.argv[2])
    command = sys.argv[3]
    dump_path = os.path.join(path, "privkey")
    auth_path = os.path.join(sys.argv[1], "authorities")
    cmd = "%s %s %s" % (command, dump_path, auth_path)
    call(cmd, shell=True)

if __name__ == '__main__':
    main()
