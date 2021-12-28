use num::{range_inclusive, PrimInt, Signed};
use pest::Parser;
use pest_derive::Parser;
use std::fmt::Debug;

#[derive(Parser)]
#[grammar = "parsers/day05-line.pest"]
pub struct LineParser;

#[derive(Debug, Clone, Copy)]
struct Point<I> {
    x: I,
    y: I,
}

#[derive(Debug)]
struct Line<I> {
    from: Point<I>,
    to: Point<I>,
}

struct LineDir<I> {
    dx: I,
    dy: I,
}

impl<I> FromIterator<I> for Line<I> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let from = Point {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        };
        let to = Point {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        };
        Self { from, to }
    }
}

impl<I: PrimInt + Signed> Line<I> {
    fn dir(&self) -> LineDir<I> {
        LineDir {
            dx: (self.to.x - self.from.x).signum(),
            dy: (self.to.y - self.from.y).signum(),
        }
    }

    #[allow(dead_code)]
    fn is_straight(&self) -> bool {
        let dir = self.dir();
        dir.dx == I::zero() || dir.dy == I::zero()
    }

    fn length(&self) -> I {
        I::max(
            (self.to.x - self.from.x).abs(),
            (self.to.y - self.from.y).abs(),
        )
    }

    fn points(&self, skip_diag: bool) -> Vec<Point<I>> {
        let dir = self.dir();
        if dir.dx != I::zero() && dir.dy != I::zero() && skip_diag {
            return vec![];
        }
        let point_indices = range_inclusive(I::zero(), self.length());

        point_indices
            .map(|index| Point {
                x: self.from.x + index * dir.dx,
                y: self.from.y + index * dir.dy,
            })
            .collect()
    }
}

const MAP_SIZE: usize = 1000;

struct Map(Vec<usize>);

impl Default for Map {
    fn default() -> Self {
        Map(vec![0; MAP_SIZE * MAP_SIZE])
    }
}

impl Map {
    fn draw_line<I: PrimInt + Signed>(&mut self, line: &Line<I>, skip_diag: bool) {
        for point in line.points(skip_diag) {
            self.add_point(&point);
        }
    }

    fn add_point<I: PrimInt + Signed>(&mut self, point: &Point<I>) {
        let pos: usize = point.y.to_usize().unwrap() * MAP_SIZE + point.x.to_usize().unwrap();
        self.0[pos] += 1;
    }

    fn num_overlap(&self) -> usize {
        let point_counts = self.0.iter();
        point_counts.filter(|&&x| x >= 2).count()
    }
}

aoc2021::main! {
    let input = include_str!("../../inputs/day05.txt").to_string();

    let line_parse = LineParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap();

    let mut p1_map = Map::default();
    let mut p2_map = Map::default();

    line_parse
        .into_inner()
        .filter(|outer| outer.as_rule() == Rule::line)
        .map(|outer| {
            outer
                .into_inner()
                .filter(|inner| inner.as_rule() == Rule::number)
                .flat_map(|inner| inner.as_str().parse::<i16>())
                .collect()
        })
        .for_each(|line| {
            p1_map.draw_line(&line, true);
            p2_map.draw_line(&line, false);
        });

    (p1_map.num_overlap(), p2_map.num_overlap())
}
