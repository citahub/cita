#!/bin/bash

cd ..

for i in `find . -name "*.rs"`
do
    if grep -q -e "Copyright 2015-2017 Parity Technologies" -e "Copyright 2016-2017 Cryptape Technologies" $i 
    then
        echo "Ignoring the " $i
    else
        echo "Starting modify" $i
        (cat ./publishtool/notices | cat - $i > file1) && mv file1 $i
    fi
done
