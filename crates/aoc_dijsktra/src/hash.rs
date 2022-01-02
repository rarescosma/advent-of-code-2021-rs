/// We want to keep track of the costs of visited GameStates, but
/// inserting into `HashMap`s requires owned values, so we use a `KeyMap`
/// instead which uses `manually_hash`, then inserts the resulting hash
/// in a regular hash table set up to use a `MirrorHasher` which simply
/// mirrors back the `u64` produced by `manually_hash`.
use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{Hash, Hasher};

use ahash::RandomState;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHER_BUILDER: RandomState = RandomState::new();
}

pub(crate) struct KeyMap {
    hashmap: HashMap<u64, usize, MirrorHashBuilder>,
}

impl KeyMap {
    pub fn entry<H: Hash>(&mut self, k: &H) -> Entry<u64, usize, MirrorHashBuilder> {
        self.hashmap.entry(manually_hash(k))
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            hashmap: HashMap::with_capacity_and_hasher(1024, MirrorHashBuilder::default()),
        }
    }
}

fn manually_hash<H: Hash>(state: &H) -> u64 {
    let mut hasher = HASHER_BUILDER.build_hasher();
    state.hash(&mut hasher);
    hasher.finish()
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
        let (int_bytes, _) = bytes.split_at(std::mem::size_of::<u64>());
        self.state = u64::from_ne_bytes(int_bytes.try_into().unwrap());
    }
}
