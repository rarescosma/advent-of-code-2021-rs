use aoc_prelude::*;
use regex::Regex;

lazy_static! {
    static ref PROG_REGEX: Regex = Regex::new(
        r"inp w
mul x 0
add x z
mod x 26
div z (-?\d+)
add x (-?\d+)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y (-?\d+)
mul y x
add z y"
    )
    .unwrap();
}

fn grok_asm(input: &str) -> Vec<[i64; 3]> {
    PROG_REGEX
        .captures_iter(input)
        .map(|captures| {
            let p0 = captures[1].parse().unwrap();
            let p1 = captures[2].parse().unwrap();
            let p2 = captures[3].parse().unwrap();
            [p0, p1, p2]
        })
        .collect()
}

fn read_input() -> Vec<[i64; 3]> {
    grok_asm(include_str!("../../inputs/day24.txt"))
}

fn solve(params: Vec<[i64; 3]>) -> (String, String) {
    let mut stack = VecDeque::new();
    let mut min = [0; 14];
    let mut max = [0; 14];
    for (j, &[p0, p1, p2]) in params.iter().enumerate() {
        match p0 {
            1 => stack.push_back((j, p2)),

            26 => {
                let (i, c) = stack.pop_back().unwrap();
                let d = p1 + c;
                let (i, j, d) = if d < 0 { (j, i, -d) } else { (i, j, d) };
                max[i] = 9 - d;
                max[j] = 9;
                min[i] = 1;
                min[j] = 1 + d;
            }
            _ => unreachable!(),
        }
    }
    let min = min.iter().join("");
    let max = max.iter().join("");
    (max, min)
}

aoc_2021::main! {
    solve(read_input())
}
