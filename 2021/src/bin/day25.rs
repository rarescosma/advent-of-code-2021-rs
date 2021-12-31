use std::str::FromStr;

use aoc_2dmap::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cuke {
    Empty = 0,
    South = 1,
    East = 2,
}

impl Cuke {
    fn is_empty(&self) -> bool {
        matches!(self, Cuke::Empty)
    }

    fn offset(&self) -> Pos {
        match self {
            Self::East => (1, 0).into(),
            Self::South => (0, 1).into(),
            _ => unreachable!(),
        }
    }
}

type CukeMap = Map<Cuke>;

trait CukeSim {
    fn has_neighbor(&self, p: Pos) -> bool;
    fn wrapping_add(&self, p: Pos, offset: Pos) -> Pos;
    fn step_cukes(self, cuke: Cuke) -> (usize, Self);
    fn step(self) -> (usize, Self);
}

impl CukeSim for CukeMap {
    fn has_neighbor(&self, p: Pos) -> bool {
        let cuke = self.get_unchecked(p);
        !self
            .get_unchecked(self.wrapping_add(p, cuke.offset()))
            .is_empty()
    }

    fn wrapping_add(&self, p: Pos, offset: Pos) -> Pos {
        let n = p + offset;
        (n.x % self.size.x, n.y % self.size.y).into()
    }

    fn step_cukes(self, cuke: Cuke) -> (usize, Self) {
        let mut num_moves = 0;
        let mut new_map = self.clone();
        let offset = cuke.offset();
        self.iter()
            .map(|p| (p, self.get_unchecked(p)))
            .into_iter()
            .for_each(|(p, c)| {
                if cuke != c || self.has_neighbor(p) {
                    return;
                }
                new_map.set(self.wrapping_add(p, offset), cuke);
                new_map.set(p, Cuke::Empty);
                num_moves += 1;
            });
        (num_moves, new_map)
    }

    fn step(self) -> (usize, Self) {
        let (e_moves, map) = self.step_cukes(Cuke::East);
        let (s_moves, map) = map.step_cukes(Cuke::South);
        (e_moves + s_moves, map)
    }
}

impl FromStr for Cuke {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "v" => Self::South,
            ">" => Self::East,
            _ => Self::Empty,
        })
    }
}

aoc_2021::main! {
    let lines: Vec<_> = include_str!("../../inputs/day25.txt").lines().collect();

    let mut map = Map::<Cuke>::new(
        (lines[0].len(), lines.len()),
        lines
            .join("")
            .chars()
            .flat_map(|x| x.to_string().parse::<Cuke>()),
    );

    let mut n_steps = 0;

    loop {
        let (n_moves, new_map) = map.step();
        map = new_map;
        n_steps += 1;
        if n_moves == 0 {
            break;
        }
    }

    (n_steps, "🕶️")
}
