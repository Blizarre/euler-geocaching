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

    BigInteger getA() {
        return BigInteger.valueOf(k * (m * m - n * n));
    }

    BigInteger getB() {
        return BigInteger.valueOf(k * 2 * m * n);
    }

    BigInteger getC() {
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

    static Result findSum(int goal) {
        for (int n = 1; n < goal / 2; n++) {
            int m = n + 1;
            while (2 * m * (m + n) <= goal) {
                int k = 1;
                while (k * 2 * m * (m + n) < goal) {
                    k++;
                }
                if (k * 2 * m * (m + n) == goal) {
                    return new Result(k, m, n);
                }
                m++;
            }
        }
        throw new IllegalArgumentException("Couldn't find valid a, b, or c for the given goal");
    }

    public static void main(String[] args) {
        System.out.println(String.format("sum is %d\n%s", 59196, findSum(59196)));
        System.out.println(String.format("sum is %d\n%s", 223060, findSum(223060)));
    }
}

