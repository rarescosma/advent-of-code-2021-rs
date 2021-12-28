use aoc_2dmap::{Adjacency, Map, Pos};
use itertools::Itertools;
use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;
use std::fmt::{Debug, Display};
use std::ops::{Add, Deref};

type VertexName = usize;

#[derive(Debug, Clone)]
struct Edge {
    tail: VertexName,
    cost: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

type AdjList = Vec<Vec<Edge>>;
type Dist = Vec<usize>;
type Prev = Vec<Option<VertexName>>;

#[allow(dead_code)]
type Path = Vec<VertexName>;

#[derive(Debug, Clone)]
struct ExtendingMap<T>(Map<T>)
where
    T: Copy + Display;

#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
}

// Explicitly implementing `Ord` so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Axis {
    fn map(&self, x: i32, y: i32) -> Pos {
        match self {
            Axis::X => Pos { x, y },
            Axis::Y => Pos { x: y, y: x },
        }
    }
}

impl<T: Debug + Copy + Clone + Default + Display> ExtendingMap<T> {
    fn extend_front<P: Fn(T) -> T>(&self, along: Axis, by: i32, f: P) -> Self {
        let mut extended = Map::<T>::fill_default(self.size + along.map(by, 0));
        for pos in self.iter() {
            let tile = self.get(pos).unwrap();
            extended.set(pos + along.map(by, 0), f(tile));
        }
        Self(extended)
    }

    fn tile<P: Fn(T) -> T>(self, along: Axis, num: i32, f: P) -> Self {
        let mut uber_tiles = vec![self.clone()];
        let offset = along.map(self.size.x, self.size.y).x;
        for _ in 1..num {
            let cur = uber_tiles.last().unwrap().extend_front(along, offset, &f);
            uber_tiles.push(cur);
        }
        uber_tiles.into_iter().reduce(|x, y| x + y).unwrap()
    }

    fn pos_to_name(&self, pos: Pos) -> usize {
        (pos.y * self.size.x + pos.x) as usize
    }

    #[allow(dead_code)]
    /// Usage:
    ///
    /// ```
    /// let (distance, prev) = dijsktra_std(&adj_list, 0);
    /// let end_path = shortest_path(end_node, &prev);
    /// print!("{}", map.to_html(&end_path, distance[end_node]));
    /// ```
    fn to_html(&self, path: &Path, distance: usize) -> String {
        let mut prev_pos = Pos { x: 0, y: 0 };
        let mut buf = format!(
            "<html><head><style>.onpath {{ font-weight: bold; color: red; }}</style></head>\
<body><pre style='font-size: 2em; color: #555555;'>Distance to end node: <b>{}</b>\n\n",
            distance
        );
        for pos in self.iter() {
            if pos.y != prev_pos.y {
                buf.push('\n');
            }
            let v_name = self.pos_to_name(pos);
            let v_val = self.get(pos).unwrap();
            if path.contains(&v_name) {
                buf.push_str(&format!("<span class='onpath'>{}</span>", v_val))
            } else {
                buf.push_str(&format!("{}", v_val))
            }

            prev_pos = pos;
        }
        buf.push_str("</pre></body></html>");
        buf
    }
}

impl<T: Copy + Clone + Display + Default> Add for ExtendingMap<T> {
    type Output = ExtendingMap<T>;

    // stich together maps by extending rhs
    fn add(self, rhs: Self) -> Self::Output {
        let width = max(self.size.x, rhs.size.x);
        let height = max(self.size.y, rhs.size.y);

        let mut inner = Map::<T>::fill_default(Pos {
            x: width,
            y: height,
        });

        for x in 0..=width {
            for y in 0..=height {
                let tile_pos = Pos { x, y };
                if let Some(e) = rhs.get(tile_pos) {
                    inner.set(tile_pos, e);
                }
                if let Some(e) = self.get(tile_pos) {
                    inner.set(tile_pos, e);
                }
            }
        }

        Self(inner)
    }
}

impl<T: Copy + Display> Deref for ExtendingMap<T> {
    type Target = Map<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn read_input() -> Vec<&'static str> {
    include_str!("../../inputs/day15.txt").lines().collect()
}

fn dijsktra_std(adj_list: &AdjList, start: VertexName) -> (Dist, Prev) {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut vis: Vec<_> = (0..adj_list.len()).map(|_| false).collect();
    let mut prev: Vec<_> = (0..adj_list.len()).map(|_| None).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Important as we may have already found a better way
        vis[position] = true;
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            if vis[edge.tail] {
                continue;
            }

            let next = State {
                cost: dist[position] + edge.cost as usize,
                position: edge.tail,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
                prev[next.position] = Some(position);
            }
        }
    }

    (dist, prev)
}

#[allow(dead_code)]
fn shortest_path(to: VertexName, prev: &Prev) -> Vec<VertexName> {
    let mut v = prev[to];
    let mut path: Vec<VertexName> = vec![to];
    while let Some(vertex) = v {
        path.push(vertex);
        v = prev[vertex];
    }
    path.reverse();
    path
}

fn inc_tile(t: usize) -> usize {
    if t == 0 {
        0
    } else {
        max(1, (t + 1).rem_euclid(10))
    }
}

fn shortest_distance(map: &ExtendingMap<usize>) -> usize {
    let adj_list: AdjList = map
        .iter()
        .map(|pos| {
            let v_name = map.pos_to_name(pos);
            let neighs: Vec<_> = pos
                .neighbors(Adjacency::Simple)
                .into_iter()
                .flat_map(|n_pos| {
                    Some(Edge {
                        tail: map.pos_to_name(n_pos),
                        cost: map.get(n_pos)?,
                    })
                })
                .collect();
            (v_name, neighs)
        })
        .sorted_by_key(|(v_name, _)| *v_name)
        .map(|(_, adj)| adj)
        .collect();

    let end_node = (map.size.x * map.size.y - 1) as usize;
    let (distance, _) = dijsktra_std(&adj_list, 0);
    distance[end_node]
}

aoc2021::main! {
    let lines = read_input();

    let tiles: Vec<_> = lines
        .iter()
        .flat_map(|x| {
            x.chars()
                .into_iter()
                .flat_map(|c| c.to_string().parse::<usize>())
        })
        .collect();

    let map = ExtendingMap(Map::<usize>::new(
        Pos {
            x: lines[0].len() as i32,
            y: lines.len() as i32,
        },
        tiles,
    ));
    let p1 = shortest_distance(&map);

    let map = map.tile(Axis::X, 5, inc_tile).tile(Axis::Y, 5, inc_tile);

    let p2 = shortest_distance(&map);
    (p1, p2)
}
