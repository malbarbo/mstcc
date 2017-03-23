
extern crate mstcc;
extern crate fera;

use fera::graph::prelude::*;
use mstcc::{MstCcProblem, TrackConflicts};

#[test]
fn add_edge() {
    let (p, e) = new();
    let mut conflicts = TrackConflicts::new(&p);

    conflicts.add_edge(e[0]);
    assert_eq!(0, conflicts.total());
    assert_eq!(0, conflicts[e[0]]);
    assert_eq!(1, conflicts[e[1]]);
    assert_eq!(0, conflicts[e[2]]);
    assert_eq!(1, conflicts[e[3]]);
    assert_eq!(0, conflicts[e[4]]);
    assert_eq!(0, conflicts[e[5]]);

    conflicts.add_edge(e[3]);
    assert_eq!(1, conflicts.total()); // e0 - e3
    assert_eq!(1, conflicts[e[0]]);
    assert_eq!(1, conflicts[e[1]]);
    assert_eq!(0, conflicts[e[2]]);
    assert_eq!(1, conflicts[e[3]]);
    assert_eq!(0, conflicts[e[4]]);
    assert_eq!(0, conflicts[e[5]]);

    conflicts.add_edge(e[5]);
    assert_eq!(1, conflicts.total()); // e0 - e3
    assert_eq!(1, conflicts[e[0]]);
    assert_eq!(2, conflicts[e[1]]);
    assert_eq!(0, conflicts[e[2]]);
    assert_eq!(1, conflicts[e[3]]);
    assert_eq!(0, conflicts[e[4]]);
    assert_eq!(0, conflicts[e[5]]);

    conflicts.add_edge(e[1]);
    assert_eq!(3, conflicts.total()); // e0 - e3, e0 - e1, e1 - e5
    assert_eq!(2, conflicts[e[0]]);
    assert_eq!(2, conflicts[e[1]]);
    assert_eq!(1, conflicts[e[2]]);
    assert_eq!(1, conflicts[e[3]]);
    assert_eq!(0, conflicts[e[4]]);
    assert_eq!(1, conflicts[e[5]]);
}

#[test]
fn remove_edge() {
    let (p, e) = new();
    let mut conflicts = TrackConflicts::new(&p);
    conflicts.add_edge(e[0]);
    conflicts.add_edge(e[3]);
    conflicts.add_edge(e[5]);
    conflicts.add_edge(e[1]);

    conflicts.remove_edge(e[3]);
    assert_eq!(2, conflicts.total()); // e0 - e1, e1 - e5
    assert_eq!(1, conflicts[e[0]]);
    assert_eq!(2, conflicts[e[1]]);
    assert_eq!(1, conflicts[e[2]]);
    assert_eq!(1, conflicts[e[3]]);
    assert_eq!(0, conflicts[e[4]]);
    assert_eq!(1, conflicts[e[5]]);
    conflicts.check();

    conflicts.remove_edge(e[1]);
    assert_eq!(0, conflicts.total());
    assert_eq!(0, conflicts[e[0]]);
    assert_eq!(2, conflicts[e[1]]);
    assert_eq!(0, conflicts[e[2]]);
    assert_eq!(1, conflicts[e[3]]);
    assert_eq!(0, conflicts[e[4]]);
    assert_eq!(0, conflicts[e[5]]);
    conflicts.check();
}

fn new() -> (MstCcProblem, Vec<Edge<StaticGraph>>) {
    let mut b = <StaticGraph as WithBuilder>::Builder::new(4, 6);
    b.add_edge(0, 1); // e0
    b.add_edge(0, 2); // e1
    b.add_edge(0, 3); // e2
    b.add_edge(1, 2); // e3
    b.add_edge(1, 3); // e4
    b.add_edge(2, 3); // e5
    let (g, _, e) = b.finalize_();
    let cc = g.edge_prop(vec![]);
    let w = g.edge_prop(1);
    let mut p = MstCcProblem {
        name: "test".to_owned(),
        g: g,
        w: w,
        cc: cc,
        num_cc: 4,
        alpha: 1.into(),
        beta: 0.into(),
    };

    p.cc[e[0]].push(e[1]);
    p.cc[e[1]].push(e[0]);

    p.cc[e[0]].push(e[3]);
    p.cc[e[3]].push(e[0]);

    p.cc[e[1]].push(e[2]);
    p.cc[e[2]].push(e[1]);

    p.cc[e[1]].push(e[5]);
    p.cc[e[5]].push(e[1]);

    p.cc[e[2]].push(e[4]);
    p.cc[e[4]].push(e[2]);

    (p, e)
}
