#[macro_use]
extern crate log;

extern crate clap;
extern crate env_logger;
extern crate fera;
extern crate rand;

#[cfg(test)]
extern crate itertools;

mod conflicts;
mod connectivity1;
mod connectivity2;
mod construct;
mod input;
mod one;
mod two;
mod utils;

pub use conflicts::*;
pub use connectivity1::*;
pub use connectivity2::*;
pub use construct::*;
pub use input::*;
pub use one::*;
pub use two::*;
pub use utils::*;

// external
use fera::graph::prelude::*;

pub struct MstCcProblem {
    pub name: String,
    pub g: StaticGraph,
    pub w: DefaultEdgePropMut<StaticGraph, u32>,
    pub cc: DefaultEdgePropMut<StaticGraph, Vec<Edge<StaticGraph>>>,
    pub num_cc: usize,
}
