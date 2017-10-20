#!/bin/bash
set -e

display_help()
{
    echo
    echo "usage: $0 -a admin_id -l ip_list -n consensus_name -m crypto_method -d block_duration -t"
    echo "option:"
    echo "-a admin_id    admin identifier"
    echo "    default value is 'admin'"
    echo
    echo "-l ip_list     list all the node's IP and port"
    echo "    default value is '127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003'"
    echo
    echo "-n consensus_name  name of consensus algorithm"
    echo "    default value is 'tendermint', other is 'raft' and 'poa'"
    echo
    echo "-m crypto_method    name of crypto algorithm"
    echo "    default value is 'SECP'"
    echo
    echo "-d block_duration    block generating duration(millisecond)"
    echo "    default value is '3000'"
    echo
    echo "-t            consensus test flag, only valid for tendermint"
    echo
    echo "-h enable jsonrpc http"
    echo "   default enable 'true'"
    echo
    echo "-w enable jsonrpc websocket "
    echo "   default enable 'false'"
    echo
    echo "-P define jsonrpc HTTP port or websocket port"
    echo "   default port is '1337' or '4337'"
    echo
    exit 0
}
CONFIG_DIR=${PWD}
BINARY_DIR=$(readlink -f $(dirname $(readlink -f $0))/../..)
export PATH=${PATH}:${BINARY_DIR}/bin

# parse options
while getopts 'a:l:n:m:d:t:h:w:P:' OPT; do
    case $OPT in
        a)
            ADMIN_ID="$OPTARG";;
        l)
            IP_LIST="$OPTARG";;
        n)
            CONSENSUS_NAME="$OPTARG";;
        m)
            CRYPTO_METHOD="$OPTARG";;
        d)
            DURATION="$OPTARG";;
        t)
            IS_TEST=true;;
        h)
            HTTP="$OPTARG";;
        w)
            WS="$OPTARG";;
        P)
            PORT="$OPTARG";;
        ?)
            display_help
    esac
done

#set default value
if [ ! -n "$ADMIN_ID" ]; then
    ADMIN_ID="admin"
fi

if [ ! -n "$IP_LIST" ]; then
    DEV_MOD=1
    IP_LIST="127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
fi

#calc size of nodes
TMP=${IP_LIST//[^\:]}
SIZE=${#TMP}

if [ ! -n "$CONSENSUS_NAME" ]; then
    CONSENSUS_NAME="tendermint"
fi

if [ ! -n "$CRYPTO_METHOD" ]; then
    CRYPTO_METHOD="SECP"
fi

if [ ! -n "$DURATION" ]; then
    DURATION=3000
fi

if [ ! -n "$IS_TEST" ]; then
    IS_TEST=false
fi

if [ -f "${CONFIG_DIR}/authorities" ]; then
    rm ${CONFIG_DIR}/authorities
fi

if [ -f "genesis.json" ]; then
    rm genesis.json
fi

if [ ! -e "${BINARY_DIR}/scripts/admintool/init_data.json" ]; then
    cp ${BINARY_DIR}/scripts/admintool/init_data_example.json ${CONFIG_DIR}/init_data.json
else
    cp ${BINARY_DIR}/scripts/admintool/init_data.json ${CONFIG_DIR}/init_data.json
fi

if [ ! -e "${CONFIG_DIR}/chain.json" ]; then
    cp ${BINARY_DIR}/scripts/admintool/chain_check_example.json ${CONFIG_DIR}/chain.json
fi

echo "Step 1: ********************************************************"
for ((ID=0;ID<$SIZE;ID++))
do
    mkdir -p ${CONFIG_DIR}/node${ID}
    echo "Start generating private Key for Node" ${ID} "!"
    python ${BINARY_DIR}/scripts/admintool/create_keys_addr.py ${CONFIG_DIR} ${ID} create_key_addr
    echo "[PrivateKey Path] : " ${CONFIG_DIR}/node${ID}
    echo "End generating private Key for Node" ${ID} "!"
    echo "Start creating Network Node" ${ID} "Configuration!"
    python ${BINARY_DIR}/scripts/admintool/create_network_config.py ${CONFIG_DIR} ${ID} $SIZE $IP_LIST
    echo "End creating Network Node" ${ID} "Configuration!"
    echo "########################################################"
done
echo "Step 2: ********************************************************"

python ${BINARY_DIR}/scripts/admintool/create_genesis.py --authorities "${CONFIG_DIR}/authorities" --init_data "${CONFIG_DIR}/init_data.json"
for ((ID=0;ID<$SIZE;ID++))
do
    echo "Start creating Node " ${ID} " Configuration!"
    python ${BINARY_DIR}/scripts/admintool/create_node_config.py ${CONFIG_DIR} $CONSENSUS_NAME ${ID} $DURATION $IS_TEST
    echo "End creating Node " ${ID} "Configuration!"
    cp genesis.json ${CONFIG_DIR}/node${ID}/genesis.json
    cp chain.json ${CONFIG_DIR}/node${ID}/chain.json
done

echo "Step 3: ********************************************************"
sed -i "s/tendermint/$CONSENSUS_NAME/g" ${BINARY_DIR}/bin/cita
for ((ID=0;ID<$SIZE;ID++))
do
    rm -f ${CONFIG_DIR}/node${ID}/.env
    echo "KAFKA_URL=localhost:9092"                         >> ${CONFIG_DIR}/node${ID}/.env
    echo "AMQP_URL=amqp://guest:guest@localhost/node${ID}"  >> ${CONFIG_DIR}/node${ID}/.env
    echo "DATA_PATH=./data"                                 >> ${CONFIG_DIR}/node${ID}/.env
done


echo "JsonRpc Configuration creating!"
echo "Step 4: ********************************************************"

HTTP_PORT=1337
HTTP_ENABLE="true"
WS_PORT=4337
WS_ENABLE="false"

if [ "$HTTP" == "true" ]; then
    HTTP_ENABLE="true"
    if [ ! -n "$PORT" ]; then
        HTTP_PORT=1337
    else
        HTTP_PORT=$PORT
    fi

    WS_PORT=4337
    WS_ENABLE="false"
fi

if [ "$WS" == "true" ]; then
    WS_ENABLE="true"
    if [ ! -n "$PORT" ]; then
        WS_PORT=4337
    else
        WS_PORT=$PORT
    fi
    HTTP_PORT=1337
    HTTP_ENABLE="false"

fi

for ((ID=0;ID<$SIZE;ID++))
do
    mkdir -p ${CONFIG_DIR}/node${ID}
    if [ -n "$DEV_MOD" ]; then
        ((H_PORT=$HTTP_PORT+${ID}))
        ((W_PORT=$WS_PORT+${ID}))
    else
        H_PORT=$HTTP_PORT
        W_PORT=$WS_PORT
    fi
    echo "Start generating JsonRpc Configuration Node" ${ID} "!"
    python ${BINARY_DIR}/scripts/admintool/create_jsonrpc_config.py $HTTP_ENABLE $H_PORT $WS_ENABLE $W_PORT ${CONFIG_DIR}
    echo "[JsonRpc Configuration Path] : " ${CONFIG_DIR}/node${ID}
    echo "JsonRpc Configuration for Node" ${ID} "!"
    cp ${CONFIG_DIR}/jsonrpc.json ${CONFIG_DIR}/node${ID}/

    echo "########################################################"
done

# clean temp files
rm -f ${CONFIG_DIR}/*.json ${CONFIG_DIR}/authorities

echo "********************************************************"
echo "WARN: remember then delete all privkey files!!!"
