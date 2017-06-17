#!/usr/bin/env python
import sys


nmb_mapping = {
    1: "one",
    2: "two",
    3: "three",
    4: "four",
    5: "five",
    6: "six",
    7: "seven",
    8: "eight",
    9: "nine",
    10: "ten",
    11: "eleven",
    12: "twelve",
    13: "thirteen",
    14: "fourteen",
    15: "fifteen",
    16: "sixteen",
    17: "seventeen",
    18: "eighteen",
    19: "nineteen",
    20: "twenty",
    30: "thirty",
    40: "forty",
    50: "fifty",
    60: "sixty",
    70: "seventy",
    80: "eighty",
    90: "ninety",
}

prepositional_mapping = {
    100: "hundred",
    1000: "thousand",
    1000000: "million"
}

zero = "zero"


def test():
    expected = {
        0: "zero",
        1: "one",
        342: "three hundred and forty-two",
        115: "one hundred and fifteen",
        48: "forty-eight",
        117: "one hundred and seventeen",
        635: "six hundred and thirty-five",
        9: "nine",
        1000000: "one million",
        1154: "one thousand one hundred and fifty-four"
    }

    for number, string in expected.items():
        if number_tostring(number) != string:
            raise AssertionError(
                "Number: {}, number_tostring: {}\n"
                "Expected: {}".format(number, number_tostring(number), string))


def number_tostring(number):
    if number == 0:
        return zero

    ret = []
    numbers = sorted(nmb_mapping.keys(), reverse=True)
    prep_numbers = sorted(prepositional_mapping.keys(), reverse=True)
    for nmb in prep_numbers:
        if number >= nmb:
            ret.append(number_tostring(number / nmb) + " " + prepositional_mapping[nmb])
            number = number % nmb

    if ret and number > 0:
        ret.append("and")

    for nmb in numbers:
        if number >= nmb:
            to_add = nmb_mapping[nmb]
            number -= nmb
            if number > 0:
                ret.append(to_add + "-" + nmb_mapping[number])
            else:
                ret.append(to_add)
            break

    return " ".join(ret)


def number_length(number):
    return len(number_tostring(i).replace("-", "").replace(" ", ""))


test()
max_number = int(sys.argv[1])
sum_words = 0
for i in range(1, max_number + 1):
    sum_words += number_length(i)
print "Sum of number's length up to {} is: {}".format(max_number, sum_words)
