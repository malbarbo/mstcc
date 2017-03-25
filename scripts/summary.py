#!/usr/bin/python3

from natsort import natsorted
from tabulate import tabulate
import csv
import numpy as np
import os
import sys

NAME = 'Name'
TIME = 'Time'
CONFLICTS = 'Conflicts'
WEIGHT = 'Weight'

def summary(case, results):
    time = np.array(list(r[TIME] for r in results))
    conflicts = np.array(list(r[CONFLICTS] for r in results))
    weight = np.array(list(r[WEIGHT] for r in results))

    row = [case, len(results)]
    row.append('{}'.format(int(round(np.median(weight)))))
    m = np.min(weight)
    row.append(int(m))
    row.append(sum(1 for x in weight if x == m))

    row.append(np.median(time))
    row.append(np.sum(time))

    m = np.min(conflicts)
    row.append(int(m))
    row.append(sum(1 for x in conflicts if x == m))

    return row


def run(filename):
    lines = list(csv.DictReader(open(filename)))
    for line in lines:
        line[CONFLICTS] = int(line[CONFLICTS])
        line[WEIGHT] = int(line[WEIGHT])
        line[TIME] = float(line[TIME])


    table = [[
        'Name', 'Sam',
        'Obj', 'Best', 'Freq',
        'Time (s)', 'Total Time (s)',
        'Min Conf', 'Freq'
    ]]

    cases = natsorted(line[NAME] for line in lines)
    saw = set()
    for case in cases:
        if case in saw:
            continue
        saw.add(case)
        results = list(line for line in lines if line[NAME] == case)
        table.append(summary(case, results))

    print(tabulate(table, headers='firstrow', floatfmt=".03f"))


def main():
    if len(sys.argv) != 2:
        print('Wrong number of parameters.')
        print('USAGE: {} directory'.format(sys.argv[0]))
        sys.exit(1)

    path = sys.argv[1]

    if os.path.isdir(path):
        files = natsorted(os.path.join(path, f) for f in os.listdir(path))
    else:
        files = [path]

    for f in files:
        print(f)
        run(f)
        print()

if __name__ == '__main__':
    main()
