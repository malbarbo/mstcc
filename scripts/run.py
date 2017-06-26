#!/usr/bin/python3 -B

import os
import subprocess
import sys

FEASIBLE_TYPE1 = [
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

DIR_TYPE1 = '../data/type1'
ALL_TYPE1 = list(os.listdir(DIR_TYPE1))

DIR_TYPE2 = '../data/type2'
ALL_TYPE2 = list(os.listdir(DIR_TYPE2))

PROG = '../target/release/mstcc'

TIMES = 30

FEASIBLE_TYPE1 = list(DIR_TYPE1 + '/' + f for f in FEASIBLE_TYPE1)
ALL_TYPE1 = list(DIR_TYPE1 + '/' + f for f in ALL_TYPE1)
ALL_TYPE2 = list(DIR_TYPE2 + '/' + f for f in ALL_TYPE2)

def run_all(prog, cases, times):
    print_header()
    for i in range(times):
        eprint('Iter {}/{}'.format(i, times))
        for case in cases:
            m = case.split('-')[1]
            run(prog, case, m)


def run(prog, case, m):
    cmd = [prog, '--ils-max-iters', m]
    cmd.extend(sys.argv[1:])
    cmd.append(case)
    eprint(cmd)
    subprocess.check_call(cmd)


def print_header():
    print('Name,Time,Conflicts,Weight,Solution', flush=True)


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)
