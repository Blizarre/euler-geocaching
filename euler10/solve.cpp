#include <cassert>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

using namespace std;

template <typename T> class PrimeGenerator {
  vector<T> primes;
  T next_number;

public:

  PrimeGenerator() {
      next_number = 0;
  }

  T next() {
    T prime;
    // Initialization for the first iteration
    if (next_number == 0) {
      next_number = 3;
      prime = 2;
    } else {
      T candidate = next_number;
      while (!is_prime(candidate)) {
        candidate += 2;
      }
      prime = candidate;
      next_number = prime + 2;
    }
    primes.push_back(prime);
    return prime;
  }

protected:
  bool is_prime(const uint64_t number) {
    uint64_t max = (uint64_t)floor(sqrt(number));
    auto it = this->primes.cbegin();
    while (*it <= max) {
      if (number % *it == 0) {
        return false;
      }
      it ++;
    }
    return true;
  }
};


uint64_t sumOfPrimesBelow(const int max_prime, const bool verbose) {
  PrimeGenerator<uint64_t> generator;
  uint64_t sum_primes = 0;
  uint64_t count_primes = 0;
  uint64_t next_prime = generator.next();

  while(next_prime < max_prime) {
    sum_primes += next_prime;
    next_prime = generator.next();
    if(verbose && count_primes % 50000 == 0) {
        cout << "\033[A\033[K" <<  (int)( 100 * next_prime / max_prime) << "%" << endl;
    }
    count_primes ++;
  }
  return sum_primes;
}

uint64_t sumOfNbPrimes(const int nb_primes, const bool verbose) {
  uint64_t sum_primes = 0;

  PrimeGenerator<uint64_t> generator;
  for (uint64_t count_primes = 0; count_primes < nb_primes; count_primes++) {
    uint64_t prime = generator.next();
    sum_primes += (prime % 10'000'000);
    if(verbose && count_primes % 50000 == 0) {
        cout << "\033[A\033[K" <<  (int)( 100 * count_primes / nb_primes) << "%" << endl;
    }
  }
  return sum_primes;
}

void test() {
  PrimeGenerator<uint64_t> p;
  assert(p.next() == 2);
  assert(p.next() == 3);
  assert(p.next() == 5);
  assert(p.next() == 7);
  assert(p.next() == 11);
  assert(p.next() == 13);
  assert(p.next() == 17);
  assert(sumOfPrimesBelow(10, false) == 17);
  assert(sumOfNbPrimes(6, false) == 41);
}

int main(int argc, char *argv[]) {
  test();
  cout << "Computing the sum of the primes < 2'000'000 (euler)" << endl;
  cout << sumOfNbPrimes(2'000'000, true) << endl;
  cout << "Computing the sum mod 7 of the  8,971,406 first primes (euler)" << endl;
  cout << sumOfPrimesBelow(8'971'406, true) << endl;
  cout << "Computing the sum mod 7 of the 434'926 first primes (euler)" << endl;
  cout << sumOfNbPrimes(434'926, true) << endl;
}
