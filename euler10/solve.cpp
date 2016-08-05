#include <vector>
#include <string>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <cassert>
#include <cmath>

using namespace std;

template<typename T>
class PrimeGenerator {
    vector<T> primes;

    public: T next() {
        T current;
        if(primes.size() == 0) {
            current = 2;
        } else {
            current = primes.back() + 1;
            while(! is_prime(current)) {
                current += 2;
            }
        }
        primes.push_back(current);
        return current;
    }

    protected:
    bool is_prime(uint64_t number) {
        uint64_t max = (uint64_t)ceil(sqrt(number));
        auto it = this->primes.cbegin();
        while(*it <= max) {
            if(number % *it == 0) {
                return false;
            }
            it += 2;
        }
        return true;
    }
};


void test() {
    PrimeGenerator<uint64_t> p;
    assert(p.next() == 2);
    assert(p.next() == 3);
    assert(p.next() == 5);
    assert(p.next() == 7);
    assert(p.next() == 11);
}

int main(int argc, char* argv[]) {
    test();
    uint64_t sumPrimes = 0;
    if(argc != 2) {
        cout << "Usage: " << argv[0] << " NumberOfPrimes" << endl;
        exit(0);
    }
    uint64_t nbElements = stoll(argv[1]);
    cout << "Computing the sum of the " << nbElements << "first primes" << endl;

    PrimeGenerator<uint64_t> generator;
    for (uint64_t nb_primes = 0; nb_primes < nbElements; nb_primes++) {
        uint64_t prime = generator.next();
        sumPrimes += (prime % 10000000);
    }
    cout << sumPrimes << endl;
}
