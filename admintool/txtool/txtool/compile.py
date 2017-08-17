#!/usr/bin/env python  
# coding=utf-8  

import argparse
import os.path
from solc import compile_standard
from pathlib import Path
from util import findDict, run_command, path_leaf, add_hex_0x, solidity_file_dirname
import simplejson

def save_abi(abi):
    abiFile = open("../output/compiled/abi", "w+")
    abiFile.write(abi)
    abiFile.close()


def save_bincode(code):
    code_file = open("../output/compiled/bytecode", "w+")
    code_file.write(code)
    code_file.close()


def save_functions(data):
    data_file = open("../output/compiled/functions", "w+")
    simplejson.dump(data, data_file)
    data_file.close()


def read_functions():
    with open("../output/compiled/functions", "r") as datafile:
       return simplejson.load(datafile)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--source', help="Solidity source code")
    parser.add_argument('-f', '--file', help="solidity file name with full path. Like ~/examplefolder/test.solc")
    parser.add_argument('-p', '--procedure', help="Solidity function name.")
    parsed = parser.parse_args()

    compile_path = Path("../output/compiled")
    if not compile_path.is_dir():
        command = 'mkdir -p ../output/compiled'.split()
        for line in run_command(command):
            print(line)

    if parsed.source:
        solidity_source = parsed.source
        output = compile_standard({
            'language': 'Solidity',
            'sources': {'standard.sol': {'content': solidity_source}}
        })
        print "abi保存到output/compiled/abi文件中"
        save_abi(str(findDict(output['contracts'], 'abi')))
        print "bincode保存到output/compiled/bytecode"
        save_bincode(str(findDict(output, 'object')))
    elif parsed.file:
        # TODO: 错误处理 文件格式检查
        print parsed.file
        paths = solidity_file_dirname(parsed.file)
        if paths is not None:
            filename, basepath, fullpath = paths
            output = compile_standard({
                'language': 'Solidity',
                'sources': {filename: {'urls': [fullpath]}},
            }, allow_paths=basepath)
            print "abi保存到output/compiled/abi文件中"
            save_abi(str(findDict(output['contracts'], 'abi')))
            print "bincode保存到output/compiled/bytecode"
            save_bincode(str(findDict(output, 'object')))
        
            save_functions(findDict(output, 'methodIdentifiers'))
    elif parsed.procedure:
        key = parsed.procedure
        functions = read_functions()
        if functions is None or functions == "":
            print "Compile Solidity source first."
        else:
            data = findDict(functions, key)
            print add_hex_0x(data)     


if __name__ == "__main__":
    main()
