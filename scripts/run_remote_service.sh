#!/bin/bash
#set -x

#0:生成Keygen公私钥   1:根据配置文件config.ini中的netwrok生成节点配置，现在cita环境依赖，并放到目录./deb下
#2:如果有公网的话，把生成的公钥和生成的cita的install目录生成到公网
# 3 - 8针对一个节点多服务器部署 ,8 - 12 针对一个节点一个服务器
#3:如果mq单独服务器的化，配置mq服务 4：安装cita环境依赖   5：上传cita到每个服务   6：启动cita 7：停止cita
#8:上传文件到服务器 9:执行远程命令 10:cita 设置 11:cita 启动 12:cita 停止

display_help()
{
    echo
    echo "usage: $0 -c config.ini -m flag -b base_path -r remote_path -u user_name -p pwd -d cmd"
    echo "option:"
    echo "-c config"
    echo
    echo "-m flag"
    echo "
         0:生成Keygen公私钥
         1:根据配置文件config.ini中的netwrok生成节点配置，现在cita环境依赖，并放到目录./deb下
         2:如果有公网的话，把生成的公钥和生成的cita的install目录生成到公网
         ##3 - 8针对一个节点多服务器部署 ,8 - 12 针对一个节点一个服务器##
         3:如果mq单独服务器的化，配置mq服务 4：安装cita环境依赖   5：上传cita到每个服务   6：启动cita 7：停止cita
         8:上传文件到服务器 9:执行远程命令 10:cita 设置 11:cita 启动 12:cita 停止
         "
    echo
    echo "-b base_path(bin基目录)"
    echo
    echo "-r remote_path"
    echo
    echo "-u user_name"
    echo
    echo "-p pwd"
    echo
    echo "-d cmd"
    echo
    echo
    exit 0
}


#读取配置文件
function readINIfile()
{
    Key=$1
    Section=$2
    Configfile=$3
    ReadINI=`awk -F '=' '/\['$Section'\]/{a=1}a==1&&$1~/'$Key'/{print $2;exit}' $Configfile`
    echo "$ReadINI"
}

#生成公私钥
function Keygen()
{
expect <<-EOF
set timeout 5

spawn ssh-keygen -t rsa
expect {
    "Enter file in which to save the key" { send "\r"; exp_continue }
    "Enter passphrase (empty for no passphrase):" { send "\r"; exp_continue }
    "Enter same passphrase again:" { send "\r" }
}
expect EOF ;
EOF
}

#上传公钥，首先要创建 .ssh目录
function remote_mkdir()  
{
dst_host=$1
dst_username=$2
dst_passwd=$3
expect <<-EOF
spawn ssh $dst_username@$dst_host "mkdir -p ~/.ssh > /dev/null"
expect {
    "(yes/no)" { send "yes\r"; exp_continue }
    "password:" { send "$dst_passwd\r" }
}
set timeout 30;
send "exit\r"
expect EOF ;
EOF
}

function scp_pub_to_remote()
{
ip=$1
user_name=$2
password=$3
src_file=~/.ssh/id_rsa.pub
dest_file="~/.ssh/authorized_keys"

expect <<-EOF
set timeout -1
spawn scp  "$src_file" $user_name@$ip:$dest_file
expect {
    "(yes/no)" { send "yes\r"; exp_continue }
    "password:" { send "$password\r" }
}
expect "100%"
expect EOF ;
EOF
}

#apt下载依赖并放到固定目录
function apt_deb()
{
    deb_path="/var/cache/apt/archives/"
    #libgoogle-perftools libunwind(每个进程都需要)
    #libsodium*(chain需要)
    #rabbitmq-server
    sudo apt-get -y -d --reinstall  install rabbitmq-server libsodium* libgoogle-perftools-dev google-perftools libunwind8 libunwind8-dev libunwind-dev libltdl7 libodbc1 libgoogle-perftools4 libtcmalloc-minimal4 erlang
    mkdir -p deb
    find $deb_path -maxdepth 1 -name "rabbitmq-server*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libsodium*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libgoogle-perftools*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libunwind*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libltdl7*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libodbc1*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libgoogle-perftools4*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libtcmalloc-minimal4*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "erlang*" -exec cp {} ./deb \;
}

