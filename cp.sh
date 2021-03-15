for file in $(ls /home/app/i-f-tree/*.zok |sort -n -t'h' -k 2);do
echo "file: $file"
docker cp $file e8e35d7acef5:/home/zokrates/.zokrates/examples
done
