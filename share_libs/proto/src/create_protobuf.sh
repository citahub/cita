#!/bin/bash
protoc blockchain.proto --rust_out .
protoc communication.proto --rust_out .
protoc request.proto --rust_out .
case "$OSTYPE" in
    darwin*)  
        sed -ig 's/    result: ::std::option::Option<Response_oneof_result>,/    pub result: ::std::option::Option<Response_oneof_result>,/g' request.rs
        sed -ig 's/    req: ::std::option::Option<Request_oneof_req>,/    pub req: ::std::option::Option<Request_oneof_req>,/g' request.rs
        ;; 
    *)       
        sed -i 's/    result: ::std::option::Option<Response_oneof_result>,/    pub result: ::std::option::Option<Response_oneof_result>,/g' request.rs
        sed -i 's/    req: ::std::option::Option<Request_oneof_req>,/    pub req: ::std::option::Option<Request_oneof_req>,/g' request.rs
        ;;
esac
protoc auth.proto --rust_out .
