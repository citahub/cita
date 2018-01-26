for((i=0;i<$1;i++))
do
../../target/install/bin/jsonrpc_performance --config call.json
sleep 20 
../../target/install/bin/jsonrpc_performance --config call.json --analysis true

h=`awk -F',' '{if(substr($2,match($2,/[0-9]+/))>0 && (substr($2,match($2,/[0-9]+/))+0)<99999) print $1}'  cita_performance.txt  | grep height | awk -F ":" '{print $2}'`
echo $h
h=`echo "obase=16;$h" | bc`
echo $h
size=`../integrate_test/cita_getBlockByNumber.sh $h | jq ".result.body.transactions" | wc -l`
num=$[$2+2]
if [ $size -gt $num ]; then
../integrate_test/cita_getBlockByNumber.sh $h | jq
exit 0
fi
hash=`../integrate_test/cita_getBlockByNumber.sh $h | jq ".result.body.transactions[0]"|sed 's/\"//g'`

echo "tx hash: "$hash
if [ "$hash" == "$create_hash" ]; then
   echo "hash == create_hash"
   continue 
fi
topics=`../integrate_test/cita_getTransactionReceipt.sh $hash| jq ".result.logs[0].topics[0]"| sed 's/\"//g'`
echo "topics: "$topics
curl -s -X POST --data "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getLogs\",\"params\":[{\"topics\":[\"$topics\"],\"fromBlock\": \"0x$h\"}],\"id\":74}" 127.0.0.1:1337 | jq
echo "call contract func tx. h = "$h

done
