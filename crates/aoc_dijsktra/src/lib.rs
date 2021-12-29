use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

pub trait GameState<V, C>
where
    V: IntoIterator,
    <V as IntoIterator>::Item: Transform<Self>,
{
    fn accept(&self) -> bool;
    fn steps(&self, context: &C) -> V;
}

pub trait Transform<S: ?Sized> {
    fn cost(&self) -> usize;
    fn transform(&self, game_state: &S) -> S;
}

/// compute the shortest path through a graph of costs and states
pub fn dijsktra<V, S, C>(initial_state: S, context: &C) -> Option<usize>
where
    V: IntoIterator,
    <V as IntoIterator>::Item: Transform<S>,
    S: GameState<V, C> + Ord + Clone + Hash,
{
    let mut pq = BinaryHeap::new();
    let mut visited = HashMap::new();
    pq.push((Reverse(0), initial_state));
    while let Some((Reverse(cost), state)) = pq.pop() {
        if state.accept() {
            return Some(cost);
        }
        for step in state.steps(context) {
            let cost = cost + step.cost();
            let new_state = step.transform(&state);

            match visited.entry(new_state.clone()) {
                Entry::Occupied(mut entry) => {
                    // If this state is already known we only keep it if the
                    // cost is less.
                    if cost < *entry.get() {
                        entry.insert(cost);
                        pq.push((Reverse(cost), new_state));
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(cost);
                    pq.push((Reverse(cost), new_state));
                }
            }
        }
    }
    None
}
