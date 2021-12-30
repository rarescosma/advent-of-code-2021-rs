use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::{BuildHasher, Hash, Hasher};

use ahash::RandomState;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHER_BUILDER: RandomState = RandomState::new();
}

pub trait Transform<G> {
    fn cost(&self) -> usize;
    fn transform(&self, game_state: &G) -> G;
}

pub trait GameState<C>: Ord + Hash {
    type Steps: IntoIterator;

    fn accept(&self) -> bool;

    fn steps(&self, ctx: &mut C) -> Self::Steps;
}

pub trait Dijsktra<C>: private::Sealed<C> {
    fn dijsktra(self, ctx: &mut C) -> Option<usize>;
}

impl<C, T> Dijsktra<C> for T
where
    T: GameState<C>,
    <T::Steps as IntoIterator>::Item: Transform<T>,
{
    /// compute the shortest path through a graph of costs and states
    fn dijsktra(self, context: &mut C) -> Option<usize> {
        let mut visited = HashMap::with_capacity(1024);

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), self));

        while let Some((Reverse(cost), state)) = pq.pop() {
            if state.accept() {
                return Some(cost);
            }
            for step in state.steps(context) {
                let cost = cost + step.cost();
                let new_state = step.transform(&state);

                match visited.entry(manually_hash(&new_state)) {
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

fn manually_hash<H: Hash>(state: &H) -> u64 {
    let mut hasher = HASHER_BUILDER.build_hasher();
    state.hash(&mut hasher);
    hasher.finish()
}

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
