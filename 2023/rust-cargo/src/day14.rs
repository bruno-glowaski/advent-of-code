use std::{collections::HashMap, fmt::Display, hash::Hasher};

use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum Tile {
    #[default]
    Empty = '.' as u8,
    RoundRock = 'O' as u8,
    CubeRock = '#' as u8,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8 as char)
    }
}

#[aoc_generator(day14)]
pub fn generator(src: &str) -> Grid<Tile> {
    let height = src.lines().count();
    let width = src.lines().next().unwrap().len();
    let mut grid = Grid::<Tile>::new_with_order(width, height, grid::Order::ColumnMajor);
    for (i, line) in src.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            grid[(i, j)] = FromPrimitive::from_u8(c).unwrap();
        }
    }
    grid
}

#[aoc(day14, part1)]
pub fn part1(input: &Grid<Tile>) -> usize {
    let height = input.rows();
    input
        .iter_cols()
        .map(|col| {
            col.fold((0, height, height), |(load, top, bottom), &t| match t {
                Tile::Empty => (load, top, bottom - 1),
                Tile::RoundRock => (load + top, top - 1, bottom - 1),
                Tile::CubeRock => (load, bottom - 1, bottom - 1),
            })
            .0
        })
        .sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &Grid<Tile>) -> usize {
    const NUM_CYCLES: usize = 1000000000;
    let mut grid = input.clone();
    let mut memo = HashMap::new();
    let mut i = 0;
    while i < NUM_CYCLES {
        cycle(&mut grid);
        let hash = hash_grid(&grid);
        if let Some(&previous) = memo.get(&hash) {
            let cycle_length = i - previous;
            let remainder = NUM_CYCLES - i;
            i = NUM_CYCLES - remainder % cycle_length;
        }
        memo.insert(hash, i);
        i += 1;
    }
    grid.indexed_iter()
        .filter(|(_, &c)| c == Tile::RoundRock)
        .map(|((i, _), _)| grid.rows() - i)
        .sum()
}

fn cycle(grid: &mut Grid<Tile>) {
    roll_towards(grid, north);
    roll_towards(grid, west);
    roll_towards(grid, south);
    roll_towards(grid, east);
}

pub fn hash_grid(grid: &Grid<Tile>) -> u64 {
    use std::hash::Hash;
    let mut state = std::collections::hash_map::DefaultHasher::new();
    grid.rows().hash(&mut state);
    grid.cols().hash(&mut state);
    for coords in (0..grid.rows()).cartesian_product(0..grid.cols()) {
        grid[coords].hash(&mut state);
    }
    state.finish()
}

fn roll_towards<
    TBacktrack: Fn((usize, usize), usize) -> (usize, usize),
    TScanline: Iterator<Item = (usize, usize)>,
    TScanlines: Iterator<Item = ((usize, usize), TScanline)>,
    T: FnOnce(usize, usize) -> (TBacktrack, TScanlines),
>(
    grid: &mut Grid<Tile>,
    traversal: T,
) {
    let width = grid.cols();
    let height = grid.rows();
    let (backtrack, scanlines) = traversal(width, height);
    for (end, scanline) in scanlines {
        let mut rock_count = 0;
        for coords in scanline {
            match grid[coords] {
                Tile::Empty => {}
                Tile::RoundRock => {
                    grid[coords] = Tile::Empty;
                    rock_count += 1;
                }
                Tile::CubeRock => {
                    for j in 1..=rock_count {
                        grid[backtrack(coords, j)] = Tile::RoundRock
                    }
                    rock_count = 0;
                }
            }
        }
        for j in 0..rock_count {
            grid[backtrack(end, j)] = Tile::RoundRock
        }
    }
}

fn north(
    cols: usize,
    rows: usize,
) -> (
    impl Fn((usize, usize), usize) -> (usize, usize),
    impl Iterator<Item = ((usize, usize), impl Iterator<Item = (usize, usize)>)>,
) {
    let backtrack = |(row, col), delta| (row + delta, col);
    let scanlines = (0..cols)
        .rev()
        .map(move |col| ((0, col), (0..rows).rev().map(move |row| (row, col))));
    (backtrack, scanlines)
}

fn west(
    cols: usize,
    rows: usize,
) -> (
    impl Fn((usize, usize), usize) -> (usize, usize),
    impl Iterator<Item = ((usize, usize), impl Iterator<Item = (usize, usize)>)>,
) {
    let backtrack = |(row, col), delta| (row, col + delta);
    let scanlines = (0..rows)
        .rev()
        .map(move |row| ((row, 0), (0..cols).rev().map(move |col| (row, col))));
    (backtrack, scanlines)
}

fn south(
    cols: usize,
    rows: usize,
) -> (
    impl Fn((usize, usize), usize) -> (usize, usize),
    impl Iterator<Item = ((usize, usize), impl Iterator<Item = (usize, usize)>)>,
) {
    let backtrack = |(row, col), delta| (row - delta, col);
    let scanlines =
        (0..cols).map(move |col| ((rows - 1, col), (0..rows).map(move |row| (row, col))));
    (backtrack, scanlines)
}

fn east(
    cols: usize,
    rows: usize,
) -> (
    impl Fn((usize, usize), usize) -> (usize, usize),
    impl Iterator<Item = ((usize, usize), impl Iterator<Item = (usize, usize)>)>,
) {
    let backtrack = |(row, col), delta| (row, col - delta);
    let scanlines =
        (0..rows).map(move |row| ((row, cols - 1), (0..cols).map(move |col| (row, col))));
    (backtrack, scanlines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    fn sample_input() -> Grid<Tile> {
        generator(SAMPLE)
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&sample_input()), 136);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&sample_input()), 64);
    }
}
