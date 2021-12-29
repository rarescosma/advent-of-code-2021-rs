#[inline]
fn count_increases(xs: &[i64]) -> usize {
    xs.windows(2).filter(|&slice| slice[1] > slice[0]).count()
}

fn read_input() -> Vec<i64> {
    include_str!("../../inputs/day01.txt")
        .lines()
        .flat_map(|s| s.parse::<i64>())
        .collect::<Vec<_>>()
}

aoc_2021::main! {
    let input = read_input();

    let p1 = count_increases(&input);
    let p2 = count_increases(
        &input
            .windows(3)
            .map(|slice| slice.iter().sum())
            .collect::<Vec<_>>(),
    );

    (p1, p2)
}
