use aoc_prelude::prelude::*;

lazy_static! {
    static ref DIRAC: HashMap<u64, u64> = {
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
            .iter()
            .cloned()
            .collect()
    };
}

const DIRAC_SCORE: u64 = 21;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    fn roll(&self, roll: u64) -> Player {
        let mut new_pos = (self.pos + roll) % 10;
        if new_pos == 0 {
            new_pos = 10;
        }
        Player {
            pos: new_pos,
            score: self.score + new_pos,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    p1: Player,
    p2: Player,
}

impl State {
    fn roll(&self, roll: u64, p1_turn: bool) -> State {
        if p1_turn {
            State {
                p1: self.p1.roll(roll),
                p2: self.p2,
            }
        } else {
            State {
                p1: self.p1,
                p2: self.p2.roll(roll),
            }
        }
    }
}

fn wins(state: State, p1_turn: bool, cache: &mut HashMap<(State, bool), (u64, u64)>) -> (u64, u64) {
    if cache.contains_key(&(state, p1_turn)) {
        return cache[&(state, p1_turn)];
    }
    let (mut p1_tot, mut p2_tot) = (0, 0);
    for (roll, weight) in DIRAC.iter() {
        let new_state = state.roll(*roll, p1_turn);
        if p1_turn && new_state.p1.score >= DIRAC_SCORE {
            p1_tot += weight;
        } else if !p1_turn && new_state.p2.score >= DIRAC_SCORE {
            p2_tot += weight;
        } else {
            let (p1_wins, p2_wins) = wins(new_state, !p1_turn, cache);
            p1_tot += weight * p1_wins;
            p2_tot += weight * p2_wins;
        }
    }
    cache.insert((state, p1_turn), (p1_tot, p2_tot));
    (p1_tot, p2_tot)
}

aoc_2021::main! {
    let s = State {
        p1: Player { pos: 2, score: 0 },
        p2: Player { pos: 5, score: 0 },
    };

    let num_wins = wins(s, true, &mut HashMap::new());

    (0, max(num_wins.0, num_wins.1))
}
