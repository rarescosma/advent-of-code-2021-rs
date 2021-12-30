use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

use aoc_prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day19-scanners.pest"]
pub struct ScannerParser;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
    origin: bool,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i16(self.x);
        state.write_i16(self.y);
        state.write_i16(self.z);
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let origin = self.origin || rhs.origin;
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, origin)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let origin = self.origin || rhs.origin;
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, origin)
    }
}

impl From<ArrayVec<i16, 3>> for Point {
    fn from(v: ArrayVec<i16, 3>) -> Self {
        Self::new(v[0], v[1], v[2], false)
    }
}

impl From<([i16; 3], bool)> for Point {
    fn from((s, o): ([i16; 3], bool)) -> Self {
        Self::new(s[0], s[1], s[2], o)
    }
}

impl Point {
    fn new(x: i16, y: i16, z: i16, origin: bool) -> Self {
        Self { x, y, z, origin }
    }

    fn permute(&self, permute: &Permute) -> Point {
        self.rotate(permute.first_rota).rotate(permute.second_rota)
    }

    fn rotate(&self, rota: [i16; 3]) -> Point {
        let pos: [i16; 3] = [self.x, self.y, self.z];
        let mut new_pos: [i16; 3] = [0, 0, 0];
        for (old_idx, r) in rota.iter().enumerate() {
            let signum = r.signum();
            let idx = (r.abs() - 1) as usize;
            new_pos[old_idx] = pos[idx] * signum;
        }
        (new_pos, self.origin).into()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Scanner {
    points: Vec<Point>,
}

type DiffFreq = HashMap<Point, usize>;

impl Scanner {
    fn permute(&self, permute: &Permute) -> Scanner {
        let points: Vec<_> = self.points.iter().map(|p| p.permute(permute)).collect();
        Scanner { points }
    }

    fn find_permute(
        &self,
        other: &Scanner,
        permutes: Vec<Permute>,
        diff_freq: &mut DiffFreq,
    ) -> Option<(Permute, Point)> {
        for (idx, permute) in permutes.iter().enumerate() {
            diff_freq.clear();
            if let Some(offset) = self.find_offset(other.permute(permute), diff_freq) {
                return Some((permutes[idx], offset));
            }
        }
        None
    }

    fn find_offset(&self, other: Scanner, diff_freq: &mut DiffFreq) -> Option<Point> {
        for x in iproduct!(&self.points, other.points).map(|(x, y)| *x - y) {
            let new_freq = diff_freq.entry(x).or_insert(0);
            *new_freq += 1;
            if *new_freq == 12 {
                return Some(x);
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone)]
struct Permute {
    first_rota: [i16; 3],
    second_rota: [i16; 3],
}

fn manhattan(p: Point, q: Point) -> usize {
    let m = p - q;
    (m.x.abs() + m.y.abs() + m.z.abs()) as usize
}

fn collapse(
    scanners: &mut BTreeMap<usize, Scanner>,
    permutes: &[Permute],
    diff_freq: &mut DiffFreq,
) {
    let mut tree: Vec<_> = Vec::with_capacity(32);

    for (k0, k1) in scanners.keys().rev().tuple_combinations() {
        if let Some((permute, offset)) =
            scanners[k0].find_permute(&scanners[k1], permutes.into(), diff_freq)
        {
            tree.push((*k0, *k1, permute, offset));
        }
    }

    let from: BTreeSet<_> = tree.iter().map(|x| x.0).collect();
    let to: BTreeSet<_> = tree.iter().map(|x| x.1).collect();
    for leaf_idx in to.difference(&from) {
        let tpl = tree.iter().find(|x| x.1 == *leaf_idx).unwrap();
        let folded = fold(&scanners[&tpl.0], &scanners[&tpl.1], &tpl.2, tpl.3);
        scanners.remove(&tpl.1);
        scanners.entry(tpl.0).and_modify(|s| *s = folded);
    }
}

fn fold(s0: &Scanner, s1: &Scanner, p: &Permute, o: Point) -> Scanner {
    let mut points: BTreeSet<_> = s1.points.iter().map(|x| x.permute(p) + o).collect();
    points.extend(s0.points.iter());
    Scanner {
        points: points.into_iter().collect(),
    }
}

fn total_points(scanners: &BTreeMap<usize, Scanner>) -> usize {
    scanners.values().map(|s| s.points.len()).sum()
}

fn permutes() -> Vec<Permute> {
    let x_y_rot = vec![
        [1, 2, 3],
        [-3, 2, 1],
        [-1, 2, -3],
        [3, 2, -1],
        [1, 2, -3],
        [1, -2, 3],
    ];
    let z_rot = vec![[1, 2, 3], [-2, 1, 3], [-1, -2, 3], [2, -1, 3]];

    iproduct!(x_y_rot, z_rot)
        .map(|(first_rota, second_rota)| Permute {
            first_rota,
            second_rota,
        })
        .collect()
}

fn scanners() -> BTreeMap<usize, Scanner> {
    let lines = include_str!("../../inputs/day19.txt").lines();
    let mut i: usize = 0;

    let mut scan_points = Vec::<(usize, Point)>::new();

    for line in lines {
        if let Ok(mut parsed) = ScannerParser::parse(Rule::line, line) {
            let parse_result = parsed.next().unwrap();
            match parse_result.as_rule() {
                Rule::scanner => {
                    i += 1;
                }
                Rule::point => {
                    let point: ArrayVec<i16, 3> = parse_result
                        .into_inner()
                        .flat_map(|x| x.as_str().parse::<i16>())
                        .collect();
                    scan_points.push((i, point.into()));
                }
                _ => (),
            }
        }
    }

    let mut scanners = BTreeMap::<usize, Scanner>::new();
    for (idx, pgroup) in &scan_points.into_iter().group_by(|(x, _)| *x) {
        let mut points: Vec<_> = pgroup.map(|(_, p)| p).collect();

        /*
        stupidly sneaky way of finding a scanner's position:
        just add (0,0,0) within its own list and track it!
         */
        points.push(Point::new(0, 0, 0, true));
        scanners.insert(idx, Scanner { points });
    }
    scanners
}

aoc_2021::main! {
    let permutes = permutes();
    let mut scanners = scanners();
    let mut diff_freq = DiffFreq::with_capacity(2^12);

    // this is brutal, we totally don't need to do multiple collapses
    // since we'd have found all possible offset points from the
    // first pass
    while scanners.len() > 1 {
        collapse(&mut scanners, &permutes, &mut diff_freq);
    }

    let total = total_points(&scanners);

    let last = scanners.values().last().unwrap();
    let origins: Vec<_> = last.points.iter().filter(|p| p.origin).collect();

    // Part 1
    let p1 = total - origins.len();

    // Part 2
    let mut m_d = 0;
    for (p, q) in origins.into_iter().tuple_combinations() {
        m_d = max(m_d, manhattan(*p, *q));
    }

    (p1, m_d)
}
