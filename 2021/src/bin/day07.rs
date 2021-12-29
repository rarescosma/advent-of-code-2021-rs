#[allow(dead_code)]
enum CrabEngine {
    Linear,
    Exponential,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Crab(i32);

impl Crab {
    fn expenditure(&self, new_pos: &i32, engine_type: &CrabEngine) -> i32 {
        let delta = _abs_diff(&self.0, new_pos);
        match engine_type {
            CrabEngine::Linear => delta,
            CrabEngine::Exponential => (delta * (delta + 1)) >> 1,
        }
    }
}

fn min_expenditure(crabs: &[Crab], engine_type: CrabEngine) -> i32 {
    let min_b = crabs.iter().min().unwrap().0;
    let max_b = crabs.iter().max().unwrap().0;
    let expenditures: Vec<i32> = (min_b..=max_b)
        .map(|new_pos| {
            crabs
                .iter()
                .map(|crab| crab.expenditure(&new_pos, &engine_type))
                .sum()
        })
        .collect();
    *expenditures.iter().min().unwrap()
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn expenditure(numbers: &[i32], median: &i32) -> i32 {
    numbers.iter().map(|x| _abs_diff(x, median)).sum()
}

fn _abs_diff(l: &i32, r: &i32) -> i32 {
    (*l - *r).abs()
}

fn read_input() -> Vec<i32> {
    include_str!("../../inputs/day07.txt")
        .to_string()
        .split(',')
        .flat_map(|s| s.parse::<i32>())
        .collect()
}

aoc_2021::main! {
    let mut numbers = read_input();

    let median = median(&mut numbers);

    let p1 = expenditure(&numbers, &median);

    let crabs: Vec<Crab> = numbers.iter().map(|x| Crab(*x)).collect();

    let p2 = min_expenditure(&crabs, CrabEngine::Exponential);

    (p1, p2)
}
