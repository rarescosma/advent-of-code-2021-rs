use std::ops::Add;

use itertools::iproduct;
use num_traits::PrimInt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl<U> From<(U, U)> for Pos
where
    U: PrimInt,
{
    fn from(tpl: (U, U)) -> Self {
        Pos {
            x: tpl.0.to_i32().unwrap(),
            y: tpl.1.to_i32().unwrap(),
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub enum Adjacency {
    Simple,
    Diagonal,
    DiagonalInc,
}

impl Pos {
    pub fn neighbors(self, adjacency: Adjacency) -> Vec<Pos> {
        match adjacency {
            Adjacency::Simple => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            Adjacency::Diagonal => iproduct!(-1..=1, -1..=1)
                .filter(|&(x, y)| x != 0 || y != 0)
                .collect(),
            Adjacency::DiagonalInc => iproduct!(-1..=1, -1..=1).collect(),
        }
        .into_iter()
        .map(|(dy, dx)| self + Pos { x: dx, y: dy })
        .collect()
    }
}
