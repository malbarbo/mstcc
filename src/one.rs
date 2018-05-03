// internal
use std::mem;

// external
use fera::graph::prelude::*;
use fera::graph::props::FnProp;
use fera::graph::sum_prop;
use fera::graph::algs::Trees;

// local
use {MstCcProblem, TrackConflicts, TrackConnectivity1, log_improvement};

pub struct OneEdgeReplacement<'a> {
    p: &'a MstCcProblem,
    in_tree: DefaultEdgePropMut<StaticGraph, bool>,
    non_tree: Vec<Edge<StaticGraph>>,
    conflicts: TrackConflicts<'a>,
    connectivity: TrackConnectivity1<'a, StaticGraph>,
    weight: u32,
    num_conflicts: u32,
    pub sort: bool,
    pub stop_on_feasible: bool,
}

impl<'a> OneEdgeReplacement<'a> {
    pub fn new(p: &'a MstCcProblem) -> Self {
        OneEdgeReplacement {
            p: p,
            non_tree: Vec::with_capacity(p.g.num_edges()),
            in_tree: p.g.edge_prop(false),
            conflicts: TrackConflicts::new(p),
            connectivity: TrackConnectivity1::new(&p.g),
            weight: 0,
            num_conflicts: 0,
            sort: false,
            stop_on_feasible: false,
        }
    }

    pub fn run(&mut self, tree: &mut [Edge<StaticGraph>]) -> u32 {
        self.setup(tree);

        debug!("Start one-edge-replacement with weight = {}", self.weight);

        loop {
            if self.stop_on_feasible && self.conflicts.total() == 0 {
                break;
            }
            if !self.one_replacement(tree) {
                break;
            }
        }

        let expected_weight: u32 = sum_prop(&self.p.w, &*tree);
        assert_eq!(expected_weight, self.weight);

        debug!("End one-edge-replacement with weight = {}", self.weight);

        self.conflicts.total()
    }

    pub fn one_replacement(&mut self, tree: &mut [Edge<StaticGraph>]) -> bool {
        self.check_conflicts();

        self.sort(tree);

        for i in 0..tree.len() {
            let (rem, a, b) = self.p.g.ends(tree[i]);

            self.conflicts.remove_edge(rem);
            self.connectivity.disconnect(a, b);

            if let Some(j) = self.find_replace(rem) {
                self.replace(tree, i, j);
                return true;
            }

            self.conflicts.add_edge(rem);
        }

        false
    }

    fn sort(&mut self, tree: &mut [Edge<StaticGraph>]) {
        if self.sort {
            let p = &self.p;
            let conflicts = &self.conflicts;
            let obj = FnProp(|e| p.obj(p.w.get(e), conflicts[e]));
            self.non_tree.sort_by_prop(&obj);
            tree.sort_by_prop(&obj);
            tree.reverse();
        }
    }

    fn find_replace(&self, rem: Edge<StaticGraph>) -> Option<usize> {
        for j in 0..self.non_tree_limit(rem) {
            let ins = self.non_tree[j];

            if self.obj_edge(ins) >= self.obj_edge(rem) {
                continue;
            }

            let (x, y) = self.p.g.ends(ins);

            if self.connectivity.is_connected(x, y) {
                continue;
            }

            return Some(j);
        }

        None
    }

    fn non_tree_limit(&self, rem: Edge<StaticGraph>) -> usize {
        if self.sort {
            let w = self.p.w.get(rem);
            let c = self.conflicts[rem] + 1;
            let key = self.p.obj(w, c);
            match self.non_tree.binary_search_by_key(&key, |e| self.obj_edge(*e)) {
                Ok(m) | Err(m) => m,
            }
        } else {
            self.non_tree.len()
        }
    }

    fn replace(&mut self, tree: &mut [Edge<StaticGraph>], i: usize, j: usize) {
        let rem = tree[i];
        let ins = self.non_tree[j];

        mem::swap(&mut tree[i], &mut self.non_tree[j]);

        self.connectivity.replace_edge(rem, ins);

        self.conflicts.add_edge(ins);

        let num_conflicts = self.conflicts.total();
        let weight = self.weight + self.p.w.get(ins) - self.p.w.get(rem);

        log_improvement("conflicts", self.num_conflicts, num_conflicts);
        log_improvement("weight   ", self.weight, weight);

        self.num_conflicts = num_conflicts;
        self.weight = weight;
    }

    fn setup(&mut self, tree: &[Edge<StaticGraph>]) {
        self.connectivity.set_edges(&*tree);

        self.conflicts.reset();
        self.conflicts.add_edges(tree);

        self.in_tree.set_values(self.p.g.edges(), false);
        self.in_tree.set_values(tree, true);

        let in_tree = &self.in_tree;
        self.non_tree.clear();

        let g = &self.p.g;
        self.non_tree.extend(g.edges().filter(|e| !in_tree[*e]));

        self.weight = sum_prop(&self.p.w, &*tree);
        self.num_conflicts = self.conflicts.total();
    }

    fn check_conflicts(&self) {
        self.conflicts.check();
        let g = &self.p.g;
        assert!(g.spanning_subgraph(self.conflicts.edges()).is_tree());
    }

    fn obj_edge(&self, e: Edge<StaticGraph>) -> u32 {
        self.p.obj(self.p.w.get(e), self.conflicts[e])
    }
}
