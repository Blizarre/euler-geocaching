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
    20: "twenty",
    30: "thirty",
    40: "fourty",
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


def number_tostring(number):
    ret = []
    numbers = sorted(nmb_mapping.keys(), reverse=True)
    prep_numbers = sorted(prepositional_mapping.keys(), reverse=True)
    for nmb in prep_numbers:
        if number >= nmb:
            ret.append(number_tostring(number / nmb) + " " + prepositional_mapping[nmb])
            number = number % nmb

    for nmb in numbers:
        if number >= nmb:
            ret.append(nmb_mapping[nmb])
            number -= nmb

    return " ".join(ret)


print number_tostring(int(sys.argv[1]))
