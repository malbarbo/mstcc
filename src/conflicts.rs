// internal
use std::ops::Index;

// external
use fera::graph::params::IntoOwned;
use fera::graph::prelude::*;

// local
use MstCcProblem;

// TODO: replace with OptionMax
const NONE: usize = ::std::usize::MAX;

pub struct TrackConflicts<'a> {
    p: &'a MstCcProblem, // TODO: only need p.cc and p.g
    edges: Vec<Edge<StaticGraph>>,
    // position of e in edges or NONE if e is not in edges
    pos: DefaultEdgePropMut<StaticGraph, usize>,
    conflicts: u32,
    // numbers of edges in the tree that conflicts with e
    cc: DefaultEdgePropMut<StaticGraph, u32>,
}

impl<'a> TrackConflicts<'a> {
    pub fn new(p: &'a MstCcProblem) -> Self {
        TrackConflicts {
            p: p,
            edges: vec![],
            pos: p.g.default_edge_prop(NONE),
            cc: p.g.default_edge_prop(0),
            conflicts: 0,
        }
    }

    pub fn with_edges<I>(p: &'a MstCcProblem, edges: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoOwned<Edge<StaticGraph>>,
    {
        let mut t = Self::new(p);
        t.add_edges(edges);
        t
    }

    pub fn reset(&mut self) {
        self.edges.clear();
        self.conflicts = 0;
        self.pos.set_values(self.p.g.edges(), NONE);
        self.cc.set_values(self.p.g.edges(), 0);
    }

    pub fn replace(&mut self, rem: Edge<StaticGraph>, add: Edge<StaticGraph>) {
        self.remove_edge(rem);
        self.add_edge(add);
    }

    pub fn remove_edge(&mut self, rem: Edge<StaticGraph>) {
        let p = self.pos[rem];
        assert!(p != NONE);
        self.edges.swap_remove(p);
        if p < self.edges.len() {
            self.pos[self.edges[p]] = p;
        }
        self.pos[rem] = NONE;
        for &e in &self.p.cc[rem] {
            self.cc[e] -= 1;
            if self.pos[e] != NONE {
                self.conflicts -= 1;
            }
        }
    }

    pub fn add_edges<I>(&mut self, edges: I)
    where
        I: IntoIterator,
        I::Item: IntoOwned<Edge<StaticGraph>>,
    {
        for e in edges {
            self.add_edge(e.into_owned())
        }
    }

    pub fn add_edge(&mut self, add: Edge<StaticGraph>) {
        let p = self.pos[add];
        assert!(p == NONE);
        self.edges.push(add);
        self.pos[add] = self.edges.len() - 1;
        for &e in &self.p.cc[add] {
            self.cc[e] += 1;
            if self.pos[e] != NONE {
                self.conflicts += 1;
            }
        }
    }

    pub fn edges(&self) -> &[Edge<StaticGraph>] {
        &self.edges
    }

    pub fn contains(&self, e: Edge<StaticGraph>) -> bool {
        self.pos[e] != NONE
    }

    pub fn total(&self) -> u32 {
        self.conflicts
    }

    pub fn check(&self) {
        let new = Self::with_edges(self.p, &self.edges);
        assert_eq!(new.total(), self.total());
        for e in self.p.g.edges() {
            assert_eq!(new.contains(e), self.contains(e));
            assert_eq!(new[e], self[e]);
        }
    }
}

impl<'a> Index<Edge<StaticGraph>> for TrackConflicts<'a> {
    type Output = u32;

    fn index(&self, e: Edge<StaticGraph>) -> &u32 {
        &self.cc[e]
    }
}
