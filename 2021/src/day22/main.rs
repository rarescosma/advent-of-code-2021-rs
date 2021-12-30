mod geometry;
mod parse;
mod tests;

use crate::geometry::*;
use aoc_prelude::*;

use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, BitAnd, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cube {
    o: Point,
    l: Point,
}

impl Cube {
    fn intersects(&self, other: &Cube) -> bool {
        let common = self & other;
        self.contains(&common) || other.contains(&common)
    }

    fn planes(&self) -> impl Iterator<Item = ArrayVec<Plane, 3>> {
        iproduct!(
            PLANE_COMBOS.iter(),
            iproduct!([self.o, self.l], [self.o, self.l], [self.o, self.l])
        )
        .map(|(dims, (p0, p1, p2))| {
            dims.iter()
                .zip([p0, p1, p2])
                .map(|(&dim, point)| point.as_plane(dim))
                .collect()
        })
    }

    fn subtract_from(&self, bigger: &Cube) -> HashSet<Cube> {
        let mut neighs = HashSet::new();

        // take the common cuboid and move it towards the planes of the cuboid
        // we're subtracting from
        let common = self & bigger;

        bigger.planes().into_iter().for_each(|planes| {
            // we get a sequence of 3 different planes here
            // do moves in turn and record all partial results along
            // with the result of the final translation
            let mut _common = common;
            for plane in planes {
                if let Some(next) = _common.project_to(plane) {
                    neighs.insert(next);
                    _common = next;
                }
            }
        });
        neighs
    }

    fn volume(&self) -> usize {
        self.l
            .iter()
            .zip(self.o.iter())
            .map(|(l, o)| (l + 1 - o) as usize)
            .product()
    }
}

impl ProjectTo<Plane> for Cube {
    type Output = Option<Cube>;

    fn project_to(&self, plane: Plane) -> Self::Output {
        let bound_l = self.l.get(plane.dim);
        let bound_o = self.o.get(plane.dim);

        if plane.pos > bound_l {
            // case 1
            let moz = self.o.project_to(Plane {
                dim: plane.dim,
                pos: bound_l + 1,
            });

            let mlz = self.l.project_to(plane);

            return Some((moz, mlz).into());
        } else if plane.pos < bound_o {
            // case 2
            let moz = self.o.project_to(plane);

            let mlz = self.l.project_to(Plane {
                dim: plane.dim,
                pos: bound_o - 1,
            });

            return Some((moz, mlz).into());
        }
        None
    }
}

impl Contains<&Point> for Cube {
    fn contains(&self, point: &Point) -> bool {
        self.o.iter().zip(point.iter()).all(|(o, p)| o <= p)
            && self.l.iter().zip(point.iter()).all(|(l, p)| l >= p)
    }
}

impl Contains<&Cube> for Cube {
    fn contains(&self, cube: &Cube) -> bool {
        self.contains(&cube.o) && self.contains(&cube.l)
    }
}

// for the "attemptive" intersection of cubes:
// if terms have no points in common then the result of BitAnd
// will be a cuboid that sits "outside" both operands
impl BitAnd for &Cube {
    type Output = Cube;

    fn bitand(self, rhs: Self) -> Self::Output {
        (
            [
                max(self.o.x, rhs.o.x),
                max(self.o.y, rhs.o.y),
                max(self.o.z, rhs.o.z),
            ],
            [
                min(self.l.x, rhs.l.x),
                min(self.l.y, rhs.l.y),
                min(self.l.z, rhs.l.z),
            ],
        )
            .into()
    }
}

impl Add for Cube {
    type Output = HashSet<Cube>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.contains(&rhs) {
            return [self].iter().cloned().collect();
        } else if rhs.contains(&self) {
            return [rhs].iter().cloned().collect();
        } else if !self.intersects(&rhs) {
            return [self, rhs].iter().cloned().collect();
        }

        let mut r_diff = rhs.subtract_from(&self);

        r_diff.insert(rhs);
        r_diff
    }
}

impl Sub for Cube {
    type Output = HashSet<Cube>;

    fn sub(self, inner: Self) -> Self::Output {
        if inner.contains(&self) {
            return HashSet::new();
        }
        if !self.intersects(&inner) {
            return [self].iter().cloned().collect();
        }
        inner.subtract_from(&self)
    }
}

fn volume<C: IntoIterator<Item = Cube>>(cubes: C) -> usize {
    cubes.into_iter().map(|c| c.volume()).sum()
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "o: {:?} | l: {:?} | vol: {}",
            self.o,
            self.l,
            self.volume()
        )
    }
}

impl<P: Into<Point>, Q: Into<Point>> From<(P, Q)> for Cube {
    fn from(pts: (P, Q)) -> Self {
        let a = pts.0.into();
        let b = pts.1.into();
        Cube {
            o: [min(a.x, b.x), min(a.y, b.y), min(a.z, b.z)].into(),
            l: [max(a.x, b.x), max(a.y, b.y), max(a.z, b.z)].into(),
        }
    }
}

fn process_cubes<F: Fn(Cube) -> bool>(cubes: Vec<(Cube, String)>, accept: F) -> usize {
    let mut proc = HashSet::new();
    for (cube, cmd) in cubes.into_iter().filter(|(c, _)| accept(*c)) {
        if proc.is_empty() && cmd == "on" {
            proc.insert(cube);
        } else {
            proc = proc
                .into_iter()
                .flat_map(|x| if cmd == "on" { x + cube } else { x - cube })
                .collect();
        }
    }
    volume(proc)
}

aoc_2021::main! {
    let cubes: Vec<_> = include_str!("../../inputs/day22.txt")
        .lines()
        .map(parse::process_line)
        .collect();

    // Part 1
    let world: Cube = ([-50, -50, -50], [50, 50, 50]).into();
    let p1 = process_cubes(cubes.to_owned(), |cube| cube.intersects(&world));

    // Part 2
    let p2 = process_cubes(cubes, |_| true);

    (p1, p2)
}
