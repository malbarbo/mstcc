// external
use fera::graph::prelude::*;

// local
use MstCcProblem;

pub struct TrackConflicts<'a> {
    p: &'a MstCcProblem, // TODO: only need p.cc and p.g
    edges: Vec<Edge<StaticGraph>>,
    in_set: DefaultEdgePropMut<StaticGraph, bool>,
    conflicts: u32,
    // numbers of edges in the tree that conflicts with e
    cc: DefaultEdgePropMut<StaticGraph, u32>,
}

impl<'a> TrackConflicts<'a> {
    pub fn new(p: &'a MstCcProblem, edges: Vec<Edge<StaticGraph>>) -> Self {
        let in_set = p.g.default_edge_prop(false);
        let cc = p.g.default_edge_prop(0);
        let mut t = TrackConflicts {
            p: p,
            edges: vec![],
            in_set: in_set,
            cc: cc,
            conflicts: 0,
        };

        for &e in &edges {
            t.add_edge(e);
        }

        t.edges = edges;

        t
    }

    pub fn reset(&mut self) {
        self.edges.clear();
        self.conflicts = 0;
        self.in_set.set_values(self.p.g.edges(), false);
        self.cc.set_values(self.p.g.edges(), 0);
    }

    pub fn replace(&mut self, rem: Edge<StaticGraph>, add: Edge<StaticGraph>) {
        self.remove_edge(rem);
        self.add_edge(add);
    }

    pub fn remove_edge(&mut self, rem: Edge<StaticGraph>) {
        assert!(self.in_set[rem]);
        self.in_set[rem] = false;
        let p = self.edges.iter().position(|x| *x == rem).unwrap();
        self.edges.remove(p);
        for &e in &self.p.cc[rem] {
            self.cc[e] -= 1;
            if self.in_set[e] {
                self.conflicts -= 1;
            }
        }
    }

    pub fn add_edge(&mut self, add: Edge<StaticGraph>) {
        assert!(!self.in_set[add]);
        self.in_set[add] = true;
        self.edges.push(add);
        for &e in &self.p.cc[add] {
            self.cc[e] += 1;
            if self.in_set[e] {
                self.conflicts += 1;
            }
        }
    }

    pub fn edges(&self) -> &[Edge<StaticGraph>] {
        &self.edges
    }

    pub fn contains(&self, e: Edge<StaticGraph>) -> bool {
        self.in_set[e]
    }

    pub fn num_conflicts_of(&self, e: Edge<StaticGraph>) -> u32 {
        self.cc[e]
    }

    pub fn num_conflicts(&self) -> u32 {
        self.conflicts
    }

    pub fn check(&self) {
        let new = Self::new(self.p, self.edges.clone());
        assert_eq!(new.num_conflicts(), self.num_conflicts());
        for e in self.p.g.edges() {
            assert_eq!(new.contains(e), self.contains(e));
            assert_eq!(new.num_conflicts_of(e), self.num_conflicts_of(e));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_edge() {
        let (p, e) = new();
        let mut t = TrackConflicts::new(&p, vec![]);

        t.add_edge(e[0]);
        assert_eq!(0, t.num_conflicts());
        assert_eq!(0, t.num_conflicts_of(e[0]));
        assert_eq!(1, t.num_conflicts_of(e[1]));
        assert_eq!(0, t.num_conflicts_of(e[2]));
        assert_eq!(1, t.num_conflicts_of(e[3]));
        assert_eq!(0, t.num_conflicts_of(e[4]));
        assert_eq!(0, t.num_conflicts_of(e[5]));

        t.add_edge(e[3]);
        assert_eq!(1, t.num_conflicts()); // e0 - e3
        assert_eq!(1, t.num_conflicts_of(e[0]));
        assert_eq!(1, t.num_conflicts_of(e[1]));
        assert_eq!(0, t.num_conflicts_of(e[2]));
        assert_eq!(1, t.num_conflicts_of(e[3]));
        assert_eq!(0, t.num_conflicts_of(e[4]));
        assert_eq!(0, t.num_conflicts_of(e[5]));

        t.add_edge(e[5]);
        assert_eq!(1, t.num_conflicts()); // e0 - e3
        assert_eq!(1, t.num_conflicts_of(e[0]));
        assert_eq!(2, t.num_conflicts_of(e[1]));
        assert_eq!(0, t.num_conflicts_of(e[2]));
        assert_eq!(1, t.num_conflicts_of(e[3]));
        assert_eq!(0, t.num_conflicts_of(e[4]));
        assert_eq!(0, t.num_conflicts_of(e[5]));

        t.add_edge(e[1]);
        assert_eq!(3, t.num_conflicts()); // e0 - e3, e0 - e1, e1 - e5
        assert_eq!(2, t.num_conflicts_of(e[0]));
        assert_eq!(2, t.num_conflicts_of(e[1]));
        assert_eq!(1, t.num_conflicts_of(e[2]));
        assert_eq!(1, t.num_conflicts_of(e[3]));
        assert_eq!(0, t.num_conflicts_of(e[4]));
        assert_eq!(1, t.num_conflicts_of(e[5]));
    }

    #[test]
    fn remove_edge() {
        let (p, e) = new();
        let mut t = TrackConflicts::new(&p, vec![]);
        t.add_edge(e[0]);
        t.add_edge(e[3]);
        t.add_edge(e[5]);
        t.add_edge(e[1]);

        t.remove_edge(e[3]);
        assert_eq!(2, t.num_conflicts()); // e0 - e1, e1 - e5
        assert_eq!(1, t.num_conflicts_of(e[0]));
        assert_eq!(2, t.num_conflicts_of(e[1]));
        assert_eq!(1, t.num_conflicts_of(e[2]));
        assert_eq!(1, t.num_conflicts_of(e[3]));
        assert_eq!(0, t.num_conflicts_of(e[4]));
        assert_eq!(1, t.num_conflicts_of(e[5]));
        t.check();

        t.remove_edge(e[1]);
        assert_eq!(0, t.num_conflicts());
        assert_eq!(0, t.num_conflicts_of(e[0]));
        assert_eq!(2, t.num_conflicts_of(e[1]));
        assert_eq!(0, t.num_conflicts_of(e[2]));
        assert_eq!(1, t.num_conflicts_of(e[3]));
        assert_eq!(0, t.num_conflicts_of(e[4]));
        assert_eq!(0, t.num_conflicts_of(e[5]));
        t.check();
    }

    fn new() -> (MstCcProblem, Vec<Edge<StaticGraph>>) {
        let mut b = <StaticGraph as WithBuilder>::Builder::new(4, 6);
        b.add_edge(0, 1); // e0
        b.add_edge(0, 2); // e1
        b.add_edge(0, 3); // e2
        b.add_edge(1, 2); // e3
        b.add_edge(1, 3); // e4
        b.add_edge(2, 3); // e5
        let (g, _, e) = b.finalize_();
        let cc = g.edge_prop(vec![]);
        let w = g.edge_prop(1);
        let mut p = MstCcProblem {
            name: "test".to_owned(),
            g: g,
            w: w,
            cc: cc,
            num_cc: 4
        };

        p.cc[e[0]].push(e[1]);
        p.cc[e[1]].push(e[0]);

        p.cc[e[0]].push(e[3]);
        p.cc[e[3]].push(e[0]);

        p.cc[e[1]].push(e[2]);
        p.cc[e[2]].push(e[1]);

        p.cc[e[1]].push(e[5]);
        p.cc[e[5]].push(e[1]);

        p.cc[e[2]].push(e[4]);
        p.cc[e[4]].push(e[2]);

        (p, e)
    }
}
