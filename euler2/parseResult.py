import sys

if len(sys.argv) != 2:
    print "1 argument required: input result file"
    sys.exit()

with open(sys.argv[1], 'r') as fd:
    for line in fd:
        label, number = [s.strip() for s in line.split()]
        sz = len(number)
        if sz % 2 == 0:
            number = number[sz/2 - 1:sz/2 + 1]
        else:
            number = number[sz/2 - 1: sz/2 + 2]
        print label, number
