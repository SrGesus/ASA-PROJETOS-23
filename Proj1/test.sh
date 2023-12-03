#! /usr/bin/env bash

echo "\"x\",\"y\",\"n\",\"distinct sides\",\"Elapsed Time (ms)\"" > calls.csv

for ((h=5; h<=$3; h+=20))
do
for ((i=128; i<=$1; i*=2))
do
    for ((j=128; j<=$2; j*=2))
    do
        # for ((h=1; h<=$i; h*=2))
        # do
            cargo run --release --quiet <<< $(./gen $i $j $h) >> calls.csv
        # done
    done
done
done