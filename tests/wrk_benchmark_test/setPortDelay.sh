function setdockerdelay()
{
    sandboxKey=$(docker inspect '--format={{ .NetworkSettings.SandboxKey }}' $1)
    echo $sandboxKey
    veth_id=$(sudo nsenter --net=$sandboxKey ethtool -S eth0 |grep peer_ifindex | awk -F: '{print $2}')
    echo $veth_id
    virnet=$(ip a|grep $veth_id | awk -F: '{print $2}' | awk -F@ '{print $1}')
    echo $virnet

    add="add"

    num=$(tc qdisc show | grep $virnet | grep netem)

    if [ "$2" ] && [ $2 == $add ];then
        sudo tc qdisc add dev $virnet root netem delay 1000ms 10ms
    elif [ num == 1 ];then
        sudo tc qdisc del dev $virnet root netem delay 1000ms 10ms
    fi
}

function setportdelay()
{
    echo $port $4
    sudo tc qdisc add dev lo root handle 1: prio bands 4
    if [ "$4" == "delay" ]; then
        sudo tc qdisc add dev lo parent 1:4 handle 40: netem delay $1ms
    elif [ "$4" == "loss" ]; then
        sudo tc qdisc add dev lo parent 1:4 handle 40: netem loss $1%
    elif [ "$4" == "dup" ]; then
        sudo tc qdisc add dev lo parent 1:4 handle 40: netem duplicate $1%
    elif [ "$4" == "corrupt" ]; then
        sudo tc qdisc add dev lo parent 1:4 handle 40: netem corrupt $1%
    fi
    sudo tc filter add dev lo protocol ip parent 1:0 prio 4 u32 match ip dport $2 0xffff flowid 1:4
    sleep $3
    sudo tc filter del dev lo pref 4
    sudo tc qdisc del dev lo root
}


function random()
{
    min=$1;
    max=$2-$1;
    num=$(echo $RANDOM)
    ((retnum=num%max+min));
    echo $retnum;
}

flag=$(tc -s qdisc  show dev lo | grep "qdisc prio" | wc -l)

if [ $flag -eq 1 ]; then
    sudo tc qdisc del dev lo root
fi
run=0
trap run=1 2 

while :
do
    index=$(random 0 4);
    port=$[$1+$index]
    setportdelay $2 $port $3 $4
    if [ $run -eq 1 ]; then
        break
    fi
done
