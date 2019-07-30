package sample

import java.math.BigInteger

fun main() {
    val limit = 29000
    print("Looking for the abundant numbers")
    val abundantNumbers = findAbundantNumbers(max = limit)

    val bitmap = Array(limit) { false }

    print("Computing bitmap")
    for (left in abundantNumbers)
        for (right in abundantNumbers) {
            if (left + right >= limit) {
                break
            } else {
                bitmap[left + right] = true
            }
        }
    bitmap.forEachIndexed { index, value -> if (!value) println(index) }
    val sum = bitmap.foldIndexed(BigInteger.ZERO) { index, accumulator, isSum ->
        if (!isSum) {
            accumulator + index.toBigInteger()
        } else {
            accumulator
        }
    }

    val product = bitmap.foldIndexed(BigInteger.ONE) { index, accumulator, isSum ->
        if (index > 0 && !isSum) {
            accumulator * index.toBigInteger()
        } else {
            accumulator
        }
    }
    println("Project euler: ${sum}")
    val productS = product.toString()
    println("Geocaching: ${productS.length}, ${product.toString().subSequence(999 until 1003)}")
    println("N 52 1${product.toString().subSequence(4218 until 4222)}")
    println("E 0   ${product.toString().subSequence(3954 until 3958)}")
}

fun findProperDivisors(number: Int): List<Int> =
    (1 until number).filter { number % it == 0 }

fun isAbundant(number: Int): Boolean =
    findProperDivisors(number).fold(0, Int::plus) > number


fun findAbundantNumbers(max: Int): List<Int> =
    (1..max).filter(::isAbundant)
