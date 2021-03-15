i=1
for file in $(ls /home/zokrates/.zokrates/examples/ifw*.zok |sort -n -t'w' -k 2);do
    for j in 0..$i;do
    num=$[RANDOM%100+1]
    var_array+=($num)
    done
    echo "var_array: ${var_array[@]}"
    echo "file: $file" >> width_result.txt
    echo "compile" >> width_result.txt
    time zokrates compile -i $file | tail -n1  >> width_result.txt
echo "setup" >> width_result.txt
    time zokrates setup | tail -n2  >> width_result.txt
    echo "compute-witness" >> width_result.txt
    time zokrates compute-witness -a ${var_array[@]} | tail -n4  >> width_result.txt
    echo "generate-proof" >> width_result.txt
    time zokrates generate-proof | tail -n1  >> width_result.txt
    i+=i
done

