// system
use std::mem;

// external
use fera::fun::vec;
use fera::graph::prelude::*;
use fera::graph::kruskal::Kruskal;
use rand::{Rng, XorShiftRng};

// local
use MstCcProblem;

pub struct Ils<'a> {
    pub p: &'a MstCcProblem,
    pub max_iters: u32,
    pub max_iters_no_improv: u32,
    pub num_excludes: u32,
}

impl<'a> Ils<'a> {
    pub fn run<F>(&mut self,
                  tree: &mut Vec<Edge<StaticGraph>>,
                  rng: &mut XorShiftRng,
                  mut fun: F)
                  -> u32
        where F: FnMut(&mut Vec<Edge<StaticGraph>>) -> u32
    {
        let (g, w) = (&self.p.g, &self.p.w);
        let mut edges = vec(g.edges());
        let mut b_weight: u32 = sum_prop(w, &*tree);
        let mut b_num_conflicts = 100_000;
        let mut best = tree.clone();
        let mut tmp = vec![];
        let mut exclude = vec![];
        let mut iters_no_impr = 0;

        for _ in 0..self.max_iters {
            let num_conflicts = fun(tree);
            let weight: u32 = sum_prop(w, &*tree);
            if self.p.obj(weight, num_conflicts) < self.p.obj(b_weight, b_num_conflicts) {
                info!("ils - conflicts {} -> {}", b_num_conflicts, num_conflicts);
                info!("ils - weight    {} -> {}", b_weight, weight);
                b_num_conflicts = num_conflicts;
                b_weight = weight;
                best.clone_from(&*tree);
                iters_no_impr = 0;
            } else {
                iters_no_impr += 1;
                if iters_no_impr >= self.max_iters_no_improv {
                    break;
                }
            }

            // TODO: use tree.clone_from(&best)?
            exclude.clear();
            for _ in 0..self.num_excludes {
                let i = rng.gen_range(0, tree.len());
                exclude.push(tree.swap_remove(i));
            }

            tmp.clear();
            while tmp.len() != g.num_vertices() - 1 {
                tmp.clear();
                // FIXME: use sample without replacement
                rng.shuffle(&mut edges);
                {
                    let edges = tree.iter().chain(edges.iter().filter(|e| !exclude.contains(*e)));
                    tmp.extend(g.kruskal().edges(edges));
                }
                exclude.clear();
            }
            mem::swap(tree, &mut tmp);
        }

        tree.clone_from(&best);

        b_num_conflicts
    }
}
