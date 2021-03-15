i=1
num=$[RANDOM%100+1]
var_array=($num)
for file in $(ls /home/zokrates/.zokrates/examples/ifh*.zok |sort -n -t'h' -k 3);do
    for j in 0..$i;do
    num=$[RANDOM%100+1]
    var_array+=($num)
    done
    echo "var_array: ${var_array[@]}"
    echo "file: $file" >> height_result.txt 
    echo "compile" >> height_result.txt
    time zokrates compile -i $file | tail -n1  >> height_result.txt
echo "setup" >> height_result.txt 
    time zokrates setup | tail -n2  >> height_result.txt
    echo "compute-witness" >> height_result.txt
    time zokrates compute-witness -a ${var_array[@]} | tail -n4  >> height_result.txt
    echo "generate-proof" >> height_result.txt
    time zokrates generate-proof | tail -n1  >> height_result.txt
    i+=i
done











