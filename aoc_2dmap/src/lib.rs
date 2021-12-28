#[macro_use]
extern crate itertools;

mod map;
mod pos;

pub use map::{Map, MapSize};
pub use pos::{Adjacency, Pos};
