#!/bin/bash
protoc blockchain.proto --rust_out .
protoc communication.proto --rust_out .
protoc auth.proto --rust_out .
protoc request.proto --rust_out .
protoc response.proto --rust_out .
protoc consensus.proto --rust_out .
protoc sync.proto --rust_out .

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

for i in `find . -name "*.rs"`
do
    if grep -q -e "Copyright 2015-2017 Parity Technologies" -e "Copyright 2016-2017 Cryptape Technologies" $i
    then
        echo "Ignoring the " $i
    else
        echo "Starting modify" $i
        (cat ../../../publishtool/notices | cat - $i > file1) && mv file1 $i
    fi
done
