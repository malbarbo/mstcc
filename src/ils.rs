// system
use std::mem;

// external
use fera::fun::vec;
use fera::graph::prelude::*;
use fera::graph::sum_prop;
use fera::graph::algs::Kruskal;
use rand::{Rng, XorShiftRng};

// local
use MstCcProblem;

pub struct Ils<'a, R> {
    pub p: &'a MstCcProblem,
    pub max_iters: u32,
    pub max_iters_no_improv: u32,
    pub num_excludes: u32,
    pub iters_restart: u32,
    pub iters_restart_to_best: u32,
    pub restart: R,
    pub stop_on_feasible: bool,
}

impl<'a, R> Ils<'a, R> {
    #[inline(never)]
    pub fn run<F>(&mut self,
                  tree: &mut Vec<Edge<StaticGraph>>,
                  rng: &mut XorShiftRng,
                  mut local_search: F)
                  -> u32
        where F: FnMut(&mut Vec<Edge<StaticGraph>>) -> u32,
              R: FnMut(&mut Vec<Edge<StaticGraph>>)
    {
        let (g, w) = (&self.p.g, &self.p.w);
        let mut edges = vec(g.edges());
        let mut best = tree.clone();
        let mut best_weight = sum_prop(w, &*tree);
        let mut best_num_conflicts = u32::max_value();
        let mut best_obj = u32::max_value();

        let mut iters_no_impr = 0;
        let mut iters_restart = 0;
        let mut iters_restart_to_best = 0;

        let mut tmp = vec![];
        let mut exclude = vec![];

        for iter in 0..self.max_iters {
            let num_conflicts = local_search(tree);
            let weight = sum_prop(w, &*tree);
            let obj = self.p.obj(weight, num_conflicts);
            if obj < best_obj {
                info!("ils - iter      {}", iter);
                info!("ils - conflicts {} -> {}", best_num_conflicts, num_conflicts);
                info!("ils - weight    {} -> {}", best_weight, weight);
                best_num_conflicts = num_conflicts;
                best_weight = weight;
                best_obj = obj;
                best.clone_from(&*tree);
                iters_no_impr = 0;
                iters_restart = 0;
                iters_restart_to_best = 0;
            } else {
                iters_no_impr += 1;
                if iters_no_impr >= self.max_iters_no_improv {
                    break;
                }

                iters_restart += 1;
                if iters_restart >= self.iters_restart {
                    info!("ils - restart");
                    tree.clear();
                    (self.restart)(tree);
                    iters_restart = 0;
                    continue
                }

                iters_restart_to_best += 1;
                if iters_restart_to_best >= self.iters_restart_to_best {
                    info!("ils - restart to best");
                    tree.clone_from(&best);
                    iters_restart_to_best = 0;
                }
            }

            if self.stop_on_feasible && num_conflicts == 0 {
                break;
            }

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

        best_num_conflicts
    }
}
