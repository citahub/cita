#!/usr/bin/env python
# coding=utf-8


import os
import sys
from subprocess import call

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
