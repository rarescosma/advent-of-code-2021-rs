use hashbrown::HashMap;

type FishState = u16;
type FishCounts = HashMap<FishState, usize>;
type Transitions = HashMap<FishState, FishState>;

const FISH_SPAWN: FishState = 8;
const FISH_RESET: FishState = 6;

fn epoch(fish_counts: &FishCounts, transitions: &Transitions) -> FishCounts {
    let mut next_fish_counts = FishCounts::new();
    let spawning_fishes = fish_counts.get(&0).unwrap_or(&0);

    for (cur_state, next_state) in transitions {
        let next_state_fishes = fish_counts.get(cur_state).unwrap_or(&0);
        *next_fish_counts.entry(*next_state).or_insert(0) += next_state_fishes;
    }

    *next_fish_counts.entry(FISH_SPAWN).or_insert(0) += spawning_fishes;

    next_fish_counts
}

fn read_input() -> Vec<FishState> {
    include_str!("../../inputs/day06.txt")
        .to_string()
        .split(',')
        .flat_map(|s| s.parse())
        .collect()
}

aoc2021::main! {
    let input = read_input();

    let mut fish_counts = FishCounts::new();
    let mut transitions = Transitions::new();

    for fish_state in input {
        *fish_counts.entry(fish_state).or_insert(0) += 1;
    }

    // 'iterative' is boring, check below for 'poetic'
    for i in 0..FISH_SPAWN {
        transitions.insert(i + 1, i);
    }
    transitions.insert(0, FISH_RESET);

    for _ in 1..=80 {
        fish_counts = epoch(&fish_counts, &transitions);
    }
    let p1: usize = fish_counts.values().into_iter().sum();

    for _ in 81..=256 {
        fish_counts = epoch(&fish_counts, &transitions);
    }
    let p2: usize = fish_counts.values().into_iter().sum();

    (p1, p2)
}
