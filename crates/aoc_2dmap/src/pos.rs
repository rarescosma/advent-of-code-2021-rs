use std::iter::once;
use std::ops::{Add, AddAssign, Sub};

use num_traits::PrimInt;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub const fn c_new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn new<X: PrimInt, Y: PrimInt>(x: X, y: Y) -> Self {
        Self::from((x, y))
    }

    pub fn signum(self) -> Self {
        Pos {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl AsRef<Pos> for Pos {
    fn as_ref(&self) -> &Pos {
        self
    }
}

impl<X, Y> From<(X, Y)> for Pos
where
    X: PrimInt,
    Y: PrimInt,
{
    fn from(tpl: (X, Y)) -> Self {
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

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Pos {
    pub fn iter(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.y).flat_map(move |y| (0..self.x).map(move |x| Pos { x, y }))
    }

    pub fn neighbors_simple(self) -> impl Iterator<Item = Pos> {
        [
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x, self.y - 1),
        ]
        .into_iter()
    }

    pub fn neighbors_rdlu(self) -> impl Iterator<Item = Pos> {
        [
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y - 1),
        ]
        .into_iter()
    }

    pub fn neighbors_simple_inclusive(self) -> impl Iterator<Item = Pos> {
        self.neighbors_simple().chain(once(self))
    }

    pub fn neighbors_diag(self) -> impl Iterator<Item = Pos> {
        self.neighbors_simple().chain([
            Pos::new(self.x + 1, self.y + 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x - 1, self.y - 1),
        ])
    }

    pub fn neighbors_diag_inclusive(self) -> impl Iterator<Item = Pos> {
        self.neighbors_diag().chain(once(self))
    }

    pub fn neighbors_only_diag(self) -> impl Iterator<Item = Pos> {
        [
            Pos::new(self.x + 1, self.y + 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x - 1, self.y - 1),
        ]
        .into_iter()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn map<X, Y>(&self, x: X, y: Y) -> Pos
    where
        X: PrimInt,
        Y: PrimInt,
    {
        match self {
            Axis::X => (x, y).into(),
            Axis::Y => (y, x).into(),
        }
    }
}
