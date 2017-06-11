#!/usr/bin/env python3

# https://projecteuler.net/problem=15

import sys
from middledigits import find_middle_digits

GRID_SIZE = int(sys.argv[1])

print("grid size: ({}, {})".format(GRID_SIZE, GRID_SIZE))

# We will use dynamic programming to solve the problem. Starting from the
# last row (let's choose index 0), for which we know all the values
# (you always only have one choice: go right), we will go up, one row
# at a time, until we reach the first row.
#
# number_of_route[row, col] = number_of_route[row-1, col] + number_of_route[row, col+1]

# We will reuse the same row array each time (memory space used O(GRID_SIZE)), because
# going from right to left, you can just increment the previous value for each
# cell with the new value from the cell on the right, as the previous value will
# be the number of path for the same cell, one row down.
row = [1] * (GRID_SIZE + 1)

# computing each row one after the other
for _ in range(GRID_SIZE):
    # we go from right to left, knowing that row[i] is the value of the previous
    # row, while row[i+1] is the value of the current row:
    for i in range(GRID_SIZE - 1, -1, -1):
        row[i] += row[i + 1]

print("Result: {}".format(row[0]))
print("Middle digits: {}".format(find_middle_digits(row[0])))
