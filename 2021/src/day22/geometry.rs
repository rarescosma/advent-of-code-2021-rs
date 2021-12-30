use aoc_prelude::*;

use std::fmt::{Debug, Formatter};

lazy_static! {
    pub static ref PLANE_COMBOS: [[Dim; 3]; 6] = {
        let v: Vec<[Dim; 3]> = [Dim::X, Dim::Y, Dim::Z]
            .into_iter()
            .permutations(3)
            .flat_map(|p| p.try_into())
            .collect();
        v.try_into().unwrap()
    };
}

pub trait Contains<O> {
    fn contains(&self, other: O) -> bool;
}

pub trait ProjectTo<T> {
    type Output;

    fn project_to(&self, to: T) -> Self::Output;
}

#[derive(Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Dim {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl From<[i64; 3]> for Point {
    fn from(t: [i64; 3]) -> Self {
        Self {
            x: t[0],
            y: t[1],
            z: t[2],
        }
    }
}

impl Point {
    pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
        [self.x, self.y, self.z].into_iter()
    }

    pub fn get(&self, dim: Dim) -> i64 {
        match dim {
            Dim::X => self.x,
            Dim::Y => self.y,
            Dim::Z => self.z,
        }
    }

    pub fn as_plane(&self, dim: Dim) -> Plane {
        Plane {
            dim,
            pos: self.get(dim),
        }
    }
}

impl ProjectTo<Plane> for Point {
    type Output = Point;

    fn project_to(&self, plane: Plane) -> Self::Output {
        let mut coords = [self.x, self.y, self.z];
        coords[plane.dim as usize] = plane.pos;
        coords.into()
    }
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub dim: Dim,
    pub pos: i64,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.dim == other.dim
    }
}

impl Debug for Plane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.dim {
            Dim::X => write!(f, "x: {}", self.pos),
            Dim::Y => write!(f, "y: {}", self.pos),
            Dim::Z => write!(f, "z: {}", self.pos),
        }
    }
}
