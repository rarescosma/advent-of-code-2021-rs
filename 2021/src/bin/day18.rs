#![allow(clippy::suspicious_arithmetic_impl)]
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, BitAnd};
use std::str::FromStr;

use aoc_prelude::*;

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

    fn topo(&mut self) -> ArrayVec<&mut T, 64> {
        let mut nodes = ArrayVec::new();
        self._topo_rec(&mut nodes);
        nodes
    }

    fn _topo_rec<'a, 'b>(&'a mut self, recv: &'b mut ArrayVec<&'a mut T, 64>) {
        match self {
            Self::Leaf(t) => recv.push(t),
            Self::Branch { left, right } => {
                left._topo_rec(recv);
                right._topo_rec(recv);
            }
        }
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
    fn new(val: T, depth: usize) -> Self {
        Self { val, depth }
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
        let mut nodes = self.topo();

        for i in 0..(nodes.len() - 1) {
            let p = nodes[i].val;
            let n = nodes[i + 1].val;
            if nodes[i + 1].depth > EXPLODE_DEPTH && nodes[i].depth == nodes[i + 1].depth {
                if i > 0 {
                    nodes[i - 1].val += p;
                }
                if i + 2 < nodes.len() {
                    nodes[i + 2].val += n;
                }
                nodes[i].val = TOMBSTONE;
                nodes[i + 1].val = TOMBSTONE;

                drop(nodes);
                self.compact();
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        let mut nodes = self.topo();
        for node in &mut nodes {
            let val = node.val;
            if val >= 10 {
                node.val = -val;

                // ArrayVec implements Drop so we manually drop here instead
                // of the end of the loop, so we can move `self` into `compact`
                drop(nodes);

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
            let nodes: ArrayVec<_, 2> = pair.into_inner().collect();
            process_pair(nodes[0].to_owned(), depth + 1)
                & process_pair(nodes[1].to_owned(), depth + 1)
        }
        _ => unreachable!(),
    }
}

aoc_2021::main! {
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
        max_sum = itermax([
            max_sum,
            add_snails(x, y).magnitude(),
            add_snails(y, x).magnitude(),
        ])
        .unwrap();
    });

    (p1, max_sum)
}
