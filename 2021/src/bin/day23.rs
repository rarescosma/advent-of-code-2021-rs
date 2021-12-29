use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use aoc_2dmap::prelude::*;
use aoc_prelude::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Empty,
    Wall,
    Pod(u8),
}

type PodMap = Map<Tile>;

impl Tile {
    fn get_pod(self) -> Option<u8> {
        match self {
            Self::Pod(c) => Some(c),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Move {
    from: Pos,
    to: Pos,
    cost: usize,
}

impl Move {
    fn apply(&self, map: &PodMap) -> PodMap {
        let mut new_map = map.clone();
        new_map.swap(self.from, self.to);
        new_map
    }

    #[inline(always)]
    fn is_valid(&self, m: &PodMap) -> bool {
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

    fn is_room_valid(&self, c: u8, m: &PodMap) -> bool {
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
    fn is_empty(&self, m: &PodMap) -> bool;
    fn is_pod(&self, m: &PodMap) -> bool;
    fn get_byte(&self, m: &PodMap) -> u8;
    fn is_hallway(&self) -> bool;
    fn is_room(&self) -> bool;
    fn is_entrance(&self) -> bool;
}

/// Return the room index (column) for the given byte (pod)
#[inline(always)]
fn room(pod: u8) -> i32 {
    [2, 4, 6, 8][(pod - b'A') as usize]
}

/// Generate all visible positions from the starting position
fn visible<'a>(
    seen: &'a RefCell<HashSet<Pos>>,
    m: &'a PodMap,
    (start_pos, steps): (Pos, usize),
) -> Box<dyn Iterator<Item = (Pos, usize)> + 'a> {
    seen.borrow_mut().insert(start_pos);

    let neighs: [Pos; 4] = [
        (start_pos.x + 1, start_pos.y).into(),
        (start_pos.x - 1, start_pos.y).into(),
        (start_pos.x, start_pos.y + 1).into(),
        (start_pos.x, start_pos.y - 1).into(),
    ];

    let base = neighs
        .into_iter()
        .filter(|p| p.is_empty(m) && !seen.borrow().contains(p))
        .map(move |p| (p, steps + 1));

    let base_tee = base.clone();
    let base = base.chain(base_tee.flat_map(|tpl| visible(seen, m, tpl)));
    Box::new(base)
}

/// Get all possible moves for a map
fn moves(m: &PodMap) -> ArrayVec<Move, 32> {
    m.iter()
        .filter(|x| x.is_pod(m))
        .flat_map(|from| {
            let step_cost = [1, 10, 100, 1000][(from.get_byte(m) - b'A') as usize];
            visible(&RefCell::new(HashSet::new()), m, (from, 0))
                .map(|(to, steps)| Move {
                    from,
                    to,
                    cost: step_cost * steps,
                })
                .collect::<ArrayVec<Move, 32>>()
        })
        .filter(|mv| mv.is_valid(m))
        .collect()
}

/// true if Map is in solved state
fn is_solved(m: &PodMap) -> bool {
    (0..=3).all(|idx| {
        let c = (idx as u8) + b'A';
        if let Some(column) = m.get_col(room(c)) {
            return column[1..].iter().all(|t| *t == Tile::Pod(c));
        }
        false
    })
}

/// compute the shortest path through the cost graph
fn dijsktra_pod(state: PodMap) -> i64 {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push((0, state));
    while let Some((cost, map)) = heap.pop() {
        if is_solved(&map) {
            return -cost;
        }
        if let Some(&c) = dist.get(&map) {
            if -cost > c {
                continue;
            }
        }
        for m in moves(&map) {
            let new_map = m.apply(&map);
            let next_cost = -cost + m.cost as i64;
            let &prev_cost = dist.get(&new_map).unwrap_or(&i64::MAX);
            if prev_cost > next_cost {
                dist.insert(new_map.clone(), next_cost);
                heap.push((-next_cost, new_map));
            }
        }
    }
    unreachable!()
}

fn solve(input: Vec<&str>) -> i64 {
    let tiles: Vec<Tile> = input
        .iter()
        .flat_map(|l| l.bytes().map(Tile::from))
        .collect();

    let map = Map::<Tile>::new((11, input.len() - 1).into(), tiles);

    dijsktra_pod(map)
}

impl PodPos for Pos {
    fn is_empty(&self, m: &PodMap) -> bool {
        m.get(*self).unwrap_or(Tile::Wall) == Tile::Empty
    }

    fn is_pod(&self, m: &PodMap) -> bool {
        matches!(m.get(*self).unwrap_or(Tile::Wall), Tile::Pod(_))
    }

    fn get_byte(&self, m: &PodMap) -> u8 {
        m.get(*self).and_then(Tile::get_pod).unwrap()
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

impl From<u8> for Tile {
    fn from(c: u8) -> Self {
        match c {
            b'.' => Tile::Empty,
            x if "ABCD".contains(x as char) => Tile::Pod(x),
            _ => Tile::Wall,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Wall => '#',
                Self::Pod(c) => *c as _,
            }
        )
    }
}

aoc_2021::main! {
    let part1: Vec<_> = include_str!("../../inputs/day23-p1.txt").lines().collect();
    let part2: Vec<_> = include_str!("../../inputs/day23-p2.txt").lines().collect();

    (solve(part1), solve(part2))
}
