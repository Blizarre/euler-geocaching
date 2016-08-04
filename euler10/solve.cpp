#include <vector>
#include <string>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <cassert>
#include <cmath>

using namespace std;

/**
 * Check if number can be divided by any of the numbers in divisors
 **/
bool is_prime_iterative(uint64_t number, vector<uint64_t> divisors) {
    uint64_t max = (uint64_t)ceil(sqrt(number));
    auto it = divisors.cbegin();
    while(*it <= max) {
        if(number % *it == 0) {
            return false;
        }
        it ++;
    }
    divisors.push_back(number);
    return true;
}

/**
 * Check if number is prime
 **/
bool is_prime(uint64_t number) {
    uint64_t max = ceil(sqrt(number));
    for(uint64_t i = 2; i <= max; i += 2) {
        if(number % i  == 0) {
            return false;
        }
    }
    return true;
}

void test() {
    vector<uint64_t> primes;
    primes.push_back(2);
    assert(is_prime(3));
    assert(is_prime_iterative(3, primes));
    assert(!is_prime(4));
    assert(!is_prime_iterative(4, primes));
    assert(is_prime(5));
    assert(is_prime_iterative(5, primes));
    assert(!is_prime(6));
    assert(!is_prime_iterative(6, primes));
    assert(is_prime(7));
    assert(is_prime_iterative(7, primes));
}

int main(int argc, char* argv[]) {
    test();

    if(argc != 2) {
        cout << "Usage: " << argv[0] << " NumberOfPrimes" << endl;
        exit(0);
    }
    uint64_t nbElements = stoll(argv[1]);
    cout << "Computing the sum of the " << nbElements << "first primes" << endl;

    vector<uint64_t> primes;
    primes.reserve(nbElements);
    primes.push_back(2);
    uint64_t i = 3;
    int nbPrimes = 1;
    uint64_t sumPrimes = 2;
    for (uint64_t nb_primes = 1; nb_primes < nbElements; nb_primes++) {
        //while( ! is_prime_iterative(i, primes) ) {
        while( ! is_prime(i) ) {
            i += 2;
        }
        nbPrimes ++;
        sumPrimes += (i % 10000000);
        i += 2;
        if(nbPrimes % 100000 == 0) cout << nbPrimes << endl;
    }
    cout << nbPrimes << " " << sumPrimes << endl;
}
