extern crate fera;
extern crate itertools;
extern crate mstcc;
extern crate rand;

use fera::graph::algs::Components;
use fera::graph::prelude::*;
use itertools::Itertools;
use mstcc::TrackConnectivity1;

#[test]
fn test1() {
    let mut rng = rand::weak_rng();

    for n in 2..20 {
        let g = StaticGraph::new_random_tree(n, &mut rng);

        for (e, u, v) in g.edges_with_ends() {
            let sub = g.spanning_subgraph(g.edges().filter(|&w| w != e));
            let comps = sub.connected_components();

            let mut track_con = TrackConnectivity1::new(&g);
            track_con.set_edges(g.edges());
            track_con.disconnect(u, v);

            for (u, v) in g.vertices().tuple_combinations() {
                assert_eq!(comps.is_connected(u, v),
                           track_con.is_connected(u, v),
                           "n = {}, u = {}, v = {})",
                           n,
                           u,
                           v);
            }
        }
    }
}
