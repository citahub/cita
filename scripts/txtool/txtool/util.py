#! /usr/bin/env python
# coding=utf-8

import subprocess
import sys
import ntpath
import os.path
from ecdsa import SigningKey, SECP256k1

def which(program):
    import os
    def is_exe(fpath):
        return os.path.isfile(fpath) and os.access(fpath, os.X_OK)

    fpath, fname = os.path.split(program)
    if fpath:
        if is_exe(program):
            return program
    else:
        for path in os.environ["PATH"].split(os.pathsep):
            path = path.strip('"')
            exe_file = os.path.join(path, program)
            if is_exe(exe_file):
                return exe_file

    return None


def run_command(command):
    p = subprocess.Popen(command,
                         stdout=subprocess.PIPE,
                         stderr=subprocess.STDOUT)
    return iter(p.stdout.readline, b'')


def findDict(dictionary, keyName):
    """
    Find a value in nest dictionary with some key name not known.
    https://stackoverflow.com/questions/19688078/how-to-access-a-nested-dict-without-knowing-sub-dicts-names
    """

    if not isinstance(dictionary, dict):
        return None

    if keyName in dictionary:
        return dictionary[keyName]

    for subdict in dictionary.values():
        ret = findDict(subdict, keyName)
        if ret:
            return ret


def stringToBytes(convertedString):
    if sys.version_info < (3, 0):
        return bytes(convertedString)
    else:
        return bytes(convertedString, 'utf8')

def path_leaf(path):
    head, tail = ntpath.split(path)
    return tail or ntpath.basename(head)

def hex2bytes(hex_string):
    return bytes(bytearray.fromhex(hex_string))


def remove_hex_0x(hex_string):
    result = hex_string
    if hex_string is not None:
        if hex_string.startswith('0x') or hex_string.startswith('0X'):
            result = hex_string[2:]

    return result


def add_hex_0x(hex_string):
    result = hex_string

    if hex_string is not None:
        starts_with_0x = hex_string.startswith('0x') or hex_string.startswith('0X')
        if not starts_with_0x:
            result = '0x' + hex_string

    return result


def _solidity_path():
    result = None
    compile_path = os.path.dirname(os.path.abspath(__file__))
    solidity_path = os.path.join(compile_path, "solidity")
    if os.path.exists(solidity_path):
        result = solidity_path

    return result


def _just_filename(file_name):
    basepath = os.path.dirname(file_name)
    return basepath is None or basepath == ""


def solidity_file_dirname(solidity_filename):
    just_filename = _just_filename(solidity_filename)
    if just_filename:
        solidity_path = _solidity_path()
        file_path = os.path.join(solidity_path, solidity_filename)
        is_exist = os.path.isfile(file_path)
        if is_exist:
            full_path_name =  os.path.abspath(file_path)
            return (solidity_filename,  os.path.dirname(file_path), full_path_name)
        else:
            print("solidity file {} may be wrong format or not in folder 'solidity'".format(solidity_filename))
            return None
    else:
        return (os.path.basename(solidity_filename), os.path.dirname(solidity_filename), solidity_filename)

def recover_pub(signkey):
    if not isinstance(signkey, str):
        raise ValueError("Sign key is not a string.")
    
    sk = SigningKey.from_string(signkey, curve=SECP256k1)
    pub = sk.get_verifying_key().to_string()
    return pub