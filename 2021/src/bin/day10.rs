use aoc_prelude::prelude::*;

struct Pair(char, char, u32, u16);

const PRS: [Pair; 4] = [
    Pair('(', ')', 3, 1),
    Pair('[', ']', 57, 2),
    Pair('{', '}', 1197, 3),
    Pair('<', '>', 25137, 4),
];

lazy_static! {
    static ref OPENING: HashSet<char> = PRS.iter().map(|x| x.0).collect();
    static ref CLOSING: HashSet<char> = PRS.iter().map(|x| x.1).collect();
    static ref OPEN_TO_CLOSE: HashMap<char, char> = PRS.iter().map(|x| (x.0, x.1)).collect();
    static ref ERR_SCORE: HashMap<char, u32> = PRS.iter().map(|x| (x.1, x.2)).collect();
    static ref COMP_SCORE: HashMap<char, u16> = PRS.iter().map(|x| (x.1, x.3)).collect();
}

#[derive(Debug)]
enum LineType {
    Complete,
    Incomplete(String), // completion sequence
    Illegal(char),      // first illegal char
}

fn read_input() -> Vec<&'static str> {
    include_str!("../../inputs/day10.txt").lines().collect()
}

fn process_line<S: AsRef<str>>(line: S) -> LineType {
    let mut stack = VecDeque::new();
    for c in line.as_ref().chars() {
        if is_opening(&c) {
            stack.push_back(c);
        }
        if is_closing(&c) && !stack.is_empty() {
            let last = stack.pop_back().unwrap();
            assert!(is_opening(&last));
            let expecting = open_to_close(&last);
            if c != expecting {
                return LineType::Illegal(c);
            }
        }
    }
    if stack.is_empty() {
        LineType::Complete
    } else {
        LineType::Incomplete(stack.iter().rev().map(open_to_close).collect())
    }
}

#[inline]
fn is_opening(c: &char) -> bool {
    OPENING.contains(c)
}

#[inline]
fn is_closing(c: &char) -> bool {
    CLOSING.contains(c)
}

#[inline]
fn open_to_close(c: &char) -> char {
    *OPEN_TO_CLOSE.get(c).unwrap()
}

fn comp_score<T: PrimInt>(completion: &(dyn AsRef<str>)) -> Option<T> {
    let mut total: T = T::zero();
    let base = T::from(5)?;
    for c in completion.as_ref().chars() {
        total = total.checked_mul(&base)?;
        total = total.checked_add(&T::from(*COMP_SCORE.get(&c)?)?)?;
    }
    Some(total)
}

fn median<T: PrimInt>(numbers: &mut [T]) -> T {
    numbers.sort_unstable();
    numbers[numbers.len() / 2]
}

aoc_2021::main! {
    let lines = read_input();

    let p1: u32 = lines
        .to_owned()
        .into_iter()
        .map(process_line)
        .filter_map(|x| match x {
            LineType::Illegal(c) => ERR_SCORE.get(&c),
            _ => None,
        })
        .sum();

    let p2_scores: Vec<_> = lines
        .into_iter()
        .map(process_line)
        .filter_map(|x| match x {
            LineType::Incomplete(s) => Some(s),
            _ => None,
        })
        .map(|s| comp_score(&s))
        .collect();

    let mut scores: Vec<_> = p2_scores.into_iter().flatten().collect();
    let p2: u64 = median(&mut scores);

    (p1, p2)
}
