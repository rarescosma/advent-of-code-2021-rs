#[cfg(test)]
use crate::*;

#[test]
fn contains() {
    let c: Cube = ([0, 0, 0], [2, 2, 2]).into();
    assert!(c.contains(&Point::from([0, 0, 0])));
    assert!(c.contains(&Point::from([1, 1, 1])));
    assert!(c.contains(&Point::from([2, 2, 2])));
    assert!(!c.contains(&Point::from([-1, 0, 0])));
}

#[test]
fn canonical() {
    let c0: Cube = ([0, 0, 0], [2, 2, 2]).into();
    let c1: Cube = ([2, 2, 2], [0, 0, 0]).into();
    let c2: Cube = ([0, 2, 2], [2, 0, 0]).into();
    assert_eq!(c0, c1);
    assert_eq!(c1, c2);
    assert_eq!(c2, c0);
}

#[test]
fn plane_combos() {
    let combos: Vec<_> = PLANE_COMBOS.iter().collect();
    assert_eq!(combos.len(), 6);
}

#[test]
fn move_to_plane() {
    let c: Cube = ([0, 0, 0], [0, 0, 0]).into();
    assert_eq!(
        Some(([1, 0, 0], [1, 0, 0]).into()),
        c.project_to(Plane {
            dim: Dim::X,
            pos: 1
        })
    );
    assert_eq!(
        Some(([-1, 0, 0], [-1, 0, 0]).into()),
        c.project_to(Plane {
            dim: Dim::X,
            pos: -1
        })
    );
    assert_eq!(
        Some(([0, 1, 0], [0, 1, 0]).into()),
        c.project_to(Plane {
            dim: Dim::Y,
            pos: 1
        })
    );
    assert_eq!(
        Some(([0, -1, 0], [0, -1, 0]).into()),
        c.project_to(Plane {
            dim: Dim::Y,
            pos: -1
        })
    );
}
