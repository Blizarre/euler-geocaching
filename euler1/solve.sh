#! /bin/sh

# https://projecteuler.net/problem=1

euler1() {
    MAX=$1
    SUM=0
    i=0
    while [ "$i" -lt "$MAX" ]; do
        if [ $(( i % 3 )) = 0 ] || [ $(( i % 5 )) = 0 ]; then
            SUM=$(( SUM + i ))
        fi
        i=$(( i + 1 ))
    done
    echo "$SUM"
}

echo "Euler project solution: "
euler1 1000

echo "Geocaching solution: "

echo $(( $(euler1 4720) + $(euler1 240) + $(euler1 77) ))

echo $(( $(euler1 175) + $(euler1 10) ))
