use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

use hashbrown::hash_map::Entry;
use num_traits::PrimInt;

mod hash;

pub use crate::hash::manually_hash;

/// Transform one game state into another incurring a cost.
pub trait Transform<G, N: PrimInt = usize> {
    fn cost(&self) -> N;
    fn transform(&self, game_state: &G) -> G;
}

/// Hash-able representation of the current game state.
/// It generates `Transform`s through the `steps` method, which also gets
/// convenience access to a context type.
pub trait GameState<C, N = usize>: Ord + Hash {
    type Steps: IntoIterator;

    fn accept(&self, cost: N, ctx: &mut C) -> bool;

    fn steps(&self, ctx: &mut C) -> Self::Steps;
}

pub trait Dijsktra<C, N>: private::Sealed<C, N> {
    fn dijsktra(self, ctx: &mut C) -> Option<N>;
}

/// `GameState` implementors who produce self-compatible `Transform`s (through
/// their `steps` method) get a nifty generic blanket implementation of
/// Dijkstra's shortest path algorithm.
///
/// Batteries (priority queue optimization) included.
impl<C, T, N> Dijsktra<C, N> for T
where
    N: PrimInt,
    T: GameState<C, N>,
    <T::Steps as IntoIterator>::Item: Transform<T, N>,
{
    /// Compute the least total cost for reaching a goal (as indicated by
    /// the `accept` method on the `GameState` implementor).
    fn dijsktra(self, context: &mut C) -> Option<N> {
        let mut known = hash::KeyMap::default();
        let mut pq = BinaryHeap::with_capacity(1024);

        pq.push((Reverse(N::from(0).unwrap()), self));

        while let Some((Reverse(cost), state)) = pq.pop() {
            if state.accept(cost, context) {
                return Some(cost);
            }
            for step in state.steps(context) {
                let new_cost = cost + step.cost();
                let new_state = step.transform(&state);

                match known.entry(&new_state) {
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
    use super::{GameState, Transform, PrimInt};

    pub trait Sealed<C, N> {}

    impl<T, C, N> Sealed<C, N> for T
    where
        N: PrimInt,
        T: GameState<C, N>,
        <T::Steps as IntoIterator>::Item: Transform<T, N>,
    {
    }
}
