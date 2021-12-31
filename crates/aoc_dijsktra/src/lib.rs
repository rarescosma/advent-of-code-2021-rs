use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

use hashbrown::hash_map::Entry;

mod hash;

/// Transform one game state into another incurring a cost.
pub trait Transform<G> {
    fn cost(&self) -> usize;
    fn transform(&self, game_state: &G) -> G;
}

/// Hash-able representation of the current game state.
/// It generates `Transform`s through the `steps` method, which also gets
/// convenience access to a context type.
pub trait GameState<C>: Ord + Hash {
    type Steps: IntoIterator;

    fn accept(&self) -> bool;

    fn steps(&self, ctx: &mut C) -> Self::Steps;
}

pub trait Dijsktra<C>: private::Sealed<C> {
    fn dijsktra(self, ctx: &mut C) -> Option<usize>;
}

/// `GameState` implementors who produce self-compatible `Transform`s (through
/// their `steps` method) get a nifty generic blanket implementation of
/// Dijkstra's shortest path algorithm.
///
/// Batteries (priority queue optimization) included.
impl<C, T> Dijsktra<C> for T
where
    T: GameState<C>,
    <T::Steps as IntoIterator>::Item: Transform<T>,
{
    /// Compute the least total cost for reaching a goal (as indicated by
    /// the `accept` method on the `GameState` implementor).
    fn dijsktra(self, context: &mut C) -> Option<usize> {
        let mut known = hash::build_hashmap();

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), self));

        while let Some((Reverse(cost), state)) = pq.pop() {
            if state.accept() {
                return Some(cost);
            }
            for step in state.steps(context) {
                let new_cost = cost + step.cost();
                let new_state = step.transform(&state);

                match known.entry(hash::manually_hash(&new_state)) {
                    // Update if there's a less costly way to get to a known state...
                    Entry::Occupied(mut entry) if new_cost < *entry.get() => {
                        entry.insert(new_cost);
                        pq.push((Reverse(new_cost), new_state));
                    }
                    // ...or if the state is unknown.
                    Entry::Vacant(entry) => {
                        entry.insert(new_cost);
                        pq.push((Reverse(new_cost), new_state));
                    }
                    _ => (),
                }
            }
        }
        None
    }
}

/// Prevent other crates from implementing the `Dijsktra` trait. 😈
mod private {
    use super::{GameState, Transform};

    pub trait Sealed<C> {}

    impl<T, C> Sealed<C> for T
    where
        T: GameState<C>,
        <T::Steps as IntoIterator>::Item: Transform<T>,
    {
    }
}
