use aoc_prelude::prelude::*;

fn find_modelnum(
    visited: &mut HashSet<(i64, usize)>,
    blocks: &[(i64, i64, i64)],
    block: usize,
    z: i64,
    range: &[i64; 9],
) -> Option<i64> {
    if block == blocks.len() {
        return if z == 0 { Some(0) } else { None };
    }
    if visited.contains(&(z, block)) {
        return None;
    }
    let (p1, p2, p3) = blocks[block];
    for &i in range {
        let x = (z % 26 + p2 != i) as i64;
        let z = (z / p1) * (25 * x + 1) + (i + p3) * x;
        if let Some(n) = find_modelnum(visited, blocks, block + 1, z, range) {
            return Some(n * 10 + i);
        }
    }
    visited.insert((z, block));
    None
}

fn solve(blocks: &[(i64, i64, i64)], biggest: bool) -> String {
    let range = if biggest {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    let answer = find_modelnum(&mut HashSet::new(), blocks, 0, 0, &range).unwrap();
    answer.to_string().chars().rev().collect()
}

aoc_2021::main! {
    let lines = include_str!("../../inputs/day24.txt")
        .lines()
        .collect::<Vec<_>>();

    let blocks: Vec<_> = lines
        .chunks(18)
        .map(|block| {
            let p1 = block[4][6..].parse().unwrap();
            let p2 = block[5][6..].parse().unwrap();
            let p3 = block[15][6..].parse().unwrap();
            (p1, p2, p3)
        })
        .collect();

    (solve(&blocks, true), solve(&blocks, false))
}
