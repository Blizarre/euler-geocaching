import java.math.BigInteger;

class Result {

    int k;
    int m;
    int n;

    public Result(int k, int m, int n) {
        this.k = k;
        this.m = m;
        this.n = n;
    }

    public BigInteger getA() {
        return BigInteger.valueOf(k * (m * m - n * n));
    }

    public BigInteger getB() {
        return BigInteger.valueOf(k * 2 * m * n);
    }

    public BigInteger getC() {
        return BigInteger.valueOf(k * (m * m + n * n));
    }

    public BigInteger getProduct() {
        return getA().multiply(getB()).multiply(getC());
    }

    public String toString() {
        StringBuilder b = new StringBuilder();
        b.append(String.format("m %d, n %d, k %d\n", m, n, k));
        b.append(String.format("a %s, b %s, c %s\n", getA(), getB(), getC()));
        b.append(String.format("Product: %s\n", this.getProduct()));
        return b.toString();
    }
}

class Main {

    /**
     * Generate all possible triplet using the extended version of Euclid's formula
     * (https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple)
     * We try to find n, m and k. I choose to set the range of n and then increase k and m until we are over the goal.
     *
     * @param goal the expected sum of the triplet
     * @return The triplet that reach the goal
     * @throws IllegalArgumentException if no suitable coefficients have been found
     */
    static Result findSum(int goal) {
        for (int n = 1; n < goal / 2; n++) {
            int m = n + 1;

            while (getSum(1, m, n) <= goal) {
                int k = 1;
                while (getSum(k, m, n) < goal) {
                    k++;
                }
                if (getSum(k, m, n) == goal) {
                    return new Result(k, m, n);
                }
                m++;
            }
        }
        throw new IllegalArgumentException("Couldn't find valid a, b, or c for the given goal");
    }

    /**
     * Compute the sum a + b + c from the Euclid's formula coefficients
     *
     * @param k coefficient k
     * @param m coefficient m
     * @param n coefficient n
     * @return Sum a + b + c
     */
    static int getSum(int k, int m, int n) {
        // a + b + c = (m2 - n2) + 2mn + m2 + n2 = 2*m2 + 2mn + (n2 - n2) = 2 * m * ( m + n)
        return k * 2 * m * (m + n);
    }

    static void assertEqual(long a, long b) {
        if (a != b) {
            throw new AssertionError(String.format("Expected %d, got %d", a, b));
        }
    }

    static void test() {
        Result res = findSum(12);
        assertEqual(res.getA().min(res.getB()).longValue(), 3);
        assertEqual(res.getA().max(res.getB()).longValue(), 4);
        assertEqual(res.getC().longValue(), 5);
        assertEqual(res.getProduct().longValue(), 60);
    }

    public static void main(String[] args) {
        test();
        System.out.println(String.format("Euler project: sum is %d\n%s", 1000, findSum(1000)));

        System.out.println(String.format("Geocaching N: sum is %d\n%s", 59196, findSum(59196)));
        System.out.println(String.format("Geocaching E: sum is %d\n%s", 223060, findSum(223060)));
    }
}

