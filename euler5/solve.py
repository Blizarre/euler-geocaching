
def test():
    assert(get_prime_divisors(10) == {2: 1, 5: 1})
    assert(get_prime_divisors(9) == {3: 2})
    assert(get_prime_divisors(7) == {7: 1})

    divs = {2: 1, 3: 1, 4: 5}
    merge_divisors(divs, {2: 2, 4: 1, 6: 2})
    assert(divs == {2: 2, 3: 1, 4: 5, 6: 2})

    assert(f(3) == 6)
    assert(f(10) == 2520)
    print "tests ok"


def get_prime_divisors(nb):
    assert(nb != 0)
    divisors = {}
    i = 2
    while nb != 1:
        if nb % i == 0:
            divisors[i] = divisors.get(i, 0) + 1
            nb /= i
        else:
            i += 1
    return divisors


def merge_divisors(divisors, new_divisors):
    for (k, v) in new_divisors.items():
        divisors[k] = max(divisors.get(k, 0), v)


def f(nb):
    divisors = {}
    product = 1
    for i in range(1, nb + 1):
        merge_divisors(divisors, get_prime_divisors(i))
    for (k, v) in divisors.items():
        product *= k ** v
    return product

test()
print "Project euler"
print f(20)

print "Geocaching  euler"
print f(125)
print f(64)
print f(1370)
print f(81)
print f(4158)
