// external
use fera::fun::first;
use fera::graph::algs::Trees;
use fera::graph::params::IntoOwned;
use fera::graph::prelude::*;
use fera::graph::props::{Color, IgnoreWriteProp};
use fera::graph::traverse::{OnDiscoverVertex, OnFinishVertex, RecursiveDfs, StampTime, Time};

pub struct TrackConnectivity1<'a, G: 'a + IncidenceGraph> {
    g: SpanningSubgraph<'a, G>,
    // TODO: change to u32
    discover: DefaultVertexPropMut<SpanningSubgraph<'a, G>, u64>,
    finish: DefaultVertexPropMut<SpanningSubgraph<'a, G>, u64>,
    root: Vertex<G>,
    sub: Vertex<G>,
}

impl<'a, G: 'a + IncidenceGraph> TrackConnectivity1<'a, G> {
    pub fn new(g: &'a G) -> Self {
        let g = g.empty_spanning_subgraph();
        let discover = g.vertex_prop(0);
        let finish = g.vertex_prop(0);
        let root = first(g.vertices());
        TrackConnectivity1 {
            g: g,
            discover: discover,
            finish: finish,
            root: root,
            sub: root,
        }
    }

    fn dfs(&mut self) {
        // TODO: is_tree is allocating!
        assert!(self.g.is_tree());
        let time = Time::default();
        // g is a tree (acyclic and connected), so we can ignore the color changes
        self.g
            .recursive_dfs((
                OnDiscoverVertex(StampTime(&time, &mut self.discover)),
                OnFinishVertex(StampTime(&time, &mut self.finish)),
            ))
            .root(self.root)
            .color(&mut IgnoreWriteProp::new_vertex_prop(&self.g, Color::White))
            .run();
    }

    #[inline]
    fn is_ancestor_of(&self, ans: Vertex<G>, v: Vertex<G>) -> bool {
        self.discover[ans] <= self.discover[v] && self.finish[v] <= self.finish[ans]
    }

    pub fn set_edges<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: IntoOwned<Edge<G>>,
    {
        self.g.clear_edges();
        self.g.add_edges(iter);
        self.dfs();
    }

    pub fn is_connected(&self, u: Vertex<G>, v: Vertex<G>) -> bool {
        if self.is_ancestor_of(self.sub, u) {
            self.is_ancestor_of(self.sub, v)
        } else {
            !self.is_ancestor_of(self.sub, v)
        }
    }

    pub fn disconnect(&mut self, u: Vertex<G>, v: Vertex<G>) {
        self.sub = if self.is_ancestor_of(u, v) { v } else { u };
    }

    pub fn replace_edge(&mut self, rem: Edge<G>, ins: Edge<G>) {
        self.g.replace_edge(rem, ins);
        self.dfs()
    }
}
