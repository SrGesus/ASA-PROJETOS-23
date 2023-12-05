#! /usr/bin/env bash

# echo "\"x\",\"y\",\"n\",\"distinct sides\",\"Elapsed Time (ms)\"" > calls.csv
# N=3
# for ((h=$3; h>20; h-=20))
# do
h=$3
for ((i=128; i<=$1; i+=128))
do
    for ((j=128; j<=$2; j+=128))
    do
        # ((k=k%N)); ((k++==0)) && wait;
        # for ((h=1; h<=$i; h*=2))
        # do
            cargo run --release --quiet <<< $(./gen $i $j $h) >> calls.csv
        # done
    done
done
# done
