use hashbrown::HashMap;
use pest::Parser;
use pest_derive::Parser;
use std::collections::BTreeMap;

#[derive(Parser)]
#[grammar = "parsers/day14-polymer.pest"]
pub struct FoldsParser;

fn read_input() -> String {
    include_str!("../../inputs/day14.txt").to_string()
}

fn extract_freqs(
    pairs: &HashMap<[char; 2], usize>,
    first: char,
    last: char,
) -> BTreeMap<char, usize> {
    let mut freqs: HashMap<char, usize> = HashMap::new();
    for (pair, count) in pairs {
        *freqs.entry(pair[0]).or_insert(0) += count;
        *freqs.entry(pair[1]).or_insert(0) += count;
    }

    freqs
        .into_iter()
        .map(|(c, s)| {
            if c == first || c == last {
                (c, (s + 1) / 2)
            } else {
                (c, s / 2)
            }
        })
        .collect()
}

fn simulate_pairs(
    pairs: &HashMap<[char; 2], usize>,
    insertions: &HashMap<[char; 2], char>,
    steps: usize,
) -> HashMap<[char; 2], usize> {
    let mut pairs = pairs.clone();
    for _ in 1..=steps {
        let mut new_pairs = HashMap::<[char; 2], usize>::new();
        for (pair, count) in pairs {
            if let Some(mid) = insertions.get(&pair) {
                *new_pairs.entry([pair[0], *mid]).or_insert(0) += count;
                *new_pairs.entry([*mid, pair[1]]).or_insert(0) += count;
            }
        }
        pairs = new_pairs;
    }
    pairs
}

aoc2021::main! {
    let input = read_input();

    let parsed = FoldsParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap();

    let mut pairs = HashMap::<[char; 2], usize>::new();
    let mut ins = HashMap::<[char; 2], char>::new();
    let mut first = char::default();
    let mut last = char::default();

    for rule in parsed.into_inner() {
        match rule.as_rule() {
            Rule::template => {
                let tpl: Vec<_> = rule.as_str().chars().collect();
                first = tpl[0];
                last = *tpl.last().unwrap();
                tpl.windows(2).for_each(|x| {
                    *pairs.entry([x[0], x[1]]).or_insert(0) += 1;
                });
            }
            Rule::insertion => {
                let fold: Vec<_> = rule.as_str().split(" -> ").collect();
                let (mut k, v) = (fold[0].chars(), fold[1]);
                let k1 = k.next().unwrap();
                let k2 = k.next().unwrap();
                ins.insert([k1, k2], v.chars().next().unwrap());
            }
            _ => (),
        }
    }

    let pairs = simulate_pairs(&pairs, &ins, 10);
    let p1_freqs = extract_freqs(&pairs, first, last);
    let p1 = p1_freqs.values().max().unwrap() - p1_freqs.values().min().unwrap();

    let pairs = simulate_pairs(&pairs, &ins, 30);
    let p2_freqs = extract_freqs(&pairs, first, last);
    let p2 = p2_freqs.values().max().unwrap() - p2_freqs.values().min().unwrap();

    (p1, p2)
}
