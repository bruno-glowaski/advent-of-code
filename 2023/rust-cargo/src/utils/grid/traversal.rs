use std::collections::VecDeque;

use grid::Grid;

use crate::utils::set::Set;

pub trait Cursor<T>: PartialEq + Eq {
    type Neighbors<'s>: IntoIterator<Item = Self> + 's
    where
        Self: 's,
        T: 's;

    fn neighbors<'s: 'd, 'd>(&'s self, grid: &'d Grid<T>) -> Self::Neighbors<'d>;
}

pub struct GridBfsOptions<S> {
    capacity: usize,
    visited_set: S,
}

impl<S> GridBfsOptions<S> {
    pub fn with_capacity_and_visited_set(capacity: usize, visited_set: S) -> Self {
        Self {
            capacity,
            visited_set,
        }
    }
}

#[inline]
pub fn grid_bfs<T, C, R, F, S>(
    grid: &Grid<T>,
    start: C,
    mut initial_value: R,
    mut f: F,
    GridBfsOptions {
        capacity,
        mut visited_set,
    }: GridBfsOptions<S>,
) -> R
where
    C: Cursor<T>,
    F: FnMut(&mut R, &C),
    S: for<'a> Set<&'a C>,
{
    let mut queue = VecDeque::with_capacity(capacity);
    queue.push_back(start);
    while let Some(cursor) = queue.pop_front() {
        if !visited_set.insert(&cursor) {
            continue;
        }
        f(&mut initial_value, &cursor);
        queue.extend(cursor.neighbors(&grid));
    }
    initial_value
}
