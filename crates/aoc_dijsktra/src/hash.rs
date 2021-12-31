use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{Hash, Hasher};

use ahash::RandomState;
use hashbrown::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHER_BUILDER: RandomState = RandomState::new();
}

/// We want to keep track of the costs of visited GameStates, but
/// inserting into `HashMap`s requires owned values, so we `manually_hash`
/// instead to avoid the extra cloning (and the `Clone` bound).
pub(crate) fn manually_hash<H: Hash>(state: &H) -> u64 {
    let mut hasher = HASHER_BUILDER.build_hasher();
    state.hash(&mut hasher);
    hasher.finish()
}

pub(crate) fn build_hashmap() -> HashMap<u64, usize, MirrorHashBuilder> {
    HashMap::with_capacity_and_hasher(1024, MirrorHashBuilder::default())
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
