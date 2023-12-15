use aoc_dijsktra::manually_hash;
use hashbrown::HashSet;
use std::hash::Hash;

pub fn multicycle<T: Clone + Hash, F: Fn(&mut T)>(m: T, cycle_f: F, num_cycles: usize) -> T {
    assert!(num_cycles > 0);

    let mut cache: HashSet<u64> = HashSet::with_capacity(512);
    let mut queue: Vec<(u64, T)> = Vec::with_capacity(512);
    let mut look_for = None;

    let m = m.clone();
    let mc = &mut m.clone();

    let cycle_after = (0..num_cycles).find(|_| {
        let h = manually_hash(mc);
        if cache.contains(&h) {
            look_for = Some(h);
            true
        } else {
            cache.insert(h);
            queue.push((h, mc.clone()));
            cycle_f(mc);
            false
        }
    });
    if cycle_after.is_none() {
        return m;
    }

    let cycle_after = cycle_after.unwrap();

    let prefix_length = queue
        .iter()
        .position(|&(h, _)| Some(h) == look_for)
        .unwrap();

    let wavelength = cycle_after - prefix_length;

    let idx = (num_cycles - prefix_length) % wavelength + prefix_length;
    queue.remove(idx).1
}
