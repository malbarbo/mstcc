#[macro_use]
extern crate log;

extern crate clap;
extern crate env_logger;
extern crate fera;
extern crate rand;

mod conflicts;
mod connectivity1;
mod connectivity2;
mod construct;
mod ils;
mod input;
mod one;
mod two;
mod utils;

pub use conflicts::*;
pub use connectivity1::*;
pub use connectivity2::*;
pub use construct::*;
pub use ils::*;
pub use input::*;
pub use one::*;
pub use two::*;
pub use utils::*;

// system
use std::cell::Cell;

// external
use fera::graph::prelude::*;

pub struct MstCcProblem {
    pub name: String,
    pub g: StaticGraph,
    pub w: DefaultEdgePropMut<StaticGraph, u32>,
    pub cc: DefaultEdgePropMut<StaticGraph, Vec<Edge<StaticGraph>>>,
    pub num_cc: usize,
    pub alpha: Cell<u32>,
    pub beta: Cell<u32>,
}

impl MstCcProblem {
    fn obj(&self, weight: u32, num_conflicts: u32) -> u32 {
        self.alpha.get() * weight + self.beta.get() * num_conflicts
    }
}
