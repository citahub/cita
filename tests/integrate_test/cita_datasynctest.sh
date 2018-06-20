#!/bin/bash

stop_node3() {
    cd ${CUR_PATH}/../../admintool/release/node3
    ./cita stop 3
}

start_node3() {
    cd ${CUR_PATH}/../../admintool/release/node3
    ./cita start 3 &
}

get_height(){
    nodeid=$1
    if [ ! -n "$nodeid" ]; then
        nodeid=0
    fi
    h=`${CUR_PATH}/blockNumber.sh 127.0.0.1 $((1337+${nodeid}))`
    h=$(echo $h | sed 's/\"//g')
    echo $((h))    
}

COUNT=$1
if [ ! -n "$COUNT" ]; then
    COUNT=100
fi

CUR_PATH=$(cd `dirname $0`; pwd)

${CUR_PATH}/cita_start.sh

stop_node3
echo "###Stop node3"

height=$(get_height)
hi=$[height+COUNT]

echo "current height:$height"
echo "aim height:$hi"

while [ $height -le $hi ]
do
    height=$(get_height)
    sleep 3
done


echo "###start node3"
start_node3

start=$(date +%s) 

num_3=$(get_height 3)

while(($num_3<$hi))
do
    num_3=$(get_height 3)
done

end=$(date +%s)

time=$((end - start))

echo "Syn $COUNT block spent time:$time"

${CUR_PATH}/cita_stop.sh