../../target/install/bin/jsonrpc_performance --config config_correct.json
sleep 8
../../target/install/bin/jsonrpc_performance --config config_correct.json --analysis true
h=`awk -F',' '{if(substr($2,match($2,/[0-9]+/))>0 && (substr($2,match($2,/[0-9]+/))+0)<99999) print $1}'  cita_performance.txt  | grep height | awk -F ":" '{print $2}'`
h=`echo "obase=16;$h" | bc`
../integrate_test/cita_getBlockByNumber.sh $h | jq

echo "create contranct height hex: "$h
create_hash=`../integrate_test/cita_getBlockByNumber.sh $h | jq ".result.body.transactions[0]"|sed 's/\"//g'`
echo "create contranct tx hash: "$create_hash
contranct=`../integrate_test/cita_getTransactionReceipt.sh $create_hash| jq ".result.contractAddress"| sed 's/\"//g'`
echo "contranct: "$contranct
echo "{
    \"ipandport\": [
        \"127.0.0.1:1337\",
        \"127.0.0.1:1338\",
        \"127.0.0.1:1339\",
        \"127.0.0.1:1340\"
    ],
    \"txnum\": 1,
    \"threads\": 5,
    \"code\":\"552410770000000000000000000000000000000000000000000000000000000012345678\",
    \"contract_address\": \"$contranct\",
    \"quota\": 100000,
    \"tx_type\": \"Correct\",
    \"tx_format_err\": false,
    \"is_change_acct\": false
}" > call.json

echo "******************call contract******************"
sleep 30
../../target/install/bin/jsonrpc_performance --config call.json
sleep 20 
../../target/install/bin/jsonrpc_performance --config call.json --analysis true

h=`awk -F',' '{if(substr($2,match($2,/[0-9]+/))>0 && (substr($2,match($2,/[0-9]+/))+0)<99999) print $1}'  cita_performance.txt  | grep height | awk -F ":" '{print $2}'`
h=`echo "obase=16;$h" | bc`
echo "call contract height hex: "$h
../integrate_test/cita_getBlockByNumber.sh $h | jq
hash=`../integrate_test/cita_getBlockByNumber.sh $h | jq ".result.body.transactions[0]"|sed 's/\"//g'`

echo "tx hash: "$hash
if [ "$hash" == "$create_hash" ]; then
   echo "hash == create_hash"
   exit 0
fi
topics=`../integrate_test/cita_getTransactionReceipt.sh $hash| jq ".result.logs[0].topics[0]"| sed 's/\"//g'`
echo "topics: "$topics
curl -s -X POST --data "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getLogs\",\"params\":[{\"topics\":[\"$topics\"],\"fromBlock\": \"0x$h\"}],\"id\":74}" 127.0.0.1:1337 | jq
echo "call contract func tx. h = "$h
