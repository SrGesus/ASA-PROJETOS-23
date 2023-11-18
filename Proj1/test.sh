#! /usr/bin/env bash

echo "\"x\",\"y\",\"calls\"" > calls.csv

for ((i=1; i<=$1; i*=2))
do
    for ((j=1; j<=$2; j*=2))
    do
        printf "$i $j \n0\n" | cargo run --release --quiet >> calls.csv
    done
done

