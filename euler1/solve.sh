#! /bin/sh

# https://projecteuler.net/problem=1

function euler1() {
    MAX=$1
    SUM=0
    for i in $(seq 1 "$MAX"); do
        if ! (( i % 3 )) || ! (( i % 5 )); then
            SUM=$(( SUM + i ))
        fi
    done
    echo "$SUM"
}

echo $(( $(euler1 4720) + $(euler1 240) + $(euler1 77) ))

echo $(( $(euler1 175) + $(euler1 10) ))
