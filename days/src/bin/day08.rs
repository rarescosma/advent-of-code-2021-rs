use hashbrown::{HashMap, HashSet};
use pest::iterators::Pair;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "parsers/day08-digits.pest"]
pub struct DigitsParser;

type Segments = HashSet<char>;

#[derive(Debug, Eq)]
pub struct Digit {
    segments: Segments,
}

impl Digit {
    fn withdraw<F: Fn(&Segments) -> bool>(predicate: F, candidates: &mut Vec<Digit>) -> Self {
        let (idx, _) = candidates
            .iter()
            .enumerate()
            .find(|&(_, y)| predicate(&y.segments))
            .unwrap();
        candidates.swap_remove(idx)
    }
}

impl ToString for Digit {
    fn to_string(&self) -> String {
        let mut chars: Vec<&char> = self.segments.iter().collect();
        chars.sort();
        chars.into_iter().collect()
    }
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            segments: HashSet::<char>::from_iter(s.chars()),
        })
    }
}

impl Hash for Digit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.to_string().as_bytes())
    }
}

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

fn process_line<'r>(line: &(impl Iterator<Item = Pair<'r, Rule>> + Clone)) -> (usize, Option<i32>) {
    let patterns = extract_digits(line.clone(), Rule::patterns);
    let outputs = extract_digits(line.clone(), Rule::outputs);

    let decode_map = get_decode_map(patterns);

    let easy = outputs
        .iter()
        .filter(|x| [2, 3, 4, 7].contains(&x.segments.len()))
        .count();

    let decoded: Vec<String> = outputs
        .iter()
        .map(|x| decode_map.get(x).unwrap().to_owned())
        .collect();

    (easy, decoded.join("").parse::<i32>().ok())
}
fn extract_digits<'r>(rules: impl Iterator<Item = Pair<'r, Rule>>, rule_type: Rule) -> Vec<Digit> {
    rules
        .filter(|x| x.as_rule() == rule_type)
        .flat_map(|inner| inner.into_inner().flat_map(|x| x.as_str().parse()))
        .collect()
}

/*
1 <== 2 segments
7 <== 3 segments
4 <== 4 segments
8 <== 7 segments
3 <== 5 segments on including 1
9 <== 6 segments on including 4
0 <== 6 segments on including 1
6 <== 6 segments on
5 <== 5 segments on included in 6
2 <== last one of the bunch
*/
fn get_decode_map(mut patterns: Vec<Digit>) -> HashMap<Digit, String> {
    let p_ref = &mut patterns;
    let one = Digit::withdraw(|x| x.len() == 2, p_ref);
    let seven = Digit::withdraw(|x| x.len() == 3, p_ref);
    let four = Digit::withdraw(|x| x.len() == 4, p_ref);
    let eight = Digit::withdraw(|x| x.len() == 7, p_ref);
    let three = Digit::withdraw(|x| x.len() == 5 && x.is_superset(&one.segments), p_ref);
    let nine = Digit::withdraw(|x| x.len() == 6 && x.is_superset(&four.segments), p_ref);
    let zero = Digit::withdraw(|x| x.len() == 6 && x.is_superset(&one.segments), p_ref);
    let six = Digit::withdraw(|x| x.len() == 6, p_ref);
    let five = Digit::withdraw(|x| x.len() == 5 && x.is_subset(&six.segments), p_ref);
    let two = Digit::withdraw(|_| true, p_ref);

    [zero, one, two, three, four, five, six, seven, eight, nine]
        .into_iter()
        .enumerate()
        .map(|(idx, d)| (d, format!("{}", idx)))
        .collect()
}

aoc2021::main! {
    let input = include_str!("../../inputs/day08.txt").to_string();

    let digits_parse = DigitsParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap();

    let res: Vec<_> = digits_parse
        .into_inner()
        .filter(|x| x.as_rule() == Rule::line)
        .map(|line| process_line(&line.into_inner()))
        .collect();

    let p1: usize = res.iter().map(|(easy, _)| easy).sum();
    let p2: i32 = res.iter().flat_map(|(_, opt)| opt).sum();

    (p1, p2)
}
