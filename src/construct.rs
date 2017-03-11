use ::{MstCcProblem, TrackConflicts};

use fera::fun::vec;
use fera::graph::prelude::*;
use fera::graph::props::FnProp;
use fera::graph::unionfind::WithUnionFind;

pub fn new_greedy(p: &MstCcProblem) -> Vec<Edge<StaticGraph>> {
    let mut edges = vec(p.g.edges());
    let mut conflicts = TrackConflicts::new(&p, vec![]);
    let mut ds = p.g.new_unionfind();
    let mut tree = vec![];
    let mut start = 0;
    while ds.num_sets() > 1 {
        edges.sort_by_prop(FnProp(|e| (conflicts.num_conflicts_of(e), p.w.get(e))));
        for (i, &e) in edges[start..].iter().enumerate() {
            let (u, v) = p.g.ends(e);
            if ds.in_same_set(u, v) {
                continue
            }
            ds.union(u, v);
            conflicts.add_edge(e);
            tree.push(e);
            start += i;
        }
    }
    tree
}
