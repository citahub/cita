#!/bin/bash
display_help()
{
	echo 
	echo "usage: $0 -a admin_id -p admin_pubkey -l ip_list -n consensus_name -m crypto_method -d block_duration -t -b block_tx_limit -f tx_filter_size"
	echo "option:"
	echo "-a admin_id    admin identifier"
	echo "    default value is 'admin'"
	echo
	echo "-p admin_pubkey    set admin pubkey"
	echo "    default value is random generated"
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
	exit 0
}

# parse options
while getopts 'a:p:l:n:m:d:tb:f:' OPT; do
    case $OPT in
        a)
            ADMIN_ID="$OPTARG";;
        p)
            ADMIN_PUBKEY="$OPTARG";;
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

DATA_PATH=`pwd`/release

if [ ! -f "$DATA_PATH" ]; then
    mkdir -p $DATA_PATH
fi

echo "Step 1: ********************************************************"
echo "Start Genesis Block's Configuration creating!"
python create_keys_addr.py $DATA_PATH 
python create_genesis.py $ADMIN_ID $CRYPTO_METHOD $DATA_PATH $ADMIN_PUBKEY

if [ -f "$DATA_PATH/authorities" ]; then
    rm $DATA_PATH/authorities
fi

echo "End for Genesis Block Configuration creating!"
echo "Step 2: ********************************************************"
for ((ID=0;ID<$SIZE;ID++))
do
	mkdir -p $DATA_PATH/node$ID
	echo "Start generating private Key for Node" $ID "!"
	python create_keys_addr.py $DATA_PATH $ID 
	echo "[PrivateKey Path] : " $DATA_PATH/node$ID
	echo "End generating private Key for Node" $ID "!"
	cp $DATA_PATH/genesis.json $DATA_PATH/node$ID/
	echo "Start creating Network Node" $ID "Configuration!"
	python create_network_config.py $DATA_PATH $ID $SIZE $IP_LIST
	echo "End creating Network Node" $ID "Configuration!"
	echo "########################################################"
done
echo "Step 3: ********************************************************"
for ((ID=0;ID<$SIZE;ID++))
do
	echo "Start creating Node " $ID " Configuration!"
	python create_node_config.py $DATA_PATH $CONSENSUS_NAME $ID $DURATION $IS_TEST $BLOCK_TX_LIMIT $TX_FILTER_SIZE
	echo "End creating Node " $ID "Configuration!"
done

if [ -f "$DATA_PATH/authorities" ]; then
    rm $DATA_PATH/authorities
fi
if [ -f "$DATA_PATH/genesis.json" ]; then
    rm $DATA_PATH/genesis.json
fi
echo "Step 4: ********************************************************"
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
echo "********************************************************"
echo "WARN: remember then delete all privkey files!!!"