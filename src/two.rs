// system
use std::mem;

// external
use fera::graph::prelude::*;
use fera::graph::props::FnProp;
use fera::graph::algs::Trees;
use fera::graph::sum_prop;

// local
use {MstCcProblem, TrackConflicts, TrackConnectivity2, log_improvement};

pub struct TwoEdgeReplacement<'a> {
    p: &'a MstCcProblem,
    in_tree: DefaultEdgePropMut<StaticGraph, bool>,
    non_tree: Vec<Edge<StaticGraph>>,
    conflicts: TrackConflicts<'a>,
    connectivity: TrackConnectivity2<'a, StaticGraph>,
    weight: u32,
    num_conflicts: u32,
    obj: u32,
    c01: Vec<usize>,
    c02: Vec<usize>,
    c12: Vec<usize>,
    pub sort: bool,
    pub stop_on_feasible: bool,
}

impl<'a> TwoEdgeReplacement<'a> {
    pub fn new(p: &'a MstCcProblem) -> Self {
        TwoEdgeReplacement {
            p: p,
            non_tree: Vec::with_capacity(p.g.num_edges()),
            in_tree: p.g.edge_prop(false),
            conflicts: TrackConflicts::new(p),
            connectivity: TrackConnectivity2::new(&p.g),
            weight: 0,
            num_conflicts: 0,
            obj: 0,
            c01: vec![],
            c02: vec![],
            c12: vec![],
            sort: false,
            stop_on_feasible: false,
        }
    }

    pub fn run(&mut self, tree: &mut [Edge<StaticGraph>]) -> u32 {
        self.setup(tree);

        debug!("Start two-edge-replacement with weight = {}", self.weight);

        let mut s = 0;
        loop {
            if self.stop_on_feasible && self.conflicts.total() == 0 {
                break;
            }
            if let Some(ss) = self.two_replacement(tree, s) {
                s = ss;
            } else {
                break;
            }
        }

        let expected_weight: u32 = sum_prop(&self.p.w, &*tree);
        assert_eq!(expected_weight, self.weight);

        debug!("End two-edge-replacement with weight = {}", self.weight);

        self.conflicts.total()
    }

