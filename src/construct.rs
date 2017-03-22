// external
use fera::fun::vec;
use fera::graph::prelude::*;
use fera::graph::props::FnProp;
use fera::graph::unionfind::WithUnionFind;

// local
use {MstCcProblem, TrackConflicts};

pub fn new_greedy(p: &MstCcProblem, tree: &mut Vec<Edge<StaticGraph>>) {
    let mut edges = vec(p.g.edges());
    let mut conflicts = TrackConflicts::new(&p);
    let mut ds = p.g.new_unionfind();
    let mut start = 0;
    while ds.num_sets() > 1 {
        edges[start..].sort_by_prop(FnProp(|e| p.obj(p.w.get(e), conflicts[e])));
        for (i, &e) in edges[start..].iter().enumerate() {
            let (u, v) = p.g.ends(e);
            if ds.in_same_set(u, v) {
                continue;
            }
            ds.union(u, v);
            conflicts.add_edge(e);
            tree.push(e);
            start += i;
        }
    }
}
