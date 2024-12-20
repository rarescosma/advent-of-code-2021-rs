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
    fn wrapping_add(&self, p: Pos, offset: Pos) -> Pos;
    fn step_cukes(&mut self, cuke: Cuke, buf: &mut Vec<Cuke>) -> usize;
    fn step(&mut self, buf: &mut Vec<Cuke>) -> usize;
}

impl CukeSim for CukeMap {
    fn wrapping_add(&self, p: Pos, offset: Pos) -> Pos {
        let mut n = p + offset;
        if n.x == self.size.x {
            n.x = 0;
        }
        if n.y == self.size.y {
            n.y = 0;
        }
        n
    }

    fn step_cukes(&mut self, cuke: Cuke, buf: &mut Vec<Cuke>) -> usize {
        let mut num_moves = 0;
        let offset = cuke.offset();

        buf.clone_from(self.get_tiles());

        for (idx, pos) in self.iter().enumerate() {
            let c = self[pos];
            if c == cuke {
                let n_pos = self.wrapping_add(pos, offset);
                if self[n_pos].is_empty() {
                    buf.swap(idx, (n_pos.x + n_pos.y * self.size.x) as usize);
                    num_moves += 1;
                }
            }
        }

        self.swap_vec(buf);
        num_moves
    }

    fn step(&mut self, buf: &mut Vec<Cuke>) -> usize {
        let e_moves = self.step_cukes(Cuke::East, buf);
        let s_moves = self.step_cukes(Cuke::South, buf);
        e_moves + s_moves
    }
}

impl FromStr for Cuke {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ">" => Self::East,
            "v" => Self::South,
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
    let mut buf = Vec::<Cuke>::with_capacity((map.size.x * map.size.y) as usize);

    loop {
        let n_moves = map.step(&mut buf);
        n_steps += 1;
        if n_moves == 0 {
            break;
        }
    }

    (n_steps, "🕶️")
}
