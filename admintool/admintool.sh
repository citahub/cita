#!/bin/bash
set -e

display_help()
{
    echo 
    echo "usage: $0 -a admin_id -l ip_list -n consensus_name -m crypto_method -d block_duration -t -b block_tx_limit -f tx_filter_size"
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
    echo "-b block_tx_limit    the limit of tx count in one block"
    echo "    default value is '300'"
    echo
    echo "-f tx_filter_size    the range of hisory tx to check duplication"
    echo "    default value is '100000'"
    echo
    echo "-c tx_pool_size    flow control for tx pool"
    echo "    default value is '0'"
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

# parse options
while getopts 'a:l:n:m:d:tb:f:c:h:w:P:' OPT; do
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
        b)
            BLOCK_TX_LIMIT="$OPTARG";;
        f)
            TX_FILTER_SIZE="$OPTARG";;
        c)
            TX_POOL_SIZE="$OPTARG";;
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

if [ ! -n "$BLOCK_TX_LIMIT" ]; then
    BLOCK_TX_LIMIT=300
fi

if [ ! -n "$TX_FILTER_SIZE" ]; then
    TX_FILTER_SIZE=100000
fi

if [ ! -n "$TX_POOL_SIZE" ]; then
    TX_POOL_SIZE=0
fi

DATA_PATH=`pwd`/release
INIT_DATA_PATH=`pwd`
CREATE_KEY_ADDR_PATH=$DATA_PATH/bin/create_key_addr

if [ ! -f "$DATA_PATH" ]; then
    mkdir -p $DATA_PATH
fi

if [ -f "$DATA_PATH/authorities" ]; then
    rm $DATA_PATH/authorities
fi

if [ -f "genesis.json" ]; then
    rm genesis.json
fi

if [ ! -e "$INIT_DATA_PATH/init_data.json" ]; then
    cp $INIT_DATA_PATH/init_data_example.json $DATA_PATH/init_data.json
else
    cp $INIT_DATA_PATH/init_data.json $DATA_PATH/init_data.json
fi

if [ ! -e "$INIT_DATA_PATH/chain.json" ]; then
    cp $INIT_DATA_PATH/chain_check_example.json $INIT_DATA_PATH/chain.json
fi

echo "Step 1: ********************************************************"
for ((ID=0;ID<$SIZE;ID++))
do
    mkdir -p $DATA_PATH/node$ID
    echo "Start generating private Key for Node" $ID "!"
    python create_keys_addr.py $DATA_PATH $ID $CREATE_KEY_ADDR_PATH
    echo "[PrivateKey Path] : " $DATA_PATH/node$ID
    echo "End generating private Key for Node" $ID "!"
    echo "Start creating Network Node" $ID "Configuration!"
    python create_network_config.py $DATA_PATH $ID $SIZE $IP_LIST
    echo "End creating Network Node" $ID "Configuration!"
    echo "########################################################"
done
echo "Step 2: ********************************************************"

python create_genesis.py --authorities "$DATA_PATH/authorities" --init_data "$DATA_PATH/init_data.json"
for ((ID=0;ID<$SIZE;ID++))
do
    echo "Start creating Node " $ID " Configuration!"
    python create_node_config.py $DATA_PATH $CONSENSUS_NAME $ID $DURATION $IS_TEST $BLOCK_TX_LIMIT $TX_FILTER_SIZE $TX_POOL_SIZE
    echo "End creating Node " $ID "Configuration!"
    cp genesis.json $DATA_PATH/node$ID/genesis.json
    cp chain.json $DATA_PATH/node$ID/chain.json
done

echo "Step 3: ********************************************************"
for ((ID=0;ID<$SIZE;ID++))
do
    echo "Start creating Node " $ID " env!"
    cp cita $DATA_PATH/node$ID/
    cp $DATA_PATH/.env $DATA_PATH/node$ID/
    case "$OSTYPE" in
        darwin*)
            sed -ig "s/dev/node$ID/g" $DATA_PATH/node$ID/.env
            sed -ig "s/tendermint/$CONSENSUS_NAME/g" $DATA_PATH/node$ID/cita
            ;;
        *)
            sed -i "s/dev/node$ID/g" $DATA_PATH/node$ID/.env
            sed -i "s/tendermint/$CONSENSUS_NAME/g" $DATA_PATH/node$ID/cita
            ;;
    esac
    echo "Start copy binary for Node " $ID "!"
    if [ -n "$DEV_MOD" ]; then
        rm -f $DATA_PATH/node$ID/bin
        ln -s $DATA_PATH/bin $DATA_PATH/node$ID/bin
    else
        cp -rf $DATA_PATH/bin $DATA_PATH/node$ID/
    fi
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
    mkdir -p $DATA_PATH/node$ID
    if [ -n "$DEV_MOD" ]; then
        ((H_PORT=$HTTP_PORT+$ID))
        ((W_PORT=$WS_PORT+$ID))
    else
        H_PORT=$HTTP_PORT
        W_PORT=$WS_PORT
    fi
    echo "Start generating JsonRpc Configuration Node" $ID "!"
    python create_jsonrpc_config.py $HTTP_ENABLE $H_PORT $WS_ENABLE $W_PORT $DATA_PATH
    echo "[JsonRpc Configuration Path] : " $DATA_PATH/node$ID
    echo "JsonRpc Configuration for Node" $ID "!"
    cp $DATA_PATH/jsonrpc.json $DATA_PATH/node$ID/

    echo "########################################################"
done

echo "********************************************************"
echo "WARN: remember then delete all privkey files!!!"