#下载libgmssl.so(不确定)
function wget_libgmssl()
{
    mkdir -p deb
    cd deb
    wget https://github.com/cryptape/GmSSL/releases/download/v1.0/libgmssl.so.1.0.0.gz
    gzip -d libgmssl.so.1.0.0.gz
    cd ..
}

#上传文件
function scp_file_to_remote()
{
ip=$1
user_name=$2
src_file=$3
dest_file=$4
scp -r $src_file $user_name@$ip:$dest_file
}

#执行服务器命令
function remote_run_cmd()
{
ip=$1
user_name=$2
cmd=$3
ssh $user_name@$ip "$cmd"
}

#启动cita
function cita_start()
{
    node=node$1
    user_name=$2
    auth_host=$3
    network_host=$4
    consensus_host=$5
    jsonrpc_host=$6
    chain_host=$7
    echo "starting ${node}"
    ssh $user_name@$auth_host      "cd install/${node}; mkdir -p logs;nohup ../bin/auth                                       >logs/${node}.auth       2>&1 & echo $! >> .pid"
    ssh $user_name@$network_host   "cd install/${node}; mkdir -p logs;nohup ../bin/network                 -c network.toml    >logs/${node}.network    2>&1 & echo $! >> .pid"
    ssh $user_name@$consensus_host "cd install/${node}; mkdir -p logs;nohup ../bin/consensus_tendermint    -c consensus.json  >logs/${node}.consensus  2>&1 & echo $! >> .pid"
    ssh $user_name@$jsonrpc_host   "cd install/${node}; mkdir -p logs;nohup ../bin/jsonrpc                 -c jsonrpc.json    >logs/${node}.jsonrpc    2>&1 & echo $! >> .pid"
    ssh $user_name@$chain_host     "cd install/${node}; mkdir -p logs;nohup ../bin/chain  -g genesis.json  -c chain.json      >logs/${node}.chain      2>&1 & echo $! >> .pid"
}

#停止cita
function cita_stop()
{
    node=node$1
    user_name=$2
    auth_host=$3
    network_host=$4
    consensus_host=$5
    jsonrpc_host=$6
    chain_host=$7
    echo "stop ${node}"
    ssh $user_name@$auth_host      "killall auth;cd install/${node}; rm -rf data/*"
    ssh $user_name@$network_host   "killall network; cd install/${node}; rm -rf data/*"
    ssh $user_name@$consensus_host "killall consensus_tendermint; cd install/${node}; rm -rf data/*"
    ssh $user_name@$jsonrpc_host   "killall jsonrpc; cd install/${node}; rm -rf data/*"
    ssh $user_name@$chain_host     "killall chain; cd install/${node}; rm -rf data/*"
}


CUR_PATH=$(cd `dirname $0`; pwd)
# parse options usage: $0 -c config.ini -m flag -b base_path -r remote_path -u user_name -p pwd -t
while getopts 'c:m:b:r:u:p:d:' OPT; do
    case $OPT in
        c)
            config="$OPTARG";;
        m)
            method="$OPTARG";;
        b)
            base_path="$OPTARG";;
        r)
            remote_path="$OPTARG";;
        u)
            user_name="$OPTARG";;
        p)
            pwd="$OPTARG";;
        d)
            cmd="$OPTARG";;
        ?)
            display_help
    esac
done

#set default value
if [ ! -n "$config" ]; then
    config="config.ini"
fi

if [ ! -n "$method" ]; then
    echo "method must be set up"
    exit 0
fi

if [ ! -n "$base_path" ]; then
    echo "base_path must be set up"
    exit 0
fi

if [ ! -n "$remote_path" ]; then
    remote_path="~/"
fi

if [ ! -n "$user_name" ]; then
    echo "user_name must be set up"
    exit 0
fi

if [ ! -n "$pwd" ]; then
    if [ $method -eq 2 ]; then
        echo "pwd must be set up"
        exit 0
    fi
    pwd=""
fi

if [ ! -n "$cmd" ]; then
    cmd=""
fi

node_num=4

if [ $method -eq 0 ]; then
    #生成公私钥
    rm -rf ~/.ssh
    Keygen > /dev/null

