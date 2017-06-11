#!/usr/bin/env python3

# A little bit too easy in python
BIG_NUMBER = str(2**17973)

NORTH_INDEX = BIG_NUMBER.find('5210')

print("North: {}".format(
    BIG_NUMBER[NORTH_INDEX:NORTH_INDEX+7]))

print("South: {}".format(
    sum([int(i) for i in BIG_NUMBER[NORTH_INDEX:NORTH_INDEX + 3069]]) -
    sum([int(i) for i in BIG_NUMBER[:1000]])))
