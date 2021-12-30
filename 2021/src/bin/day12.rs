use aoc_prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day12-graph.pest"]
pub struct GraphParser;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    End,
    Node { name: &'a str, small: bool },
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(s: &'a str) -> Cave<'a> {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            name => Cave::Node {
                name,
                small: is_lower(name),
            },
        }
    }
}

type Graph<'g> = MultiMap<Cave<'g>, Cave<'g>>;

fn read_input() -> Graph<'static> {
    let mut inner = Vec::with_capacity(2);

    GraphParser::parse(Rule::lines, include_str!("../../inputs/day12.txt"))
        .expect("failed parse")
        .next()
        .unwrap()
        .into_inner()
        .filter(|pair| pair.as_rule() == Rule::line)
        .flat_map(|pair| {
            inner.clear();
            inner.extend(pair.into_inner());
            let (from, to) = (inner[0].as_str().into(), inner[1].as_str().into());
            [(from, to), (to, from)]
        })
        .fold(Graph::with_capacity(256), |mut graph, (from, to)| {
            graph.insert(from, to);
            graph
        })
}

fn is_lower(s: &str) -> bool {
    s.chars().all(char::is_lowercase)
}

fn solve(graph: &Graph<'_>, allow_small_revisit: bool) -> Option<usize> {
    let mut paths = 0;

    let mut deck = VecDeque::with_capacity(2048);
    deck.push_front((vec![Cave::Start], false));

    while let Some((path, small_revisited)) = deck.pop_front() {
        for cave in graph.get_vec(path.last()?)? {
            match *cave {
                Cave::Start => continue,
                Cave::End => paths += 1,
                Cave::Node { name: _, small } => {
                    let mut small_revisited = small_revisited;
                    if small && path.contains(cave) {
                        if !allow_small_revisit || small_revisited {
                            continue;
                        }
                        small_revisited = true;
                    };
                    let new_path = path.iter().cloned().chain([*cave]).collect();
                    deck.push_front((new_path, small_revisited));
                }
            }
        }
    }
    Some(paths)
}

aoc_2021::main! {
    let graph = read_input();

    let p1 = solve(&graph, false).unwrap();
    let p2 = solve(&graph, true).unwrap();

    (p1, p2)
}
