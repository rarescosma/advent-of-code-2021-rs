use aoc_2dmap::prelude::*;
use aoc_prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day13-folds.pest"]
pub struct FoldsParser;

#[derive(Clone)]
struct FoldingMap<T>(Map<T>);

struct Fold {
    axis: Axis,
    at: i32,
}

impl<T: Copy + Default> FoldingMap<T> {
    fn flip(self, along: Axis) -> Self {
        let mut flipped = Map::<T>::fill_default(self.0.size);
        for pos in self.0.iter() {
            let flip_pos = match along {
                Axis::X => Pos {
                    x: self.0.size.x - pos.x - 1,
                    y: pos.y,
                },
                Axis::Y => Pos {
                    x: pos.x,
                    y: self.0.size.y - pos.y - 1,
                },
            };
            flipped.set(flip_pos, self.0.get_unchecked(pos));
        }

        FoldingMap(flipped)
    }

    fn partition(&self, along: Axis, at: i32) -> (Self, Self) {
        let size = along.map(self.0.size.x, self.0.size.y);
        let mut left = Map::<T>::fill_default(along.map(at, size.y));
        let mut right = Map::<T>::fill_default(self.0.size + along.map(-at - 1, 0));
        for outer_coord in 0..size.x {
            for inner_coord in 0..size.y {
                let tile = self.0.get_unchecked(along.map(outer_coord, inner_coord));

                if outer_coord < at {
                    left.set(along.map(outer_coord, inner_coord), tile);
                } else {
                    // -1 because we eat the fold line
                    right.set(along.map(outer_coord - at - 1, inner_coord), tile);
                }
            }
        }
        (Self(left), Self(right))
    }

    fn extend_front(self, along: Axis, by: i32) -> Self {
        let mut extended = Map::<T>::fill_default(self.0.size + along.map(by, 0));
        for pos in self.0.iter() {
            let tile = self.0.get_unchecked(pos);
            extended.set(pos + along.map(by, 0), tile);
        }
        Self(extended)
    }

    fn fold<P>(&mut self, along: Axis, at: i32, f: P)
    where
        P: Fn(T, T) -> T,
    {
        let (mut left, mut right) = self.partition(along, at);

        let right_outer_size = along.map(right.0.size.x, right.0.size.y).x;
        let left_outer_size = along.map(left.0.size.x, left.0.size.y).x;

        // always flip 2nd half, extend as necessary
        right = right.flip(along);
        if right_outer_size < left_outer_size {
            right = right.extend_front(along, left_outer_size - right_outer_size);
        } else {
            left = left.extend_front(along, right_outer_size - left_outer_size)
        }

        let mut folded = Map::<T>::fill_default(right.0.size);

        for pos in right.0.iter() {
            folded.set(
                pos,
                f(right.0.get_unchecked(pos), left.0.get_unchecked(pos)),
            );
        }
        self.0 = folded;
    }
}

fn read_input() -> String {
    include_str!("../../inputs/day13.txt").to_string()
}

fn char_sum(a: char, b: char) -> char {
    if a == '#' || b == '#' {
        '#'
    } else {
        ' '
    }
}

aoc_2021::main! {
    let input = read_input();

    let parsed = FoldsParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap();

    let mut points = Vec::<Pos>::new();
    let mut folds = Vec::<Fold>::new();
    for rule in parsed.into_inner() {
        match rule.as_rule() {
            Rule::point => {
                let coords: Vec<_> = rule.as_str().split(',').collect();
                points.push(Pos {
                    x: coords[0].parse().unwrap(),
                    y: coords[1].parse().unwrap(),
                });
            }
            Rule::fold => {
                let fold: Vec<_> = rule.as_str().split('=').collect();
                folds.push(Fold {
                    axis: {
                        match fold[0] {
                            "x" => Axis::X,
                            "y" => Axis::Y,
                            _ => unreachable!(),
                        }
                    },
                    at: fold[1].parse().unwrap(),
                })
            }
            _ => (),
        }
    }
    let max_x = points.iter().map(|p| p.x).reduce(max).unwrap() + 1;
    let max_y = points.iter().map(|p| p.y).reduce(max).unwrap() + 1;

    let mut map = FoldingMap(Map::<char>::new(
        MapSize { x: max_x, y: max_y },
        vec![' '; max_x as usize * max_y as usize],
    ));

    for point in points {
        map.0.set(point, '#');
    }

    let mut folds = folds.into_iter();
    let first_fold = folds.next().unwrap();
    map.fold(first_fold.axis, first_fold.at, char_sum);

    let p1 = map
        .0
        .iter()
        .flat_map(|p| map.0.get(p))
        .filter(|x| *x == '#')
        .count();

    folds.for_each(|fold| {
        map.fold(fold.axis, fold.at, char_sum);
    });

    (p1, format!("\n{}", &map.0))
}
