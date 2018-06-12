#!/usr/bin/env bash

protoc --proto_path=proto --python_out=. blockchain.proto
