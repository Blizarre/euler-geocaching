#!/usr/bin/env python3

import sys
import itertools

index = int(sys.argv[1])

permutation = next(itertools.islice(itertools.permutations(range(10)), index - 1, index))

print("".join([str(i) for i in permutation]))
