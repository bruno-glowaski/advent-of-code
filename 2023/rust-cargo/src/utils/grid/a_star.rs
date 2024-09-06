use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    hash::Hash,
};

use grid::Grid;

use crate::utils::grid::GridDimensions;

pub trait Cursor<T>: Debug + PartialEq + Eq + Clone + Copy + Hash {
    type Neighbors<'s>: IntoIterator<Item = Self> + 's
    where
        Self: 's;

    fn neighbors<'s: 'd, 'd>(&'s self, dimensions: &'d GridDimensions) -> Self::Neighbors<'d>;

    fn h(&self, grid: &Grid<T>) -> usize;
    fn c(&self, grid: &Grid<T>) -> usize;
}

pub fn grid_a_star<T, C, I, H, E>(grid: &Grid<T>, start_cursors: I, end_fn: E) -> usize
where
    C: Cursor<T>,
    I: IntoIterator<IntoIter = H>,
    H: Iterator<Item = C> + Clone,
    E: Fn(&C) -> bool,
{
    let dimensions = (grid.rows(), grid.cols());
    let start_cursors = start_cursors
        .into_iter()
        .map(|cursor| (cursor, cursor.c(&grid), cursor.h(&grid)));
    let mut tags = start_cursors
        .clone()
        .map(|(cursor, c, h)| (cursor, CursorTag::new(None, c, c + h, true)))
        .collect::<HashMap<_, _>>();
    let mut queue = start_cursors
        .map(|(cursor, c, _)| QueuedCursor(cursor, c))
        .collect::<BinaryHeap<_>>();
    while let Some(QueuedCursor(current, _)) = queue.pop() {
        if end_fn(&current) {
            return tags[&current].g_score;
        }
        let current_tag = &tags[&current];
        let current_g_score = current_tag.g_score;
        for neighbor in current.neighbors(&dimensions) {
            let neighbor_tag =
                tags.entry(neighbor)
                    .or_insert(CursorTag::new(None, usize::MAX, usize::MAX, false));
            let tentative_g_score = current_g_score + neighbor.c(&grid);
            if tentative_g_score < neighbor_tag.g_score {
                neighbor_tag.previous = Some(current);
                neighbor_tag.g_score = tentative_g_score;
                neighbor_tag.f_score = tentative_g_score + neighbor.h(&grid);
                if !neighbor_tag.visited {
                    neighbor_tag.visited = true;
                    queue.push(QueuedCursor(neighbor, neighbor_tag.f_score))
                }
            }
        }
    }
    unreachable!()
}

#[derive(Debug, PartialEq, Eq)]
struct QueuedCursor<C>(C, usize);

impl<C: PartialEq + Eq> PartialOrd for QueuedCursor<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<C: PartialEq + Eq> Ord for QueuedCursor<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

#[derive(Debug, Clone, Copy)]
struct CursorTag<C> {
    previous: Option<C>,
    g_score: usize,
    f_score: usize,
    visited: bool,
}

impl<C> CursorTag<C> {
    fn new(previous: Option<C>, g_score: usize, f_score: usize, visited: bool) -> Self {
        Self {
            previous,
            g_score,
            f_score,
            visited,
        }
    }
}
