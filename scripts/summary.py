#!/usr/bin/python3

import csv
import os
import sys
import numpy as np


ALLOW = [
    'z50-200-199.gcc',
    'z50-200-398.gcc',
    'z50-200-597.gcc',
    'z50-200-995.gcc',
    'z100-300-448.gcc',
    'z100-300-897.gcc',
    'z100-500-1247.gcc',
    'z100-500-2495.gcc',
    'z100-500-3741.gcc',
    'z200-600-1797.gcc',
    'z200-800-3196.gcc',
]

NAME = 'Name'
TIME = 'Time'
CONFLICTS = 'Conflicts'
WEIGHT = 'Weight'

def summary(case, results):
    time = np.array(list(r[TIME] for r in results))
    conflicts = np.array(list(r[CONFLICTS] for r in results))
    weight = np.array(list(r[WEIGHT] for r in results))

    row = [case, len(results)]
    row.append('{:0.2f} ± {:0.2f}'.format(np.mean(time), np.std(time)))
    row.append('{}'.format(np.median(weight)))

    m = np.min(weight)
    row.append('{} / {}'.format(m, sum(1 for x in weight if x == m)))

    m = np.min(conflicts)
    row.append('{} / {}'.format(m, sum(1 for x in conflicts if x == m)))

    return row


def run(filename):
    lines = list(csv.DictReader(open(filename)))
    for line in lines:
        line[CONFLICTS] = int(line[CONFLICTS])
        line[WEIGHT] = int(line[WEIGHT])
        line[TIME] = float(line[TIME])


    table = [[
        'Name', 'Sam',
        'Time (s)',
        'Weight', 'Min Weight / Freq',
        'Min Conflicts / Freq'
    ]]
    saw = set()
    for line in lines:
        case = line[NAME]
        if case in saw or case not in ALLOW:
            continue
        saw.add(case)
        results = list(line for line in lines if line[NAME] == case)
        table.append(summary(case, results))

    for row in table:
        print('{:21}  {:4}  {:14}  {:10}  {:18}  {:8} '.format(*row))


def main():
    if len(sys.argv) != 2:
        print('Wrong number of parameters.')
        print('USAGE: {} directory'.format(sys.argv[0]))
        sys.exit(1)

    d = sys.argv[1]
    for f in sorted(os.listdir(d)):
        f = os.path.join(d, f)
        print(f)
        run(f)
        print()

if __name__ == '__main__':
    main()
