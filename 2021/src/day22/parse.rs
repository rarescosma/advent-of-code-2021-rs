use crate::Cube;
use aoc_prelude::prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day22-cubes.pest"]
struct CubeParser;

pub fn process_line<S: AsRef<str>>(line: S) -> (Cube, String) {
    let parsed = CubeParser::parse(Rule::cube, line.as_ref())
        .expect("failed parse")
        .next()
        .unwrap();

    let v: Vec<i64> = parsed
        .to_owned()
        .into_inner()
        .filter(|r| r.as_rule() == Rule::number)
        .map(|r| r.as_str().parse::<i64>().unwrap())
        .collect();

    let cmd = parsed
        .into_inner()
        .filter(|r| r.as_rule() == Rule::cmd)
        .map(|r| r.as_str().to_owned())
        .next()
        .unwrap();

    (([v[0], v[2], v[4]], [v[1], v[3], v[5]]).into(), cmd)
}
