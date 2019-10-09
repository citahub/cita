#!/usr/bin/env python3
# coding=utf-8

import argparse
import os.path
from solc import compile_standard, compile_files, compile_source
from pathlib import Path
from util import findDict, run_command, add_hex_0x, solidity_file_dirname
from log import logger
import simplejson


def save_abi(abi):
    with open("../output/compiled/abi", "w+") as abifile:
        simplejson.dump(abi, abifile, indent=4)


def save_bincode(code):
    with open("../output/compiled/bytecode", "w+") as code_file:
        code_file.write(code)


def save_functions(data):
    with open("../output/compiled/functions", "w+") as func_file:
        simplejson.dump(data, func_file, indent=4)


def read_functions():
    with open("../output/compiled/functions", "r") as datafile:
        return simplejson.load(datafile)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--source', help="Solidity source code")
    parser.add_argument(
        '-f',
        '--file',
        help="solidity file name with full path. Like ~/examplefolder/test.solc"
    )
    parser.add_argument('-p', '--procedure', help="Solidity function name.")
    parsed = parser.parse_args()

    compile_path = Path("../output/compiled")
    if not compile_path.is_dir():
        command = 'mkdir -p ../output/compiled'.split()
        for line in run_command(command):
            logger.debug(line)

    if parsed.source:
        solidity_source = parsed.source
        output = compile_standard({
            'language': 'Solidity',
            'sources': {
                'standard.sol': {
                    'content': solidity_source
                }
            }
        })
        logger.info("contract abi stored in 'output/compiled/bytecode'")
        save_abi(findDict(output['contracts'], 'abi'))
        logger.info("function signature stored in 'output/compiled/functions'")
        save_functions(findDict(output, 'methodIdentifiers'))

        bytecode = compile_source(parsed.source)
        logger.info("contract bytecode stored in 'output/compiled/bytecode'")
        save_bincode(str(findDict(bytecode, 'bin')))
        logger.debug(str(findDict(bytecode, 'bin')))

    elif parsed.file:
        # TODO: error handling check file format
        logger.debug(parsed.file)
        paths = solidity_file_dirname(parsed.file)
        origin_path = os.getcwd()
        if paths is not None:
            filename, basepath, fullpath = paths
            os.chdir(basepath)
            output = compile_standard({
                'language': 'Solidity',
                'sources': {
                    filename: {
                        'urls': [fullpath]
                    }
                },
            },
                                      allow_paths=basepath)

            os.chdir(origin_path)
            logger.info("contract abi stored in 'output/compiled/abi'")
            save_abi(findDict(output['contracts'], 'abi'))
            logger.info(
                "function signature stored in 'output/compiled/functions'")
            save_functions(findDict(output, 'methodIdentifiers'))

            bytecode = compile_files([parsed.file])
            logger.info(
                "contract bytecode stored in 'output/compiled/bytecode'")
            save_bincode(str(findDict(bytecode, 'bin')))
            logger.info(str(findDict(bytecode, 'bin')))

    elif parsed.procedure:
        key = parsed.procedure
        functions = read_functions()
        if functions is None or functions == "":
            logger.info("Compile Solidity source first.")
        else:
            data = findDict(functions, key)
            logger.debug(add_hex_0x(data))


if __name__ == "__main__":
    main()
