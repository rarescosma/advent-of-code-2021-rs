use arrayvec::ArrayVec;
use std::ops::Add;

use num_traits::PrimInt;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
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

impl Pos {
    pub fn neighbors_simple(self) -> ArrayVec<Pos, 4> {
        [
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x, self.y - 1),
        ]
        .into_iter()
        .collect()
    }

    pub fn neighbors_diag(self) -> ArrayVec<Pos, 8> {
        [
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x + 1, self.y + 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x - 1, self.y - 1),
        ]
        .into_iter()
        .collect()
    }

    pub fn neighbors_diag_inclusive(self) -> ArrayVec<Pos, 9> {
        [
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x + 1, self.y + 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x - 1, self.y - 1),
            self,
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn map<U>(&self, x: U, y: U) -> Pos
    where
        U: PrimInt,
    {
        match self {
            Axis::X => (x, y).into(),
            Axis::Y => (y, x).into(),
        }
    }
}
