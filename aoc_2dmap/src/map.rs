use crate::pos::Pos;
use std::fmt::{Debug, Display, Formatter};

pub type MapSize = Pos;

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Map<T> {
    pub size: MapSize,
    tiles: Vec<T>,
}

impl<T: Clone> Clone for Map<T> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            tiles: self.tiles.to_owned(),
        }
    }
}

impl<T: Copy> Map<T> {
    pub fn new(size: Pos, tiles: Vec<T>) -> Self {
        Self { size, tiles }
    }

    pub fn get(&self, pos: Pos) -> Option<T> {
        self.index(pos).map(|index| self.tiles[index])
    }

    pub fn get_col(&self, col: i32) -> Option<Vec<T>> {
        if (0..self.size.x).contains(&col) {
            return Some(
                (0..self.size.y)
                    .flat_map(|y| self.get((col, y).into()))
                    .collect(),
            );
        }
        None
    }

    pub fn set(&mut self, pos: Pos, tile: T) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile;
        }
    }

    pub fn swap(&mut self, p0: Pos, p1: Pos) {
        if let Some(i0) = self.index(p0) {
            if let Some(i1) = self.index(p1) {
                self.tiles.swap(i0, i1)
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.size.y)
            .map(move |y| (0..self.size.x).map(move |x| Pos { x, y }))
            .flatten()
    }

    fn index(&self, pos: Pos) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.x + pos.y * self.size.x) as _)
        } else {
            None
        }
    }
}

impl<T: Clone + Default> Map<T> {
    pub fn fill_default(size: MapSize) -> Self {
        Self::fill(size, T::default())
    }
}

impl<T: Clone> Map<T> {
    pub fn fill(size: MapSize, default: T) -> Self {
        let tiles = vec![default; size.x as usize * size.y as usize];
        Self { size, tiles }
    }
}

impl<T> Debug for Map<T>
where
    T: Debug + Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{}", self.get(Pos { x, y }).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
