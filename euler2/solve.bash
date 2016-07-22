#! /bin/bash

# https://projecteuler.net/problem=2

euler2() {
    MAX=$1
    SUM_EVEN=2
    N_1=2
    N_2=1
    N=0
    while (( N_1 + N_2 < MAX )); do
        (( N = N_1 + N_2, N_2 = N_1, N_1 = N ))
        (( N % 2 == 0 )) && (( SUM_EVEN += N ))
    done
    echo $SUM_EVEN
}

echo "Euler project"
euler2 4000000
echo ""

euler2geo() {
    MAX_NB=$1
    SIZE=1
    SUM_EVEN=2
    N_1=2
    N_2=1
    N=0
    while (( SIZE < MAX_NB )); do
        (( N = N_1 + N_2, N_2 = N_1, N_1 = N ))
        (( N % 2 == 0 )) && (( SIZE += 1, SUM_EVEN += N ))
    done
    echo $SUM_EVEN
}

echo "Euler geocaching"
euler2geo 150
euler2geo 516
euler2geo 6506
euler2geo 14
euler2geo 24
euler2geo 3232
