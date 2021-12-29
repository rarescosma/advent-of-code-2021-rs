use aoc_2dmap::prelude::*;
use aoc_prelude::prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day09-depth.pest"]
pub struct DepthParser;

#[derive(Debug)]
struct DepthMap(Map<u8>);

const MAX_DEPTH: u8 = 9;

impl DepthMap {
    fn new(depths: Vec<Vec<u8>>) -> Self {
        Self(Map::new(
            (depths[0].len(), depths.len()).into(),
            depths.into_iter().flatten().collect(),
        ))
    }

    fn depth_of(&self, pos: Pos) -> u8 {
        self.0.get(pos).unwrap_or(MAX_DEPTH)
    }

    fn flows<F: Fn(u8) -> bool>(&self, pos: Pos, pred: F) -> Vec<(Pos, bool)> {
        let depth = self.depth_of(pos);
        pos.neighbors_simple()
            .into_iter()
            .flat_map(move |pos| {
                self.0
                    .get(pos)
                    .map(|n_depth| (pos, pred(n_depth) && n_depth > depth))
            })
            .collect()
    }

    fn basin_size(&self, pos: Pos) -> usize {
        let depth = self.depth_of(pos);
        if depth == MAX_DEPTH {
            return 0;
        }
        let mut stack = VecDeque::<Pos>::new();
        stack.push_back(pos);

        let mut basin_acc = HashSet::<Pos>::new();
        basin_acc.insert(pos);

        while let Some(pos) = stack.pop_back() {
            let basin: Vec<_> = self.basin(pos).collect();
            basin_acc.extend(basin.iter());
            stack.extend(basin.iter());
        }
        basin_acc.len()
    }

    fn basin(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        self.flows(pos, |n_depth| n_depth < MAX_DEPTH)
            .into_iter()
            .filter(|&(_, n_flow)| n_flow)
            .map(|(n_pos, _)| n_pos)
    }
}

fn process_line<'r>(line: &(impl Iterator<Item = Pair<'r, Rule>> + Clone)) -> Vec<u8> {
    line.to_owned()
        .filter(|x| x.as_rule() == Rule::digit)
        .flat_map(|x| x.as_str().parse::<u8>())
        .collect()
}

aoc_2021::main! {
    let input = include_str!("../../inputs/day09.txt").to_string();

    let depth_parse = DepthParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap();

    let depths: Vec<_> = depth_parse
        .into_inner()
        .filter(|x| x.as_rule() == Rule::line)
        .map(|line| process_line(&line.into_inner()))
        .collect();

    let depth_map = DepthMap::new(depths);

    let p1: u64 = depth_map
        .0
        .iter()
        .filter(|&pos| depth_map.flows(pos, |_| true).into_iter().all(|(_, x)| x))
        .map(|pos| depth_map.depth_of(pos) as u64 + 1)
        .sum();

    let mut basins: Vec<_> = depth_map
        .0
        .iter()
        .filter(|&pos| {
            depth_map
                .flows(pos, |n_depth| n_depth < MAX_DEPTH)
                .into_iter()
                .all(|(_, x)| x)
        })
        .map(|pos| depth_map.basin_size(pos))
        .collect();

    basins.sort_unstable_by(|a, b| b.cmp(a));

    let p2: usize = basins.iter().take(3).product();

    (p1, p2)
}
