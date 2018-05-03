// external
use fera::fun::first;
use fera::graph::params::IntoOwned;
use fera::graph::prelude::*;
use fera::graph::props::{Color, IgnoreWriteProp};
use fera::graph::traverse::{OnDiscoverVertex, OnFinishVertex, RecursiveDfs, StampTime, Time};
use fera::graph::algs::Trees;

pub struct TrackConnectivity2<'a, G: 'a + IncidenceGraph> {
    g: SpanningSubgraph<'a, G>,
    // TODO: change to u32
    discover: DefaultVertexPropMut<SpanningSubgraph<'a, G>, u64>,
    finish: DefaultVertexPropMut<SpanningSubgraph<'a, G>, u64>,
    root: Vertex<G>,
    sub_a: Vertex<G>,
    sub_b: Vertex<G>,
}

impl<'a, G: 'a + IncidenceGraph> TrackConnectivity2<'a, G> {
    pub fn new(g: &'a G) -> Self {
        let g = g.empty_spanning_subgraph();
        let discover = g.vertex_prop(0);
        let finish = g.vertex_prop(0);
        let root = first(g.vertices());
        TrackConnectivity2 {
            g: g,
            discover: discover,
            finish: finish,
            root: root,
            sub_a: root,
            sub_b: root,
        }
    }

    fn dfs(&mut self) {
        // FIXME: is_tree is allocating!
        assert!(self.g.is_tree());
        let time = Time::default();
        // g is a tree (acyclic and connected), so we can ignore the color changes
        self.g
            .recursive_dfs((OnDiscoverVertex(StampTime(&time, &mut self.discover)),
                            OnFinishVertex(StampTime(&time, &mut self.finish))))
            .root(self.root)
            .color(&mut IgnoreWriteProp::new_vertex_prop(&self.g, Color::White))
            .run();
    }

    #[inline]
    fn is_ancestor_of(&self, ans: Vertex<G>, v: Vertex<G>) -> bool {
        self.discover[ans] <= self.discover[v] && self.finish[v] <= self.finish[ans]
    }

    pub fn set_edges<I>(&mut self, iter: I)
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<G>>
    {
        self.g.clear_edges();
        self.g.add_edges(iter);
        self.dfs();
    }

    pub fn disconnect2(&mut self, (u, v): (Vertex<G>, Vertex<G>), (x, y): (Vertex<G>, Vertex<G>)) {
        self.reset();
        self.disconnect(u, v);
        self.disconnect(x, y);
    }

    pub fn check_reconnect(&mut self,
                     (u, v): (Vertex<G>, Vertex<G>),
                     (x, y): (Vertex<G>, Vertex<G>))
                     -> bool {
        assert!(!self.is_connected(u, v));
        assert!(!self.is_connected(x, y));

        let mut con = [false; 3];
        con[self.comp(u)] = true;
        con[self.comp(v)] = true;
        con[self.comp(x)] = true;
        con[self.comp(y)] = true;

        con[0] && con[1] && con[2]
    }

    pub fn is_connected(&self, u: Vertex<G>, v: Vertex<G>) -> bool {
        self.comp(u) == self.comp(v)
    }

    pub fn disconnect(&mut self, u: Vertex<G>, v: Vertex<G>) {
        // FIXME: assert that this function in not called more than 2 times
        assert!(self.is_connected(u, v));

        let r = if self.is_ancestor_of(u, v) { v } else { u };
        let comp = self.comp(r);

        if comp == 0 {
            self.sub_b = self.sub_a;
            self.sub_a = r;
        } else if comp == 1 {
            self.sub_b = r;
        } else if comp == 2 {
            if self.root == self.sub_a {
                self.sub_a = r;
                self.sub_b = r
            } else {
                self.sub_b = r;
            }
        } else {
            unreachable!()
        }
    }

    pub fn comp(&self, v: Vertex<G>) -> usize {
        if self.is_ancestor_of(self.sub_b, v) {
            2
        } else if self.is_ancestor_of(self.sub_a, v) {
            1
        } else {
            0
        }
    }

    pub fn reset(&mut self) {
        self.sub_a = self.root;
        self.sub_b = self.root;
    }
}
