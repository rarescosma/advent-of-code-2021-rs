use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::{BuildHasher, Hash, Hasher};

use ahash::RandomState;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref BUILDER: RandomState = RandomState::new();
}

pub trait Transform<G> {
    fn cost(&self) -> usize;
    fn transform(&self, game_state: &G) -> G;
}

pub trait GameState<V, C>: Ord + Hash + Sized
where
    V: IntoIterator,
    <V as IntoIterator>::Item: Transform<Self>,
{
    fn accept(&self) -> bool;
    fn steps(&self, context: &C) -> V;

    /// compute the shortest path through a graph of costs and states
    fn dijsktra(self, context: &C) -> Option<usize> {
        let mut visited = HashMap::new();

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), self));

        while let Some((Reverse(cost), state)) = pq.pop() {
            if state.accept() {
                return Some(cost);
            }
            for step in state.steps(context) {
                let cost = cost + step.cost();
                let new_state = step.transform(&state);
                let new_hash = manual_hash(&new_state);

                match visited.entry(new_hash) {
                    // can we get to this (alread seen) state with a reduced cost?
                    Entry::Occupied(mut entry) if cost < *entry.get() => {
                        entry.insert(cost);
                        pq.push((Reverse(cost), new_state));
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(cost);
                        pq.push((Reverse(cost), new_state));
                    }
                    _ => (),
                }
            }
        }
        None
    }
}

fn manual_hash<H>(what: H) -> u64
where
    H: Hash,
{
    let mut hasher = BUILDER.build_hasher();
    what.hash(&mut hasher);
    hasher.finish()
}
