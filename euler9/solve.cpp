#include <vector>
#include <string>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <cassert>
#include <cmath>

/**
 * Check if number can be divided by any of the numbers in divisors
 **/
bool is_prime_iterative(uint64_t number, const std::vector<uint64_t> divisors) {
    for(uint64_t div: divisors) {
        if(number % div == 0) {
            return false;
        }
    }
    return true;
}

/**
 * Check if number can be divided by any of the numbers in divisors
 **/
bool is_prime(uint64_t number, const std::vector<uint64_t> divisors) {
    uint64_t i;
    uint64_t max = std::sqrt(number) + 1;
    for(i = 2; i <= max; i += 2) {
        if(number % i  == 0) {
            return false;
        }
    }
//    std::cout << number << std::endl;
    return true;
}

int main(int argc, char* argv[]) {
    if(argc != 2) {
        std::cout << "Usage: " << argv[0] << " NumberOfPrimes" << std::endl;
        std::exit(0);
    }
    uint64_t nbElements = std::stoll(argv[1]);
    std::cout << "Computing the sum of the " << nbElements << "first primes" << std::endl;
    std::vector<uint64_t> primes;
    primes.reserve(nbElements);
    primes.push_back(2);
    uint64_t i = 3;
    int nbPrimes = 1;
    for (uint64_t nb_primes = 1; nb_primes < nbElements; nb_primes++) {
        while( ! is_prime(i, primes) ) {
            i += 2;
        }
        nbPrimes ++;
        i += 2;
        //primes.push_back(i);
        if(nbPrimes % 10000 == 0) std::cout << nbPrimes << std::endl;
    }
    std::cout << nbPrimes;
    //std::cout << std::accumulate(std::begin(primes), std::end(primes), 0) << std::endl;
}
