use std::hash::Hash;
use std::ops::{Add, Deref};

use aoc_2dmap::prelude::*;
use aoc_dijsktra::{dijsktra, GameState, Transform};
use aoc_prelude::*;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
struct State {
    pos: Pos,
    goal: Pos,
}

struct Move {
    to: Pos,
    cost: usize,
}

impl GameState<ArrayVec<Move, 4>, ExtendingMap> for State {
    fn accept(&self) -> bool {
        self.pos == self.goal
    }

    fn steps(&self, map: &ExtendingMap) -> ArrayVec<Move, 4> {
        self.pos
            .neighbors_simple()
            .into_iter()
            .flat_map(|n_pos| {
                Some(Move {
                    to: n_pos,
                    cost: map.get(n_pos)?,
                })
            })
            .collect()
    }
}

impl Transform<State> for Move {
    fn cost(&self) -> usize {
        self.cost
    }

    fn transform(&self, state: &State) -> State {
        State {
            pos: self.to,
            goal: state.goal,
        }
    }
}

#[derive(Clone)]
struct ExtendingMap(Map<usize>);

impl ExtendingMap {
    fn extend_front<P: Fn(usize) -> usize>(&self, along: Axis, by: i32, f: P) -> Self {
        let mut extended = Map::<usize>::fill_default(self.size + along.map(by, 0));
        for pos in self.iter() {
            let tile = self.get(pos).unwrap();
            extended.set(pos + along.map(by, 0), f(tile));
        }
        Self(extended)
    }

    fn tile<P: Fn(usize) -> usize>(self, along: Axis, num: i32, f: P) -> Self {
        let mut uber_tiles = vec![self.clone()];
        let offset = along.map(self.size.x, self.size.y).x;
        for _ in 1..num {
            let cur = uber_tiles.last().unwrap().extend_front(along, offset, &f);
            uber_tiles.push(cur);
        }
        uber_tiles.into_iter().reduce(|x, y| x + y).unwrap()
    }
}

impl Add for ExtendingMap {
    type Output = ExtendingMap;

    // stich together maps by extending rhs
    fn add(self, rhs: Self) -> Self::Output {
        let width = max(self.size.x, rhs.size.x);
        let height = max(self.size.y, rhs.size.y);

        let mut inner = Map::<usize>::fill_default(Pos {
            x: width,
            y: height,
        });

        for x in 0..=width {
            for y in 0..=height {
                let tile_pos = Pos { x, y };
                if let Some(e) = rhs.get(tile_pos) {
                    inner.set(tile_pos, e);
                }
                if let Some(e) = self.get(tile_pos) {
                    inner.set(tile_pos, e);
                }
            }
        }

        Self(inner)
    }
}

impl Deref for ExtendingMap {
    type Target = Map<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn read_input() -> Vec<&'static str> {
    include_str!("../../inputs/day15.txt").lines().collect()
}

fn inc_tile(t: usize) -> usize {
    if t == 0 {
        0
    } else {
        max(1, (t + 1).rem_euclid(10))
    }
}

fn solve(map: &ExtendingMap) -> usize {
    let initial_state = State {
        pos: Pos::default(),
        goal: (map.size + (-1, -1).into()),
    };
    dijsktra(initial_state, map).unwrap()
}

aoc_2021::main! {
    let lines = read_input();

    let tiles: Vec<_> = lines
        .iter()
        .flat_map(|x| {
            x.chars()
                .into_iter()
                .flat_map(|c| c.to_string().parse::<usize>())
        })
        .collect();

    let map = ExtendingMap(Map::<usize>::new(
        Pos {
            x: lines[0].len() as i32,
            y: lines.len() as i32,
        },
        tiles,
    ));
    let p1 = solve(&map);

    let large_map = map.tile(Axis::X, 5, inc_tile).tile(Axis::Y, 5, inc_tile);
    let p2 = solve(&large_map);

    (p1, p2)
}
