// external
use fera::graph::ext::IntoOwned;
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
    pub fn new(p: &'a MstCcProblem) -> Self {
        let in_set = p.g.default_edge_prop(false);
        let cc = p.g.default_edge_prop(0);
        TrackConflicts {
            p: p,
            edges: vec![],
            in_set: in_set,
            cc: cc,
            conflicts: 0,
        }
    }

    pub fn with_edges<I>(p: &'a MstCcProblem, edges: I) -> Self
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<StaticGraph>>
    {
        let mut t = Self::new(p);
        t.add_edges(edges);
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

    pub fn add_edges<I>(&mut self, edges: I)
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<StaticGraph>>
    {
        for e in edges {
            self.add_edge(e.into_owned())
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
        let new = Self::with_edges(self.p, &self.edges);
        assert_eq!(new.num_conflicts(), self.num_conflicts());
        for e in self.p.g.edges() {
            assert_eq!(new.contains(e), self.contains(e));
            assert_eq!(new.num_conflicts_of(e), self.num_conflicts_of(e));
        }
    }
}
