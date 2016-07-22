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
# this version is definitely slower than the previous one, but bash's arithmetic is limited to 32/64 bits
# which is wayyy to small for fibonacci. I therefore use bc to make computations as it has infinite precision
# for integers. As far as bash is concerned, N, N_1, N_2 and SUM_EVEN are strings.
    while (( SIZE < MAX_NB )); do
        N=$(echo "$N_1 + $N_2" | bc)
        N_2=$N_1
        N_1=$N
        # instead of calling bc and parsing the result to check if the number is even, i check the evenness of the
        # last digit of N.
        if (( ${N: -1} % 2 == 0 )); then
            (( SIZE += 1 ))
            SUM_EVEN=$(echo "$SUM_EVEN + $N" | bc)
        fi
    done
    echo "$SUM_EVEN"
}

printResult() {
    NUMBER=$1
    echo -n "$NUMBER: "
    euler2geo "$NUMBER"
}

echo "Euler geocaching"
printResult 10
printResult 150
printResult 516
printResult 6506
printResult 14
printResult 24
printResult 3232
