use std::fmt::Display;
use std::ops::Deref;

use aoc_2dmap::prelude::*;

const LIGHT: char = '#';
const DARK: char = '.';

#[derive(Clone)]
struct EnhanceMap<T>(Map<T>);

impl<T: Copy + Display> Deref for EnhanceMap<T> {
    type Target = Map<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EnhanceMap<char> {
    fn pad(&mut self, padding: char) {
        let mut new_map = Map::<char>::fill((self.size.x + 2, self.size.y + 2), padding);
        for pos in new_map.to_owned().iter() {
            if pos.x == 0 || pos.y == 0 || pos.x == self.size.x + 1 || pos.y == self.size.y + 1 {
                continue;
            }
            new_map[pos] = self[pos + (-1, -1).into()];
        }
        std::mem::swap(&mut self.0, &mut new_map)
    }

    fn enhance<S: AsRef<str>>(&mut self, algo: S, step: usize) {
        let algo: Vec<char> = algo.as_ref().chars().collect();

        let pad_char = Self::pad_char(step, algo[0]);
        self.pad(pad_char);

        let mut out_map = Map::<char>::fill(self.size, DARK);

        for pos in self.iter() {
            let mut algo_idx = 0_usize;
            for neigh in pos
                .neighbors_diag_inclusive()
                .map(|x| self.get(x).unwrap_or(pad_char))
            {
                algo_idx <<= 1;
                algo_idx += (neigh == LIGHT) as usize
            }
            out_map[pos] = algo[algo_idx];
        }
        std::mem::swap(&mut self.0, &mut out_map)
    }

    fn pad_char(step: usize, alg0: char) -> char {
        if alg0 == DARK || step % 2 == 0 {
            DARK
        } else {
            LIGHT
        }
    }
}

fn pixel_count(map: &EnhanceMap<char>) -> usize {
    map.iter()
        .map(|pos| map[pos])
        .filter(|x| *x == LIGHT)
        .count()
}

aoc_2021::main! {
    let mut lines = include_str!("../../inputs/day20.txt").lines();

    let algo = lines.next().expect("could not find algo");
    let _ = lines.next().expect("no newline");

    let map_lines: Vec<_> = lines.collect();
    let width = map_lines[0].len();
    let height = map_lines.len();

    let mut map = EnhanceMap(Map::new(
        (width, height),
        map_lines.iter().flat_map(|x| x.chars()),
    ));

    for step in 0..=1 {
        map.enhance(algo, step);
    }

    let p1 = pixel_count(&map);

    for step in 2..50 {
        map.enhance(algo, step);
    }

    let p2 = pixel_count(&map);

    (p1, p2)
}
