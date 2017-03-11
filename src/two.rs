// system
use std::mem;

// external
use fera::fun::vec;
use fera::graph::prelude::*;
use fera::graph::props::FnProp;
use fera::graph::trees::Trees;

// local
use {MstCcProblem, TrackConflicts, TrackConnectivity2, log_improvement};

// TODO: Create a struct

// FIXME: remove allow attribute when https://github.com/rust-lang/rust/issues/28570 got fixed
#[allow(unused_assignments)]
pub fn two_opt(p: &MstCcProblem,
               tree: &mut [Edge<StaticGraph>],
               exclude: &[Edge<StaticGraph>],
               alfa: u32,
               beta: u32)
               -> u32 {
    let (g, w) = (&p.g, &p.w);
    let ww = |e, c: &TrackConflicts| alfa * w.get(e) + beta * c.num_conflicts_of(e);

    let mut in_tree = g.default_edge_prop(false);
    in_tree.set_values(&*tree, true);
    let mut non_tree = vec(g.edges().filter(|e| !in_tree[*e] && !exclude.contains(e)));

    let mut conflicts = TrackConflicts::new(p, tree.to_vec());

    let weight: u32 = sum_prop(w, &*tree);
    debug!("Start two with weight = {}", weight);

    let mut search = true;
    let mut connectivity = TrackConnectivity2::new(g);
    'out: while search {
        search = false;
        connectivity.set_edges(&*tree);
        // TODO: sort tree edges?
        tree.sort_by_prop(FnProp(|e| ww(e, &conflicts)));
        tree.reverse();
        non_tree.sort_by_prop(FnProp(|e| ww(e, &conflicts)));

        conflicts.check();
        assert!(g.spanning_subgraph(conflicts.edges()).is_tree());
        // i and j are the index of the edges to be replaced
        for i in 0..tree.len() {
            let ei = tree[i];
            let (a, b) = g.ends(ei);
            for j in (i + 1)..tree.len() {
                let ej = tree[j];
                let (c, d) = g.ends(ej);

                connectivity.reset();
                connectivity.disconnect(a, b);
                connectivity.disconnect(c, d);

                let prev_num_conflicts = conflicts.num_conflicts();
                conflicts.remove_edge(ei);
                conflicts.remove_edge(ej);

                // FIXME: wei can conflict with wij
                let wei = ww(ei, &conflicts);
                let wej = ww(ej, &conflicts);

                // TODO: use a better scheme to check connectivity
                let mut connected = [false, false, false];
                let mut new = [None, None];
                let mut sub_w = wei + wej;

                // search for new edges
                for k in 0..non_tree.len() {
                    let e = non_tree[k];
                    let we = ww(e, &conflicts);
                    if we >= sub_w {
                        continue;
                        //if conflicts.num_conflicts_of(e) == 0 {
                        //    break;
                        //} else {
                        //    continue;
                        //}
                    }

                    let (x, y) = g.ends(e);
                    let comp_x = connectivity.comp(x);
                    let comp_y = connectivity.comp(y);
                    if comp_x == comp_y || connected[comp_x] && connected[comp_y] {
                        continue;
                    }

                    // (x, y) is being added to the three, so update conflicts
                    conflicts.add_edge(e);
                    if new[0] == None {
                        // this is the first edge to be added to the tree
                        new[0] = Some(k);
                        sub_w -= we;
                        connected[comp_x] = true;
                        connected[comp_y] = true;
                    } else {
                        // this is the second edge
                        new[1] = Some(k);
                        break;
                    }
                }

                // if two new edges were found, put it on the tree
                if let (Some(k), Some(l)) = (new[0], new[1]) {
                    let pre_weight: u32 = sum_prop(w, &*tree);
                    mem::swap(&mut tree[i], &mut non_tree[k]);
                    mem::swap(&mut tree[j], &mut non_tree[l]);

                    let weight: u32 = sum_prop(w, &*tree);

                    log_improvement("conflicts", prev_num_conflicts, conflicts.num_conflicts());
                    log_improvement("weight   ", pre_weight, weight);

                    search = true;
                    continue 'out;
                }

                // the edges was not replaced, so restore conflicts
                if let Some(k) = new[0] {
                    conflicts.remove_edge(non_tree[k]);
                }

                conflicts.add_edge(ei);
                conflicts.add_edge(ej);
            }
        }
    }

    let weight: u32 = sum_prop(w, &*tree);
    debug!("End two with weight = {}", weight);

    conflicts.num_conflicts()
}
