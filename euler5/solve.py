
def test():
    assert(is_divisable_by_all(2520, 10))
    assert(not is_divisable_by_all(2521, 10))
    assert(f(10) == 2520)
    assert(f(3) == 6)
    print "tests ok"


def is_divisable_by_all(nb, divisors):
    for i in range(2, divisors+1):
        if nb % i != 0:
            return False
    return True


def f(nb):
    i = 1
    while not is_divisable_by_all(i, nb):
        i += 1
    return i

test()

print f(125)
print f(64)
print f(1370)
print f(81)
print f(4158)