elif [ $method -eq 1 ]; then
    #生成节点的配置
    admintool_path=$base_path
    echo "admintool_path = "$admintool_path
    #读取配置文件
    network_host=`readINIfile "netwrok" "host" "$config"`

    admintool="./bin/admintool.sh -l "$network_host
    echo "执行: "$admintool
    cd $admintool_path
    $($admintool > /dev/null 2>&1)
    cd $CUR_PATH

    #下载deb
    apt_deb

elif [ $method -eq 2 ]; then
    remote_host=`readINIfile "remote_host" "host" "$config"`
    echo "$pwd"
    #上传公钥到公网
    scp_pub_to_remote $remote_host $user_name "$pwd"

    #上传cita执行文件到外网服务器
    src_cita="$base_path"
    scp_file_to_remote $remote_host $user_name "$src_cita" "$remote_path"

elif [ $method -eq 3 ]; then
    #mq服务器配置
    echo "mq服务器配置"
    amqp_host=`readINIfile "amqp" "host" "$config"`
    OLD_IFS="$IFS"  
    IFS=":"  
    amqp_host_arr=($amqp_host)  
    IFS="$OLD_IFS"  
    length=${#amqp_host_arr[@]}
    i=0
    install_cmd="dpkg -i rabbitmq-server*"
    rabbitmq_path="deb/rabbitmq-server*"
    while :
    do
        #mq服务器上传公钥
        remote_mkdir ${amqp_host_arr[$i]} $user_name $pwd
        scp_pub_to_remote ${amqp_host_arr[$i]} $user_name $pwd > /dev/null
        
        #上传deb到mq服务
        scp_file_to_remote "$remote_host" "$user_name" "$rabbitmq_path" "$remote_path"

        #dpkg -i 安装    
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$install_cmd"

        #生成rabbitmq用户
        add_vhost_cmd="rabbitmqctl add_vhost node$i"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$add_vhost_cmd"

        add_user_cmd="rabbitmqctl add_user cita cita"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$add_user_cmd"
        
        set_user_tags_cmd="rabbitmqctl set_user_tags cita  administrator"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$set_user_tags_cmd"
        
        set_permissions_cmd='rabbitmqctl set_permissions -p node$i cita ".*" ".*" ".*"'
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$set_permissions_cmd"
        
        service_cmd="service rabbitmq-server restart"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$service_cmd"

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done

elif [ $method -eq 4 ]; then

    #安装cita环境依赖
    amqp_host=`readINIfile "amqp" "host" "$config"`
    OLD_IFS="$IFS"  
    IFS=":"
    amqp_host_arr=($amqp_host)  
    IFS="$OLD_IFS"
    length=${#amqp_host_arr[@]}
    deb_install="cd deb;dpkg -i libsodium* libgoogle-perftools* libunwind*"
    for((i=0; i<$node_num; i++))
    do
        Section="node""$i"
        host_name=$Section"_mq"
        #修改每个节点的.env
        sed -ig "s/localhost/$host_name/g" $base_path/node$i/.env
        sed -ig "s/guest/cita/g" $base_path/node$i/.env

        cmd="echo ${amqp_host_arr[$i]}    $host_name >> /etc/hosts"
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        remote_mkdir $jsonrpc_host $user_name $pwd
        scp_pub_to_remote $jsonrpc_host $user_name > /dev/null
        remote_run_cmd "$jsonrpc_host" "$user_name" "$cmd"
        #上传deb到服务并安装
        scp_file_to_remote "$jsonrpc_host" "$user_name" "deb" "~/"
        remote_run_cmd "$jsonrpc_host" "$user_name" "$deb_install"

        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        remote_mkdir $chain_host $user_name $pwd
        scp_pub_to_remote $chain_host $user_name $pwd > /dev/null
        remote_run_cmd "$chain_host" "$user_name" "$cmd"
        #上传deb到服务并安装
        scp_file_to_remote "$chain_host" "$user_name" "deb" "~/"
        remote_run_cmd "$chain_host" "$user_name" "$deb_install"

        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        remote_mkdir $consensus_host $user_name $pwd
        scp_pub_to_remote $consensus_host $user_name $pwd > /dev/null
        remote_run_cmd "$consensus_host" "$user_name" "$cmd"
        #上传deb到服务并安装
        scp_file_to_remote "$consensus_host" "$user_name" "deb" "~/"
        remote_run_cmd "$consensus_host" "$user_name" "$deb_install"

        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`
        remote_mkdir $auth_host $user_name $pwd
        scp_pub_to_remote $auth_host $user_name $pwd > /dev/null
        remote_run_cmd "$auth_host" "$user_name" "$cmd"
        #上传deb到服务并安装
        scp_file_to_remote "$auth_host" "$user_name" "deb" "~/"
        remote_run_cmd "$auth_host" "$user_name" "$deb_install"
    done

elif [ $method -eq 5 ]; then
#上传cita到每个服务
    src_cita="$base_path"
    for((i=0; i<$node_num; i++))
    do
        Section="node""$i"
        host_name=$Section"_mq"
        echo "=============$Section================"
        #修改每个节点的.env
        sed -ig "s/localhost/$host_name/g" $src_cita/node$i/.env
        sed -ig "s/guest/cita/g" $src_cita/node$i/.env
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        ssh $user_name@$jsonrpc_host "rm -rf install"
        scp_file_to_remote $jsonrpc_host $user_name $src_cita "$remote_path"
        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        ssh $user_name@$chain_host "rm -rf install"
        scp_file_to_remote $chain_host $user_name $src_cita "$remote_path"
        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        ssh $user_name@$consensus_host "rm -rf install"
        scp_file_to_remote $consensus_host $user_name $src_cita "$remote_path"
        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`
        ssh $user_name@$auth_host "rm -rf install"
        scp_file_to_remote $auth_host $user_name $src_cita "$remote_path"
    done



elif [ $method -eq 6 ]; then
#启动cia
    for((i=0; i<$node_num; i++))
    do
        Section="node""$i"
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`

        cita_start $i $user_name $auth_host $consensus_host $consensus_host $jsonrpc_host $chain_host
    done
elif [ $method -eq 7 ]; then
#停止cita
    for((i=0; i<0; i++))
    do
        Section="node""$i"
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`
        cita_stop $i $user_name $auth_host $consensus_host $consensus_host $jsonrpc_host $chain_host
    done

elif [ $method -eq 8 ]; then
#上传文件到服务
    cita_host=`readINIfile "cita" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    cita_host_arr=($cita_host)
    IFS="$OLD_IFS"
    length=${#cita_host_arr[@]}
    i=0
    while :
    do
        #mq服务器上传公钥
        remote_mkdir ${cita_host_arr[$i]} $user_name $pwd
        scp_pub_to_remote ${cita_host_arr[$i]} $user_name $pwd > /dev/null
        scp_file_to_remote "${cita_host_arr[$i]}" "$user_name" "$base_path" "$remote_path"

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 9 ]; then
#执行命令
    cita_host=`readINIfile "cita" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    cita_host_arr=($cita_host)
    IFS="$OLD_IFS"
    length=${#cita_host_arr[@]}
    i=0
    while :
    do
        remote_run_cmd "${cita_host_arr[$i]}" "$user_name" "$cmd"
        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 10 ]; then
#用cita命令setup
    echo "cita setup"
    cita_host=`readINIfile "cita" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    cita_host_arr=($cita_host)
    IFS="$OLD_IFS"
    length=${#cita_host_arr[@]}
    i=0
    while :
    do
        remote_run_cmd ${cita_host_arr[$i]} $user_name "cd $base_path;rm -rf node$i/data;./bin/cita setup node$i" &
        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 11 ]; then
#用cita命令启动cita
    echo "启动cita"
    cita_host=`readINIfile "cita" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    cita_host_arr=($cita_host)
    IFS="$OLD_IFS"
    length=${#cita_host_arr[@]}
    i=0
    while :
    do
        #mq服务器上传公钥
        remote_run_cmd ${cita_host_arr[$i]} $user_name "cd $base_path;./bin/cita start node$i" &

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 12 ]; then
#用cita命令停止cita
    echo "停止cita"
    cita_host=`readINIfile "cita" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    cita_host_arr=($cita_host)
    IFS="$OLD_IFS"
    length=${#cita_host_arr[@]}
    i=0
    while :
    do
        #mq服务器上传公钥
        remote_run_cmd ${cita_host_arr[$i]} $user_name "cd install;./bin/cita stop node$i;rm -rf node$i/data" &

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
fi
