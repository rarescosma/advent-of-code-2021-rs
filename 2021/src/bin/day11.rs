use std::fmt::{Debug, Formatter};

use aoc_2dmap::prelude::{Adjacency, Map, Pos};
use aoc_prelude::prelude::*;

const MAX_ENERGY: u16 = 9;

fn read_input() -> Vec<&'static str> {
    include_str!("../../inputs/day11.txt").lines().collect()
}

#[derive(Default, Copy, Clone)]
struct Octo(u16);

impl From<char> for Octo {
    fn from(c: char) -> Self {
        Self(c.to_string().parse::<u16>().unwrap())
    }
}

impl Debug for Octo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)?;
        Ok(())
    }
}

impl Flashing for Octo {
    fn is_flashing(&self) -> bool {
        self.0 > MAX_ENERGY
    }

    fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}

trait Flashing {
    fn is_flashing(&self) -> bool;
    fn increment(self) -> Self;
}

struct OctoMap(Map<Octo>);

impl OctoMap {
    fn flashing(&self) -> Vec<Pos> {
        self.0
            .iter()
            .filter(|&p| matches!(self.0.get(p), Some(x) if x.is_flashing()))
            .collect()
    }

    // return number of flashes
    fn step(&mut self) -> usize {
        let positions: Vec<_> = self.0.iter().collect();
        for pos in positions {
            if let Some(tile) = self.0.get(pos) {
                self.0.set(pos, tile.increment());
            }
        }

        let mut flashing: VecDeque<_> = VecDeque::from(self.flashing());
        let mut flashed: HashSet<_> = HashSet::new();

        while !flashing.is_empty() {
            let p = flashing.pop_front().unwrap();
            flashing.extend(self.flash(p, &mut flashed));
        }

        self.flashing()
            .iter()
            .map(|&pos| self.0.set(pos, Octo::default()))
            .count()
    }

    // return positions of cascading flashes
    fn flash(&mut self, pos: Pos, flashed: &mut HashSet<Pos>) -> Vec<Pos> {
        // if already flashed => noop, otherwise => mark as flashed
        if flashed.contains(&pos) {
            return Vec::default();
        } else {
            flashed.insert(pos);
        }

        let mut cascade = Vec::new();

        // increase energy of all neighboring tiles
        for n_pos in pos.neighbors(Adjacency::Diagonal) {
            if let Some(neighbor) = self.0.get(n_pos) {
                let neighbor = neighbor.increment();
                self.0.set(n_pos, neighbor);

                // if a new neighbor wants to flash (and hasn't already)
                if neighbor.is_flashing() && !flashed.contains(&n_pos) {
                    cascade.push(n_pos);
                }
            }
        }
        cascade
    }
}

aoc_2021::main! {
    let lines = read_input();

    let octos: Vec<Octo> = lines.concat().chars().map(|x| x.into()).collect();

    let octo_count = octos.len();

    let width = lines[0].len();
    let height = lines.len();

    let mut map = OctoMap(Map::new((width, height).into(), octos));

    let mut p1 = 0;
    let mut total_count = 0;
    let mut p2 = 0;
    for x in 1.. {
        let flash_count = map.step();
        total_count += flash_count;
        if x == 100 {
            p1 = total_count;
        }
        if flash_count == octo_count {
            p2 = x;
            break;
        }
    }

    (p1, p2)
}
