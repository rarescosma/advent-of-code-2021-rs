use std::hash::Hash;

use aoc_2dmap::prelude::*;
use aoc_dijsktra::{Dijsktra, GameState, Transform};
use aoc_prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Pod(u8),
}

impl Tile {
    fn get_pod(self) -> Option<u8> {
        match self {
            Self::Pod(c) => Some(c),
            _ => None,
        }
    }
}

type State = Map<Tile>;

struct PodContext {
    vis: Vec<(Pos, usize)>,
    seen: HashSet<Pos>,
    q: VecDeque<(Pos, usize)>,
}

impl PodContext {
    fn new() -> Self {
        let vis = Vec::with_capacity(100);
        let seen = HashSet::<Pos>::with_capacity(100);
        let q = VecDeque::with_capacity(100);
        Self { vis, seen, q }
    }

    fn clear(&mut self) {
        self.vis.clear();
        self.seen.clear();
        self.q.clear();
    }
}

impl GameState<PodContext> for State {
    type Steps = ArrayVec<Move, 64>;

    /// true if PodMap is in solved state
    fn accept(&self) -> bool {
        (0..=3).all(|idx| {
            let c = (idx as u8) + b'A';
            if let Some(column) = self.get_col(room(c)) {
                return column[1..].iter().all(|&t| t == Tile::Pod(c));
            }
            false
        })
    }

    /// Get all possible moves for a map
    fn steps(&self, ctx: &mut PodContext) -> ArrayVec<Move, 64> {
        self.iter()
            .filter(|x| x.is_pod(self))
            .flat_map(|from| {
                let step_cost = [1, 10, 100, 1000][(from.get_byte(self) - b'A') as usize];
                visible(self, from, ctx)
                    .into_iter()
                    .map(move |(to, steps)| Move {
                        from,
                        to,
                        cost: step_cost * steps,
                    })
            })
            .filter(|mv| mv.is_valid(self))
            .collect()
    }
}

struct Move {
    from: Pos,
    to: Pos,
    cost: usize,
}

impl Transform<State> for Move {
    fn cost(&self) -> usize {
        self.cost
    }

    fn transform(&self, state: &State) -> State {
        let mut new_map = (*state).clone();
        new_map.swap(self.from, self.to);
        new_map
    }
}

impl Move {
    #[inline(always)]
    fn is_valid(&self, m: &State) -> bool {
        if self.to.is_entrance() {
            return false;
        }

        let c = self.from.get_byte(m);

        if self.from.is_room() && self.to.is_hallway() {
            // can only move out of our own room if any of the underlings are wrong
            if self.from.x == room(c) {
                if let Some(column) = m.get_col(self.from.x) {
                    return ((self.from.y as _)..column.len()).any(|y| column[y] != Tile::Pod(c));
                }
            }
            return true;
        }

        if self.from.is_room() && self.to.is_room() && self.from.x != self.to.x {
            return self.is_room_valid(c, m);
        }

        if self.from.is_hallway() && self.to.is_room() {
            return self.is_room_valid(c, m);
        }

        false
    }

    fn is_room_valid(&self, c: u8, m: &State) -> bool {
        if self.to.x != room(c) {
            return false;
        }

        if let Some(column) = m.get_col(self.to.x) {
            return if (self.to.y as usize) < column.len() - 1 {
                // trying to move into non-empty room, check for aliens
                ((self.to.y + 1) as usize..column.len()).all(|y| column[y] == Tile::Pod(c))
            } else {
                // trying to move to bottom of room, check if empty
                *column.last().unwrap() == Tile::Empty
            };
        }
        false
    }
}

trait PodPos {
    fn is_empty(&self, m: &State) -> bool;
    fn is_pod(&self, m: &State) -> bool;
    fn get_byte(&self, m: &State) -> u8;
    fn is_hallway(&self) -> bool;
    fn is_room(&self) -> bool;
    fn is_entrance(&self) -> bool;
}

impl PodPos for Pos {
    fn is_empty(&self, m: &State) -> bool {
        matches!(m.get(self), Some(Tile::Empty))
    }

    fn is_pod(&self, m: &State) -> bool {
        matches!(m.get(self), Some(Tile::Pod(_)))
    }

    fn get_byte(&self, m: &State) -> u8 {
        m.get(self).and_then(Tile::get_pod).unwrap()
    }

    fn is_hallway(&self) -> bool {
        self.y == 0
    }

    fn is_room(&self) -> bool {
        self.y >= 1 && [2, 4, 6, 8].contains(&self.x)
    }

    fn is_entrance(&self) -> bool {
        self.is_hallway() && [2, 4, 6, 8].contains(&self.x)
    }
}

/// Return the room index (column) for the given pod byte
#[inline(always)]
fn room(pod: u8) -> i32 {
    [2, 4, 6, 8][(pod - b'A') as usize]
}

/// Generate all visible positions from the starting position
fn visible(m: &State, start_pos: Pos, ctx: &mut PodContext) -> impl Iterator<Item = (Pos, usize)> {
    ctx.clear();

    ctx.q.push_back((start_pos, 0));

    while let Some((pos, steps)) = ctx.q.pop_back() {
        for neigh in pos.neighbors_simple() {
            if neigh.is_empty(m) && !ctx.seen.contains(&neigh) {
                ctx.vis.push((neigh, steps + 1));
                ctx.q.push_back((neigh, steps + 1));
                ctx.seen.insert(neigh);
            }
        }
    }
    ctx.vis.to_owned().into_iter()
}

fn solve(input: Vec<&str>) -> usize {
    let tiles: Vec<Tile> = input
        .iter()
        .flat_map(|l| l.bytes().map(Tile::from))
        .collect();

    let map = Map::<Tile>::new((11, input.len() - 1).into(), tiles);

    map.dijsktra(&mut PodContext::new()).unwrap()
}

impl From<u8> for Tile {
    fn from(c: u8) -> Self {
        match c {
            b'.' => Tile::Empty,
            x if "ABCD".contains(x as char) => Tile::Pod(x),
            _ => Tile::Wall,
        }
    }
}

aoc_2021::main! {
    let part1: Vec<_> = include_str!("../../inputs/day23-p1.txt").lines().collect();
    let part2: Vec<_> = include_str!("../../inputs/day23-p2.txt").lines().collect();

    (solve(part1), solve(part2))
}
