#! /usr/bin/env bash

echo "\"Res\",\"V\",\"E\",\"Elapsed Time (µs)\"" > calls.csv

for ((i=16; i<=$1; i+=128))
do
    for ((j=16; j<=$2; j+=128))
    do
        h=$(( $j / 2 ))  # Set the value of $h to $j divided by 2
        cargo run --release --quiet <<< $(./gen $j $i $h) >> calls.csv
    done
done
