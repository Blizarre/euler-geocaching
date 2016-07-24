#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#define BUFF_SIZE (256)
#define TRUE (1)
#define FALSE (0)

int is_palindrome(long int nb);
long int max_palindrome(long int max);


void test_is_palindrome() {
    assert(is_palindrome(88));
    assert(!is_palindrome(18));
    assert(is_palindrome(121));
    assert(is_palindrome(212));
    assert(!is_palindrome(221));
    assert(is_palindrome(1221));
    assert(!is_palindrome(2211));
    assert(is_palindrome(36863));
}

int is_palindrome(long int nb) {
    char buffer[BUFF_SIZE];
    size_t i, nb_len;
    snprintf(buffer, BUFF_SIZE, "%ld", nb);
    nb_len = strlen(buffer);
    for (i = 0; i < nb_len / 2; i++) {
        if (buffer[i] != buffer[nb_len - i - 1]) {
            return FALSE;
        }
    }
    return TRUE;
}


long int max_palindrome(long int max) {
    long int candidate, max_candidate = 0;
    long int nb1, nb2;

    // brute force all combination of {100..max} x {100..max}
    for(nb1 = max; nb1 >= 100; nb1--) {
        // we start from nb1 instead of max because we already checked numbers above nb1,
        // multiplication is symetric 200x100 == 100x200
        for(nb2 = nb1; nb2 >= 100; nb2--) {
            candidate = nb1 * nb2;
            if(is_palindrome(candidate) && candidate > max_candidate) {
                max_candidate = candidate;
            }
        }
    }
    return max_candidate;
}


void print_result(long int nb, int precision) {
    long int max = max_palindrome(nb);
    char buffer[BUFF_SIZE];
    assert(precision < BUFF_SIZE);
    snprintf(buffer, precision + 1, "%ld", max);
    printf("%ld (%d) : %s\n", nb, precision, buffer);
}

int main(int argc, char* argv[]) {
    test_is_palindrome();
    assert(max_palindrome(200) == 36863);

    printf("Euler 4 solution:\n");
    printf("%ld\n", max_palindrome(1000));

    printf("geocaching euler 4 solution:\n");
    print_result(747, 2);
    print_result(390, 2);
    print_result(750, 3);
    print_result(615, 3);
    print_result(564, 3);
    print_result(273, 1);
    print_result(911, 3);
}