    pub fn two_replacement(&mut self, tree: &mut [Edge<StaticGraph>], s: usize) -> Option<usize> {
        self.check_conflicts();

        self.sort(tree);

        for i in s..tree.len() {
            let (ei, a, b) = self.p.g.ends(tree[i]);

            self.conflicts.remove_edge(ei);

            for j in (i + 1)..tree.len() {
                let (ej, c, d) = self.p.g.ends(tree[j]);

                self.conflicts.remove_edge(ej);
                self.connectivity.disconnect2((a, b), (c, d));

                if let Some((k, l)) = self.find_replace(ei, ej) {
                    self.replace(tree, (i, j), (k, l));
                    return Some(i);
                }

                self.conflicts.add_edge(ej);
            }

            self.conflicts.add_edge(ei);
        }

        None
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


    /*
    fn find_replace2(&mut self,
                    ei: Edge<StaticGraph>,
                    ej: Edge<StaticGraph>)
                    -> Option<(usize, usize)> {

        // FIXME: wei can conflict with wij
        let wei = self.obj_edge(ei);
        let wej = self.obj_edge(ej);

        // TODO: use a better scheme to check connectivity
        let mut connected = [false, false, false];
        let mut new = [None, None];
        let mut sub_w = wei + wej;

        for k in 0..self.non_tree.len() {
            let e = self.non_tree[k];
            let we = self.obj_edge(e);

            if we >= sub_w {
                if self.sort {
                    break
                } else {
                    continue
                }
            }

            let (x, y) = self.p.g.ends(e);
            let comp_x = self.connectivity.comp(x);
            let comp_y = self.connectivity.comp(y);
            if comp_x == comp_y || connected[comp_x] && connected[comp_y] {
                continue;
            }

            self.conflicts.add_edge(e);
            if new[0] == None {
                new[0] = Some(k);
                sub_w -= we;
                connected[comp_x] = true;
                connected[comp_y] = true;
            } else {
                new[1] = Some(k);
                break;
            }
        }

        match (new[0], new[1]) {
            (Some(k), Some(l)) => Some((k, l)),
            (Some(k), None) => {
                self.conflicts.remove_edge(self.non_tree[k]);
                None
            },
            _ => None,
        }
    }
    */

    fn find_replace(&mut self,
                     ei: Edge<StaticGraph>,
                     ej: Edge<StaticGraph>)
                     -> Option<(usize, usize)> {
        self.separate_comps(ei, ej);

        self.find_replace_(ei, ej, 0, 1)
            .or_else(|| self.find_replace_(ei, ej, 0, 2))
            .or_else(|| self.find_replace_(ei, ej, 1, 2))
    }

    fn separate_comps(&mut self, ei: Edge<StaticGraph>, ej: Edge<StaticGraph>) {
        self.c01.clear();
        self.c02.clear();
        self.c12.clear();

        for k in 0..self.non_tree_limit(ei, ej) {
            let (e, u, v) = self.p.g.ends(self.non_tree[k]);

            if self.obj_edge(e) >= self.obj_edge(ei) + self.obj_edge(ej) {
                continue;
            }

            match (self.connectivity.comp(u), self.connectivity.comp(v)) {
                (x, y) if x == y => continue,
                (0, 1) | (1, 0) => self.c01.push(k),
                (0, 2) | (2, 0) => self.c02.push(k),
                (1, 2) | (2, 1) => self.c12.push(k),
                _ => unreachable!(),
            }
        }
    }

    fn non_tree_limit(&self, ei: Edge<StaticGraph>, ej: Edge<StaticGraph>) -> usize {
        if self.sort {
            let w = self.p.w.get(ei) + self.p.w.get(ej);
            let c = self.conflicts[ei] + self.conflicts[ej] + 2;
            let key = self.p.obj(w, c);
            match self.non_tree.binary_search_by_key(&key, |e| self.obj_edge(*e)) {
                Ok(to) | Err(to) => to,
            }
        } else {
            self.non_tree.len()
        }
    }

    fn find_replace_(&mut self,
                     ei: Edge<StaticGraph>,
                     ej: Edge<StaticGraph>,
                     a: usize,
                     b: usize)
                     -> Option<(usize, usize)> {
        let aa = if a == 0 {
            &self.c01
        } else if a == 1 {
            &self.c02
        } else {
            &self.c12
        };

        let bb = if b == 0 {
            &self.c01
        } else if b == 1 {
            &self.c02
        } else {
            &self.c12
        };

        let w = &self.p.w;
        for &k in aa {
            let ek = self.non_tree[k];

            self.conflicts.add_edge(ek);

            for &l in bb {
                let el = self.non_tree[l];

                self.conflicts.add_edge(el);

                let new_weight = self.weight - w.get(ei) - w.get(ej) + w.get(ek) + w.get(el);
                let new_obj = self.p.obj(new_weight, self.conflicts.total());

                if new_obj < self.obj {
                    return Some((k, l));
                }

                self.conflicts.remove_edge(el);
            }

            self.conflicts.remove_edge(ek);
        }

        None
    }

    fn replace(&mut self,
               tree: &mut [Edge<StaticGraph>],
               (i, j): (usize, usize),
               (k, l): (usize, usize)) {
        mem::swap(&mut tree[i], &mut self.non_tree[k]);
        mem::swap(&mut tree[j], &mut self.non_tree[l]);

        self.connectivity.set_edges(&*tree);

        let num_conflicts = self.conflicts.total();
        let weight = sum_prop(&self.p.w, &*tree);

        log_improvement("conflicts", self.num_conflicts, num_conflicts);
        log_improvement("weight   ", self.weight, weight);

        self.num_conflicts = num_conflicts;
        self.weight = weight;
        self.obj = self.p.obj(self.weight, self.num_conflicts);
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
        self.obj = self.p.obj(self.weight, self.num_conflicts);
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
