use std::fmt::Display;

use grid::Grid;
use itertools::Itertools;
use num_derive::FromPrimitive;

pub mod a_star;
pub mod traversal;

pub type GridDimensions = (usize, usize);

pub type GridCoords = (usize, usize);

pub type GridVector = (isize, isize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromPrimitive, Hash)]
pub enum S4Direction {
    West,
    North,
    East,
    South,
}

pub const ALL_S4_DIRECTIONS: [S4Direction; 4] = [
    S4Direction::West,
    S4Direction::North,
    S4Direction::East,
    S4Direction::South,
];

impl S4Direction {
    #[inline]
    pub fn vector(self, len: isize) -> GridVector {
        use S4Direction::*;
        match self {
            West => (0, -len),
            North => (-len, 0),
            East => (0, len),
            South => (len, 0),
        }
    }

    #[inline]
    pub fn reverse(self) -> S4Direction {
        use S4Direction::*;
        match self {
            West => East,
            North => South,
            East => West,
            South => North,
        }
    }

    #[inline]
    pub fn turn_left(self) -> S4Direction {
        use S4Direction::*;
        match self {
            West => South,
            North => West,
            East => North,
            South => East,
        }
    }

    #[inline]
    pub fn turn_right(self) -> S4Direction {
        use S4Direction::*;
        match self {
            West => North,
            North => East,
            East => South,
            South => West,
        }
    }
}

impl TryFrom<GridVector> for S4Direction {
    type Error = ();

    #[inline]
    fn try_from((di, dj): GridVector) -> Result<Self, Self::Error> {
        use std::cmp::Ordering::*;
        use S4Direction::*;
        Ok(match (di.cmp(&0), dj.cmp(&1)) {
            (Equal, Greater) => West,
            (Less, Equal) => North,
            (Equal, Less) => East,
            (Greater, Equal) => South,
            _ => return Err(()),
        })
    }
}

#[inline]
pub fn grid_translate(
    (i0, j0): GridCoords,
    (di, dj): GridVector,
    &(rows, cols): &GridDimensions,
) -> Option<GridCoords> {
    let i1 = i0.checked_add_signed(di)?;
    let j1 = j0.checked_add_signed(dj)?;
    if i1 >= rows || j1 >= cols {
        None
    } else {
        Some((i1, j1))
    }
}

#[inline]
pub fn grid_vector_add((i0, j0): GridVector, (i1, j1): GridVector) -> GridVector {
    (i0 + i1, j0 + j1)
}

#[inline]
pub fn grid_distance((i0, j0): GridCoords, (i1, j1): GridCoords) -> usize {
    i1.abs_diff(i0) + j1.abs_diff(j0)
}

#[inline]
pub fn parse_grid_with_u8<'s, T, F>(src: &'s str, mut f: F) -> Grid<T>
where
    T: 's,
    F: FnMut(GridCoords, u8) -> T,
{
    let cols = src.lines().next().unwrap().len();
    let buffer = src
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.bytes().enumerate().map(move |(j, c)| ((i, j), c)))
        .map(|(p, c)| f(p, c))
        .collect_vec();
    Grid::from_vec(buffer, cols)
}

#[inline]
pub fn print_grid<T: Display>(grid: &Grid<T>) {
    for row in grid.iter_rows() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[inline]
pub fn print_grid_with<T, U: Display, F: Fn(GridCoords, &T) -> U>(grid: &Grid<T>, f: F) {
    for (i, row) in grid.iter_rows().enumerate() {
        for (j, c) in row.enumerate() {
            print!("{}", f((i, j), c));
        }
        println!();
    }
}
