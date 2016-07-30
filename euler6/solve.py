
def square(x):
    return x*x


def f(n):
    sum_square = sum([square(x) for x in range(1, n + 1)])
    square_sum = square(sum(range(1, n + 1)))
    return square_sum - sum_square


def test():
    assert(f(10) == 2640)

test()

print "Euler project:"
print 100, f(100)

print "Geocache:"
print 1201, f(1201)
print 1415, f(1415)
print 2215, f(2215)
print 1, f(1)
print 1378, f(1378)
print 1127, f(1127)
