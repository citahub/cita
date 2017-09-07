#!/bin/bash
set +e
CUR_PATH=$(cd `dirname $0`; pwd)
EXEC=${CUR_PATH}/release
count=4

function cita_help(){
    echo
    echo "      usage:      cita command nodeid"
    echo
    echo "      command description:"
    echo
    echo "      setup            setup cita run environment"
    echo
    echo "      start            start cita"
    echo
    echo "      stop             stop cita"
    echo
    echo "      restart          restart cita"
    echo
    echo "      status           display cita run status"
    echo
    echo "      jsonrpc          display cita's jsonrpc log information"
    echo
    echo "      chain            display cita's chain log information"
    echo
    echo "      consensus        display cita's consensus log information"
    echo
    echo "      network          display cita's network log information"
    echo
    echo "      clean            clean cita log file"
    echo
    echo "      version          display cita version"
    echo
    echo "      help             display help information"
    echo
    echo
}

operate_all(){
    operate=$1
    debug=$2
    while true
    do
        count=`expr $count - 1`
        echo "${EXEC}/node${count} ${operate} ${debug}"
        cd ${EXEC}/node${count}
        ./cita $operate ${count} $debug
        if test $count -eq 0
        then
            break;
        fi;
    done
}

if [ $# -gt 3 ];
then
    cita_help
else
    case $1 in
        setup) ./admintool.sh ; operate_all $1 ;;
        start) operate_all $1 $2;;
        stop) operate_all  $1 ;;
        clean) operate_all $1 ;;
        restart) operate_all $1;;
        status) operate_all $1;;
        jsonrpc) operate_all $1;;
        chain) operate_all $1;;
        consensus) operate_all $1;;
        network) operate_all $1;;
        version) echo "0.9";;
        help) cita_help ;;
        *) cita_help;;
    esac
fi



