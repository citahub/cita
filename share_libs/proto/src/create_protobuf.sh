#!/bin/bash
protoc blockchain.proto --rust_out .
protoc communication.proto --rust_out .
