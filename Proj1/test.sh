#! /usr/bin/env bash

echo "\"x\",\"y\",\"calls\"" > calls.csv

for ((i=1; i<=$1; i++))
do
    for ((j=1; j<=$2; j++))
    do
        printf "$i $j \n0\n" | target/release/proj1 >> calls.csv
    done
done

