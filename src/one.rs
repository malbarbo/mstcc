// external
use fera::graph::prelude::*;
use fera::graph::props::FnProp;

// local
use {MstCcProblem, TrackConflicts, TrackConnectivity1, log_improvement};

pub struct OneEdgeReplacement<'a> {
    p: &'a MstCcProblem,
    non_tree: Vec<Edge<StaticGraph>>,
    connectivity: TrackConnectivity1<'a, StaticGraph>,
    conflicts: TrackConflicts<'a>,
}

impl<'a> OneEdgeReplacement<'a> {
    pub fn new(p: &'a MstCcProblem) -> Self {
        let connectivity = TrackConnectivity1::new(&p.g);
        let conflicts = TrackConflicts::new(&p, vec![]);
        OneEdgeReplacement {
            p: p,
            non_tree: vec![],
            connectivity: connectivity,
            conflicts: conflicts,
        }
    }

    #[allow(unused_assignments)]
    pub fn run(&mut self, tree: &mut [Edge<StaticGraph>], alfa: u32, beta: u32) -> u32 {
        self.prepare(tree);
        let (g, w) = (&self.p.g, &self.p.w);
        // weighted cost function
        let ww = |e, c: &TrackConflicts| alfa * w.get(e) + beta * c.num_conflicts_of(e);
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
            non_tree.sort_by_prop(FnProp(|e| ww(e, &conflicts)));
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
                    if ww(ins, &conflicts) >= ww(rem, &conflicts) {
                        // all remaining non_tree have ww greater or equal than rem
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

    pub fn prepare(&mut self, tree: &mut [Edge<StaticGraph>]) {
        self.connectivity.set_edges(&*tree);

        self.conflicts.reset();
        for &e in &*tree {
            self.conflicts.add_edge(e);
        }

        // TODO: move to the struct?
        let mut in_tree = self.p.g.default_edge_prop(false);
        in_tree.set_values(&*tree, true);

        self.non_tree.clear();
        self.non_tree.extend(self.p.g.edges().filter(|e| !in_tree[*e]));
    }
}
