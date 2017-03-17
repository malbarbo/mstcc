// external
use fera::graph::prelude::*;
use fera::graph::props::FnProp;

// local
use {MstCcProblem, TrackConflicts, TrackConnectivity1, log_improvement};

pub struct OneEdgeReplacement<'a> {
    p: &'a MstCcProblem,
    in_tree: DefaultEdgePropMut<StaticGraph, bool>,
    non_tree: Vec<Edge<StaticGraph>>,
    connectivity: TrackConnectivity1<'a, StaticGraph>,
    conflicts: TrackConflicts<'a>,
}

impl<'a> OneEdgeReplacement<'a> {
    pub fn new(p: &'a MstCcProblem) -> Self {
        let connectivity = TrackConnectivity1::new(&p.g);
        let conflicts = TrackConflicts::new(&p);
        OneEdgeReplacement {
            p: p,
            in_tree: p.g.edge_prop(false),
            non_tree: vec![],
            connectivity: connectivity,
            conflicts: conflicts,
        }
    }

    #[allow(unused_assignments)]
    pub fn run(&mut self, tree: &mut [Edge<StaticGraph>]) -> u32 {
        self.setup(tree);

        let (p, g, w) = (&self.p, &self.p.g, &self.p.w);
        let obj = |e, c: &TrackConflicts| p.obj(w.get(e), c.num_conflicts_of(e));

        let connectivity = &mut self.connectivity;
        let conflicts = &mut self.conflicts;
        let non_tree = &mut self.non_tree;

        let mut weight = sum_prop(w, &*tree);
        let mut prev_weight = weight;
        let mut prev_num_conflicts = conflicts.num_conflicts();

        debug!("Start one-edge-replacement with weight = {}", weight);

        let mut search = true;
        'out: while search {
            search = false;
            non_tree.sort_by_prop(FnProp(|e| obj(e, &conflicts)));
            tree.sort_by_prop(w);
            tree.reverse();
            for i in 0..tree.len() {
                let rem = tree[i];
                let (a, b) = g.ends(rem);
                conflicts.remove_edge(rem);
                connectivity.disconnect(a, b);

                // TODO: sort non_tree edges here
                // TODO: filter edges in the same component
                // TODO: find a limit using binary search
                for j in 0..non_tree.len() {
                    let ins = non_tree[j];
                    if obj(ins, &conflicts) >= obj(rem, &conflicts) {
                        // all remaining non_tree have obj greater or equal than rem
                        break;
                    }

                    let (x, y) = g.ends(ins);

                    if connectivity.is_connected(x, y) {
                        continue;
                    }

                    connectivity.replace_edge(rem, ins);

                    conflicts.add_edge(ins);
                    conflicts.check();

                    // assert!(alfa == 0 || conflicts.num_conflicts() <= prev_num_conflicts);

                    tree[i] = ins;
                    non_tree[j] = rem;

                    weight -= w.get(rem);
                    weight += w.get(ins);

                    log_improvement("conflicts", prev_num_conflicts, conflicts.num_conflicts());
                    log_improvement("weight   ", prev_weight, weight);

                    prev_weight = prev_weight;
                    prev_num_conflicts = conflicts.num_conflicts();

                    search = true;

                    continue 'out;
                }

                conflicts.add_edge(rem);
            }
        }

        let expected_weight: u32 = sum_prop(w, &*tree);
        assert_eq!(expected_weight, weight);

        debug!("End one-edge-replacement with weight = {}", weight);

        conflicts.num_conflicts()
    }

    fn setup(&mut self, tree: &[Edge<StaticGraph>]) {
        self.connectivity.set_edges(&*tree);

        self.conflicts.reset();
        self.conflicts.add_edges(tree);

        self.in_tree.set_values(self.p.g.edges(), false);
        self.in_tree.set_values(tree, true);

        let in_tree = &self.in_tree;
        self.non_tree.clear();
        self.non_tree.extend(self.p.g.edges().filter(|e| !in_tree[*e]));
    }
}
