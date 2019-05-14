""" It's used to check the genensis file for compatibility. """
#!/usr/bin/env python3
# -*- coding:utf-8 -*-

import json
import argparse

NODE_KEYS = [
    '0x98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b',
    '0x0958fc78ca04a68fd52c5622ef2ca3cdb177cce96303c25fe1f487c6436a6dac'
]


def parse_arguments():
    """ parse the arguments: genesis file. """
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--genesis', required=True, help='The genesis file to be checked.')

    check_code_parser = parser.add_mutually_exclusive_group(required=False)
    check_code_parser.add_argument(
        '--check_code',
        dest='check_code',
        action='store_true',
        help='Check genesis contract code.')
    check_code_parser.add_argument(
        '--no_check_code',
        dest='check_code',
        action='store_false',
        help='Do not check genesis contract code.')
    parser.set_defaults(check_code=False)

    args = parser.parse_args()
    return args


def check(old, new, check_code):
    """ Check the new genesis is not changed. """
    old_alloc = old['alloc']
    new_alloc = new['alloc']

    for addr in old_alloc:
        # Ignore the acount with value: admin and authorities
        if 'value' in old_alloc[addr]:
            continue
        # Check the code
        if check_code and old_alloc[addr]['code'] != new_alloc[addr]['code']:
            return False
        # Check the storage
        storage = old_alloc[addr]['storage']
        for key in storage:
            # Ignore the node manager's constructor
            if key in NODE_KEYS:
                continue
            if key not in new_alloc[addr]['storage'] \
                    or storage[key] != new_alloc[addr]['storage'][key]:
                return False

    return True


def main():
    """ Read the genesis file and check. """
    args = parse_arguments()

    with open('scripts/config_tool/genesis/genesis.json', 'r') as gene:
        old = json.load(gene)
    with open(args.genesis, 'r') as gene:
        new = json.load(gene)

    assert check(old, new, args.check_code)


if __name__ == '__main__':
    main()
