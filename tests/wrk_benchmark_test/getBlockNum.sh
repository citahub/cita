num=0
for((h=$1; h<=$2;h++))
do

tmp=$(../integrate_test/cita_getBlockTransactionNumByHeight.sh $h $3)
#echo $tmp
num=$[$num+$tmp]

done

echo $num
