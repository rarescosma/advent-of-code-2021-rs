use itertools::{max, Itertools};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, BitAnd};
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "parsers/day18-snails.pest"]
pub struct SnailParser;

#[derive(Debug, Clone)]
enum BTree<T> {
    Leaf(T),
    Branch {
        left: Box<BTree<T>>,
        right: Box<BTree<T>>,
    },
}

impl<T> BitAnd for BTree<T> {
    type Output = BTree<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        BTree::Branch {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl<T> BTree<T> {
    fn fold<F: Fn(T, T) -> T + Copy>(self, op: F) -> T {
        match self {
            Self::Leaf(t) => t,
            Self::Branch { left, right } => op(left.fold(op), right.fold(op)),
        }
    }

    fn visit<F: Fn(T) -> T + Copy>(self, f: F) -> Self {
        match self {
            Self::Leaf(t) => Self::Leaf(f(t)),
            Self::Branch { left, right } => left.visit(f) & right.visit(f),
        }
    }

    fn topo_ordering(&mut self) -> Vec<&mut T> {
        let mut nodes: Vec<_> = Vec::new();

        match self {
            Self::Leaf(t) => nodes.push(t),
            Self::Branch { left, right } => {
                nodes.extend(left.topo_ordering());
                nodes.extend(right.topo_ordering());
            }
        }

        nodes
    }

    fn get_leaf(&self) -> Option<&T> {
        if let Self::Leaf(n) = self {
            Some(n)
        } else {
            None
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Node<T> {
    val: T,
    depth: usize,
}

impl<T> Node<T> {
    fn new(t: T, depth: usize) -> Self {
        Self { val: t, depth }
    }

    fn deepen(self) -> Node<T> {
        Node {
            depth: self.depth + 1,
            ..self
        }
    }
}

const TOMBSTONE: isize = -1;
impl Node<isize> {
    fn is_tombstone(&self) -> bool {
        self.val == TOMBSTONE
    }

    fn is_splittable(&self) -> bool {
        self.val < 0
    }
}

const EXPLODE_DEPTH: usize = 4;
impl BTree<Node<isize>> {
    fn explode(&mut self) -> bool {
        let mut ordering = self.topo_ordering();

        for i in 0..(ordering.len() - 1) {
            let p = ordering[i].val;
            let n = ordering[i + 1].val;
            if ordering[i + 1].depth > EXPLODE_DEPTH && ordering[i].depth == ordering[i + 1].depth {
                if i > 0 {
                    ordering[i - 1].val += p;
                }
                if i + 2 < ordering.len() {
                    ordering[i + 2].val += n;
                }
                ordering[i].val = TOMBSTONE;
                ordering[i + 1].val = TOMBSTONE;

                self.compact();
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        let mut ordering = self.topo_ordering();

        for i in 0..ordering.len() {
            let val = ordering[i].val;
            if val >= 10 {
                ordering[i].val = -val;
                self.compact();
                return true;
            }
        }
        false
    }

    fn compact(&mut self) {
        match self {
            Self::Branch { left, right } => {
                if let (Some(ln), Some(rn)) = (left.get_leaf(), right.get_leaf()) {
                    if ln.is_tombstone() && rn.is_tombstone() {
                        *self = Self::Leaf(Node::new(0, ln.depth - 1));
                        return;
                    }
                }
                left.compact();
                right.compact();
            }
            Self::Leaf(node) if node.is_splittable() => {
                let (val, depth) = (-node.val, node.depth);
                let rem = val % 2;
                *self = Self::Leaf(Node::new(val / 2, depth + 1))
                    & Self::Leaf(Node::new(val / 2 + rem, depth + 1));
            }
            _ => (),
        }
    }

    fn magnitude(self) -> isize {
        self.fold(|x, y| Node::new(3 * x.val + 2 * y.val, x.depth))
            .val
    }
}

type SnailNum = BTree<Node<isize>>;

fn add_snails(t1: &SnailNum, t2: &SnailNum) -> SnailNum {
    let mut t_sum = t1.clone() + t2.clone();

    let mut more = true;

    while more {
        let res = t_sum.explode();
        if !res {
            more = t_sum.split();
        }
    }
    t_sum
}

impl Display for SnailNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(x) => write!(f, "{}", x.val),
            Self::Branch { left, right } => write!(f, "[{}, {}]", *left, *right),
        }
    }
}

impl<T> Add for BTree<Node<T>> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.visit(Node::deepen) & rhs.visit(Node::deepen)
    }
}

impl<S: AsRef<str>, T> From<S> for BTree<Node<T>>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    fn from(s: S) -> Self {
        let parsed = SnailParser::parse(Rule::expr, s.as_ref())
            .expect("failed parsing tree :-(")
            .next()
            .unwrap();

        process_pair(parsed, 0)
    }
}

fn process_pair<T>(pair: Pair<Rule>, depth: usize) -> BTree<Node<T>>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    match pair.as_rule() {
        Rule::number => {
            let node = Node::new(pair.as_str().parse::<T>().unwrap(), depth);
            BTree::Leaf(node)
        }
        Rule::expr => {
            let nodes: Vec<_> = pair.into_inner().collect();
            process_pair(nodes[0].to_owned(), depth + 1)
                & process_pair(nodes[1].to_owned(), depth + 1)
        }
        _ => unreachable!(),
    }
}

aoc2021::main! {
    let input = include_str!("../../inputs/day18.txt");
    let mut input_lines = input.lines();

    // Part 1
    let mut t: SnailNum = input_lines.next().unwrap().into();
    for line in input_lines {
        t = add_snails(&t, &line.into());
    }

    let p1 = t.magnitude();

    // Part 2
    let trees: Vec<SnailNum> = input.to_owned().lines().map(|x| x.into()).collect();

    let mut max_sum = 0;
    trees.iter().tuple_combinations().for_each(|(x, y)| {
        max_sum = max([
            max_sum,
            add_snails(x, y).magnitude(),
            add_snails(y, x).magnitude(),
        ])
        .unwrap();
    });

    (p1, max_sum)
}
