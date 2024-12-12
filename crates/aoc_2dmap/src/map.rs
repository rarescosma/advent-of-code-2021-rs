use crate::pos::Pos;
use std::fmt::{Display, Formatter};

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

impl<T> Map<T> {
    pub fn new<S: Into<MapSize>>(size: S, tiles: impl Iterator<Item = T>) -> Self {
        let size = size.into();
        let tiles = tiles.collect::<Vec<T>>();
        assert_eq!(tiles.len(), (size.x * size.y) as usize);
        Self { size, tiles }
    }

    pub fn fill<S: Into<MapSize>>(size: S, default: T) -> Self
    where
        T: Clone,
    {
        let size = size.into();
        let tiles = vec![default; size.x as usize * size.y as usize];
        Self { size, tiles }
    }

    pub fn fill_default<S: Into<MapSize>>(size: S) -> Self
    where
        T: Default,
    {
        let size = size.into();
        let num_tiles = size.x as usize * size.y as usize;
        let mut tiles = Vec::<T>::with_capacity(num_tiles);
        for _ in 0..num_tiles {
            tiles.push(T::default());
        }
        Self { size, tiles }
    }

    pub fn get<P: AsRef<Pos>>(&self, pos: P) -> Option<T>
    where
        T: Clone,
    {
        self.index(*pos.as_ref())
            .map(|index| self.tiles[index].clone())
    }

    pub fn get_ref<P: AsRef<Pos>>(&self, pos: P) -> Option<&T> {
        self.index(*pos.as_ref()).map(|index| &self.tiles[index])
    }

    pub fn get_unchecked<P: AsRef<Pos>>(&self, pos: P) -> T
    where
        T: Clone,
    {
        let pos = pos.as_ref();
        self.tiles[(pos.x + pos.y * self.size.x) as usize].clone()
    }

    pub fn get_unchecked_ref<P: AsRef<Pos>>(&self, pos: P) -> &T {
        let pos = pos.as_ref();
        &self.tiles[(pos.x + pos.y * self.size.x) as usize]
    }

    pub fn get_unchecked_mut_ref<P: AsRef<Pos>>(&mut self, pos: P) -> &mut T {
        let pos = pos.as_ref();
        &mut self.tiles[(pos.x + pos.y * self.size.x) as usize]
    }

    pub fn get_row(&self, row: i32) -> impl Iterator<Item = T> + '_
    where
        T: Clone,
    {
        (0..self.size.x).map(move |x| self.get_unchecked(Pos::new(x, row)))
    }

    pub fn get_col(&self, col: i32) -> impl Iterator<Item = T> + '_
    where
        T: Clone,
    {
        (0..self.size.y).map(move |y| self.get_unchecked(Pos::new(col, y)))
    }

    pub fn get_tiles(&self) -> &Vec<T> {
        &self.tiles
    }

    pub fn set<P: AsRef<Pos>>(&mut self, pos: P, tile: T) {
        if let Some(index) = self.index(*pos.as_ref()) {
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

    #[allow(clippy::ptr_arg)]
    pub fn swap_vec(&mut self, new_tiles: &Vec<T>)
    where
        T: Copy,
    {
        assert_eq!(new_tiles.len(), (self.size.x * self.size.y) as usize);
        self.tiles.clone_from(new_tiles);
    }

    pub fn iter(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.size.y).flat_map(move |y| (0..self.size.x).map(move |x| Pos { x, y }))
    }

    pub fn within(&self, pos: Pos) -> bool {
        (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y)
    }

    fn index(&self, pos: Pos) -> Option<usize> {
        if self.within(pos) {
            Some((pos.x + pos.y * self.size.x) as _)
        } else {
            None
        }
    }
}

impl<T: Display> Display for Map<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut row = -1;
        for pos in self.iter() {
            if pos.y != row {
                row = pos.y;
                writeln!(f)?;
            }
            write!(f, "{}", self.get_unchecked_ref(pos))?;
        }
        Ok(())
    }
}
