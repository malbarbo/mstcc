#!/usr/bin/python3

import argparse
import math
import networkx as nx
import sys


def main():
    parser = argparse.ArgumentParser(
        description='Check mstcc solutions',
        epilog='the solutions are read from stdin (format: [weight] u1-v1 u2-v2 ...)')
    parser.add_argument(
        '-w',
        help='indicates that the first value in each solution is the weight',
        action="store_true")
    parser.add_argument(
        'graph',
        help='graph file (samer-urrutia format)')

    args = parser.parse_args()
    g, w, cc = read_graph(args.graph)

    check(args.w, g, w, cc)


def check(parse_weight, g, w, cc):
    n = g.number_of_nodes()
    vals = sys.stdin.read().split()
    csize = n if parse_weight else n - 1
    for i, vals in enumerate(chunks(vals, csize)):
        skip, weight = (1, int(vals[0])) if parse_weight else (0, None)
        edges = set(map(parse_edge, vals[skip:]))
        violations = check_one(edges, weight, g, w, cc)
        if violations != []:
            print('{} fail\n{}'.format(i, '  \n'.join(violations)))


def check_one(edges, weight, g, w, cc):
    expected_weight = sum(w[e] for e in edges)

    violations = []

    G = nx.Graph()
    G.add_edges_from(edges)
    if G.number_of_nodes() != g.number_of_nodes() or \
            G.number_of_edges() + 1 != g.number_of_nodes() or \
            not nx.algorithms.is_tree(G):
        violations.append('not a spanning tree')

    for e1 in edges:
        for e2 in edges:
            if (e1, e2) in cc:
                violations.append('conflicting edges: {}, {}'.format(e1, e2))

    if weight != None and weight != expected_weight:
        violations.append('weight {} != {}'.format(weight, expected_weight))

    return violations


def read_graph(filename):
    lines = open(filename).readlines()
    name, g, w, cc = parse(lines)
    return g, w, cc


def parse(lines):
    i = 0
    while lines[i].startswith('#'):
        i += 1
    name = lines[i]
    n = int(lines[i + 1])
    m = int(lines[i + 2])
    c = int(lines[i + 3])
    g = nx.Graph()
    w = {}
    cc = set()
    i += 4
    for line in lines[i:i + m]:
        (u, v, e) = list(map(int, line.split()))
        g.add_edge(u, v)
        w[(u, v)] = e
        w[(v, u)] = e
    i += m
    for line in lines[i:i + c]:
        (a, b, x, y) = list(map(int, line.split()))
        cc.add(((a, b), (x, y)))
        cc.add(((a, b), (y, x)))
        cc.add(((b, a), (x, y)))
        cc.add(((b, a), (y, x)))
    assert len(cc) == 4 * c
    return name, g, w, cc


def parse_edge(s):
    u, v = s.split('-')
    return int(u), int(v)


def chunks(lst, n):
    for i in range(0, len(lst), n):
        yield lst[i:i + n]


if __name__ == "__main__":
    main()
