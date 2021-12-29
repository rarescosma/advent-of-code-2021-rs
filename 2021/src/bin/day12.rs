use aoc_prelude::prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day12-graph.pest"]
pub struct GraphParser;

type CaveId<'a> = &'a str;

#[derive(Debug, Clone)]
enum CaveType {
    Big,
    Small,
}

#[derive(Debug)]
struct Graph<'a> {
    cave_types: HashMap<CaveId<'a>, CaveType>,
    edges: MultiMap<CaveId<'a>, CaveId<'a>>,
}

impl<'a> Graph<'a> {
    fn is_big(&self, cave_id: CaveId) -> bool {
        return matches!(self.cave_types.get(cave_id).unwrap(), CaveType::Big);
    }
}

fn read_input() -> String {
    include_str!("../../inputs/day12.txt").into()
}

fn process_line<'r>(
    line: &(impl Iterator<Item = Pair<'r, Rule>> + Clone),
) -> Vec<(CaveId<'r>, CaveType)> {
    line.to_owned()
        .map(|x| match x.as_rule() {
            Rule::big_cave => (x.as_str(), CaveType::Big),
            Rule::small_cave => (x.as_str(), CaveType::Small),
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let input = read_input();

    let graph_parse = GraphParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap();

    let caves: Vec<Vec<(CaveId, CaveType)>> = graph_parse
        .into_inner()
        .filter(|x| x.as_rule() == Rule::line)
        .map(|line| process_line(&line.into_inner()))
        .collect();

    let mut cave_types: HashMap<CaveId, CaveType> = HashMap::new();
    caves
        .to_owned()
        .into_iter()
        .flatten()
        .for_each(|(cave_id, cave)| {
            cave_types.insert(cave_id, cave);
        });

    let mut edges: MultiMap<CaveId, CaveId> = MultiMap::new();
    caves.into_iter().for_each(|x| {
        edges.insert(x[0].0, x[1].0);
        edges.insert(x[1].0, x[0].0)
    });

    let graph = Graph { cave_types, edges };

    let mut path = vec!["start"];
    let mut answer: usize = 0;
    bfs(&graph, "start", "end", &mut path, &mut answer);
    dbg!(&answer);
}

fn get_outgoing<'a>(graph: &Graph<'a>, from: CaveId) -> Vec<CaveId<'a>> {
    if from == "end" {
        return Vec::new();
    }

    if let Some(out) = graph.edges.get_vec(from) {
        return out.to_owned();
    }
    Vec::new()
}

fn allowed_revisit(graph: &Graph, next_node: CaveId, cur_path: &[CaveId]) -> bool {
    if graph.is_big(next_node) {
        return true;
    }
    if next_node == "start" || next_node == "end" {
        return false;
    }
    let times = cur_path.iter().filter(|x| **x == next_node).count();
    times < 2
}

fn no_of_repeated_small_caves(graph: &Graph, path: &[CaveId]) -> usize {
    let mut freqs: BTreeMap<&CaveId, usize> = BTreeMap::new();
    for cave_id in path {
        if !graph.is_big(cave_id) {
            *freqs.entry(cave_id).or_insert(0) += 1;
        }
    }
    freqs.values().filter(|&&x| x >= 2).count()
}

fn bfs<'a>(
    graph: &Graph<'a>,
    start_id: CaveId,
    end_id: CaveId,
    path: &mut Vec<CaveId<'a>>,
    num_paths: &mut usize,
) {
    let next_nodes = get_outgoing(graph, start_id);
    for next_node in next_nodes.iter() {
        if *next_node == end_id {
            if no_of_repeated_small_caves(graph, path) < 2 {
                *num_paths += 1;
            }
        } else if allowed_revisit(graph, next_node, path) {
            path.push(next_node);
            bfs(graph, next_node, end_id, path, num_paths);
            path.pop();
        }
    }
}
