/// We want to keep track of the costs of visited GameStates, but
/// inserting into `HashMap`s requires owned values, so we use a `KeyMap`
/// instead which uses `manually_hash`, then inserts the resulting hash
/// in a regular hash table set up to use a `MirrorHasher` which simply
/// mirrors back the `u64` produced by `manually_hash`.
use std::hash::{BuildHasherDefault, Hash, Hasher};

use ahash::RandomState;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use num_traits::PrimInt;

lazy_static! {
    static ref HASHER_BUILDER: RandomState = RandomState::new();
}

pub(crate) struct KeyMap<N: PrimInt> {
    hashmap: HashMap<u64, N, MirrorHashBuilder>,
}

impl<N: PrimInt> KeyMap<N> {
    pub fn entry<H: Hash>(&mut self, k: &H) -> Entry<u64, N, MirrorHashBuilder> {
        self.hashmap.entry(manually_hash(k))
    }
}

impl<N: PrimInt> Default for KeyMap<N> {
    fn default() -> Self {
        Self {
            hashmap: HashMap::with_capacity_and_hasher(1024, MirrorHashBuilder::default()),
        }
    }
}

pub fn manually_hash<H: Hash>(state: &H) -> u64 {
    HASHER_BUILDER.hash_one(state)
}

#[derive(Default)]
pub(crate) struct MirrorHasher {
    state: u64,
}

type MirrorHashBuilder = BuildHasherDefault<MirrorHasher>;

impl Hasher for MirrorHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        let (int_bytes, _) = bytes.split_at(size_of::<u64>());
        self.state = u64::from_ne_bytes(int_bytes.try_into().unwrap());
    }
}
