#!/bin/bash
protoc blockchain.proto --rust_out .
protoc communication.proto --rust_out .
protoc auth.proto --rust_out .
protoc request.proto --rust_out .
protoc response.proto --rust_out .

case "$OSTYPE" in
    darwin*)  
        sed -ig 's/    data: ::std::option::Option<Response_oneof_data>,/    pub data: ::std::option::Option<Response_oneof_data>,/g' response.rs
        sed -ig 's/    req: ::std::option::Option<Request_oneof_req>,/    pub req: ::std::option::Option<Request_oneof_req>,/g' request.rs
        ;; 
    *)       
        sed -i 's/    data: ::std::option::Option<Response_oneof_data>,/    pub data: ::std::option::Option<Response_oneof_data>,/g' response.rs
        sed -i 's/    req: ::std::option::Option<Request_oneof_req>,/    pub req: ::std::option::Option<Request_oneof_req>,/g' request.rs
        ;;
esac

