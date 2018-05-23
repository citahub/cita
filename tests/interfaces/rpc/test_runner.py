#!/usr/bin/env python3

import os
import sys
import json
import copy
import argparse
import shlex
import subprocess

import jsonschema
import requests

DEFAULT_RPC_URL = 'http://127.0.0.1:1337'


class FixResolver(jsonschema.RefResolver):
    def __init__(self, schema_data, schema_base_uri):
        super(FixResolver, self).__init__(
            base_uri=schema_base_uri, referrer=None)
        self.store[schema_base_uri] = schema_data


def jq_check(assertion, data):
    output = subprocess.Popen(shlex.split('echo \'{}\' | jq "{}"'.format(
        json.dumps(data), assertion)))
    return output.strip() == 'true'


class TestRunner(object):
    def __init__(self, rpc_url, fast_fail):
        self.rpc_url = rpc_url
        self.fast_fail = fast_fail
        self.session = requests.Session()
        self.assertion_fail_count = 0
        self.assertion_count = 0

    def assertion_fail(self, force=False):
        if self.assertion_fail_count and (force or self.fast_fail):
            print('>> Assertion Failed {}!!!'.format(
                '' if self.fast_fail else '({}/{} times)'.format(
                    self.assertion_fail_count,
                    self.assertion_count)))
            sys.exit(-1)

    def run_all(self, directory):
        directory = directory or ''
        tests_dir = os.path.join(directory, 'tests')
        for filename in os.listdir(tests_dir):
            test_path = os.path.join(tests_dir, filename)
            self.run_method(test_path)

    def run_method(self, test_path):
        with open(test_path) as f:
            test_data = json.load(f)

        tests_dir = os.path.dirname(test_path)
        print('[Test Method]: {}'.format(test_data['title']))
        schema_path = os.path.join(tests_dir, test_data['schema']['$ref'])
        with open(schema_path) as f:
            schema_data = json.load(f)

        for i, test_case in enumerate(test_data['tests']):
            self.run_test_case(
                test_case,
                schema_data['request'],
                schema_data['response'],
                schema_path,
            )
        print('=' * 60)

    def run_test_case(self, test_case, request_schema, response_schema,
                      schema_path):
        print('  * [Test Case]: {}'.format(test_case['title']))
        schema_base_uri = 'file://{}/'.format(
            os.path.dirname(os.path.abspath(schema_path)))
        request_payload = {
            'jsonrpc': '2.0',
            'id': 1,
            'method': test_case['request']['method'],
            'params': test_case['request']['params'],
        }
        request_should_fail_schema = test_case['request'].get(
            'shouldFailSchema', False)
        if not request_should_fail_schema:
            request_resolver = FixResolver(request_schema, schema_base_uri)
            jsonschema.validate(
                request_payload,
                request_schema,
                resolver=request_resolver,
            )

        expected_response = copy.deepcopy(test_case['expectedResponse'])
        expected_response['jsonrpc'] = "2.0"
        expected_response['id'] = 1
        response_resolver = FixResolver(response_schema, schema_base_uri)
        jsonschema.validate(
            expected_response,
            response_schema,
            resolver=response_resolver,
        )

        resp = self.session.post(self.rpc_url, json=request_payload)
        assert_data = {
            'receivedResponse': resp.json(),
            'expectedResponse': expected_response
        }
        for assertion in test_case['asserts']:
            assertion_result = jq_check(assertion['program'], assert_data)
            print("    - [Assertion]: {} => {}".format(
                assertion['description'], assertion_result))
            if assertion_result is False:
                print('[receivedResponse]:')
                print(json.dumps(
                    assert_data['receivedResponse'].get('result'), indent=2))
                print('-' * 10)
                print('[expectedResponse]:')
                print(json.dumps(
                    assert_data['expectedResponse'].get('result'), indent=2))
                self.assertion_fail_count += 1
            self.assertion_count += 1
            self.assertion_fail()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--rpc-url',
        metavar='URL',
        default=DEFAULT_RPC_URL,
        help=u'JSONRPC server URL [default: {}]'.format(DEFAULT_RPC_URL))
    parser.add_argument(
        '--directory', help='The tests/schemas directory (run all methods)')
    parser.add_argument(
        '--tests', metavar='PATH', nargs='*', help=u'The test file path list')
    parser.add_argument(
        '--fast-fail',
        action='store_true',
        help=u'Shall we stop when an assertion failed [default: false]')
    args = parser.parse_args()

    runner = TestRunner(args.rpc_url, args.fast_fail)
    if args.tests:
        for test_path in args.tests:
            runner.run_method(test_path)
    else:
        runner.run_all(args.directory)

    if runner.assertion_fail_count:
        runner.assertion_fail(force=True)
    else:
        print('>>> All tests run successfully!')


if __name__ == '__main__':
    main()
