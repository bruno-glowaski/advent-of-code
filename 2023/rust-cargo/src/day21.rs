use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use grid::Grid;
use num::integer::Roots;

use crate::utils::{
    grid::{
        grid_translate, grid_vector_add, parse_grid_with_u8,
        traversal::{grid_bfs, Cursor, GridBfsOptions},
        GridCoords, GridVector, ALL_S4_DIRECTIONS,
    },
    set::Set,
};

pub type Input = (Grid<bool>, GridCoords);

#[aoc_generator(day21)]
pub fn generator(src: &str) -> Input {
    let map = parse_grid_with_u8(src, |_, c| c == '#' as u8);
    let start = src
        .lines()
        .flat_map(str::chars)
        .position(|c| c == 'S')
        .unwrap();
    let cols = map.cols();
    let start = (start / cols, start % cols);
    (map, start)
}

#[aoc(day21, part1)]
pub fn part1(input: &Input) -> usize {
    part1_generic::<64>(input)
}

fn part1_generic<const N: usize>((map, start): &Input) -> usize {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct DistanceCursor<const N: usize>(GridCoords, usize);

    impl<const N: usize> Cursor<bool> for DistanceCursor<N> {
        type Neighbors<'s> = ArrayVec<Self, 4>
    where
        Self: 's;

        fn neighbors<'s: 'd, 'd>(&'s self, grid: &'d Grid<bool>) -> Self::Neighbors<'d> {
            if self.1 >= N {
                return std::iter::empty().collect();
            }
            ALL_S4_DIRECTIONS
                .clone()
                .into_iter()
                .filter_map(|d| grid_translate(self.0, d.vector(1), &(grid.rows(), grid.cols())))
                .filter_map(|c| (!grid[c]).then_some(Self(c, self.1 + 1)))
                .collect()
        }
    }

    impl<const N: usize> Set<&DistanceCursor<N>> for GridSet {
        fn insert(&mut self, value: &DistanceCursor<N>) -> bool {
            if self.0[value.0] {
                return false;
            }
            self.0[value.0] = true;
            true
        }
    }

    let parity = N % 2;
    grid_bfs(
        &map,
        DistanceCursor::<N>(*start, 0),
        0,
        |acc, &DistanceCursor(_, d)| *acc += (d % 2 == parity) as usize,
        GridBfsOptions::with_capacity_and_visited_set(
            2 * (map.rows() + map.cols()),
            GridSet(Grid::new(map.rows(), map.cols())),
        ),
    )
}

// #[aoc(day21, part2, naive)]
pub fn part2_naive(input: &Input) -> usize {
    part2_naive_impl::<26501365>(input)
}

fn part2_naive_impl<const N: usize>((map, start): &Input) -> usize {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    struct VCursor<const N: usize>(GridVector, usize);

    impl<const N: usize> Cursor<bool> for VCursor<N> {
        type Neighbors<'s> = ArrayVec<Self, 4>
        where
            Self: 's;

        fn neighbors<'s: 'd, 'd>(&'s self, grid: &'d Grid<bool>) -> Self::Neighbors<'d> {
            if self.1 >= N {
                return std::iter::empty().collect();
            }
            ALL_S4_DIRECTIONS
                .clone()
                .into_iter()
                .map(|d| grid_vector_add(self.0, d.vector(1)))
                .filter_map(|(i, j)| {
                    (!grid[(
                        i.rem_euclid(grid.rows() as isize) as usize,
                        j.rem_euclid(grid.cols() as isize) as usize,
                    )])
                        .then_some(Self((i, j), self.1 + 1))
                })
                .collect()
        }
    }

    #[derive(Debug)]
    struct VCursorSet<const N: usize>(HashSet<GridVector>);

    impl<const N: usize> Set<&VCursor<N>> for VCursorSet<N> {
        fn insert(&mut self, value: &VCursor<N>) -> bool {
            self.0.insert(value.0)
        }
    }

    let parity = N % 2;
    let start = (start.0 as isize, start.1 as isize);
    grid_bfs(
        &map,
        VCursor::<N>(start, 0),
        0,
        |acc, VCursor(_, d)| *acc += (d % 2 == parity) as usize,
        GridBfsOptions::with_capacity_and_visited_set(2 * N.sqrt(), VCursorSet(HashSet::new())),
    )
}

#[aoc(day21, part2, pattern)]
pub fn part2_pattern((map, start): &Input) -> usize {
    // This implementation uses the following additional properties observed in the input
    // as assumptions to optimize the algorithmn:
    // 1. The input size is 131x131;
    // 2. The step count is (131 * k) + 65, such that k is a natural number;
    // 3. The starting row and column are empty;
    // 4. The input can be divided into 5 regions: NW, NE, SW, SE corners and center;

    const STEPS: usize = 26501365;
    const GRID_SIDE: usize = 131;
    const N: usize = STEPS / GRID_SIDE;
    const CORNER_DISTANCE: usize = STEPS % GRID_SIDE;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct DistanceCursor(GridCoords, usize);

    impl Cursor<bool> for DistanceCursor {
        type Neighbors<'s> = ArrayVec<Self, 4>
    where
        Self: 's;

        fn neighbors<'s: 'd, 'd>(&'s self, grid: &'d Grid<bool>) -> Self::Neighbors<'d> {
            ALL_S4_DIRECTIONS
                .clone()
                .into_iter()
                .filter_map(|d| grid_translate(self.0, d.vector(1), &(grid.rows(), grid.cols())))
                .filter_map(|c| (!grid[c]).then_some(Self(c, self.1 + 1)))
                .collect()
        }
    }

    impl Set<&DistanceCursor> for GridSet {
        fn insert(&mut self, value: &DistanceCursor) -> bool {
            if self.0[value.0] {
                return false;
            }
            self.0[value.0] = true;
            true
        }
    }

    let distance_map = grid_bfs(
        &map,
        DistanceCursor(*start, 0),
        Grid::new(map.rows(), map.cols()),
        |distances, &DistanceCursor(coords, d)| distances[coords] = Some(d),
        GridBfsOptions::with_capacity_and_visited_set(
            2 * (map.rows() + map.cols()),
            GridSet(Grid::new(map.rows(), map.cols())),
        ),
    );

    let even_corners = distance_map
        .iter()
        .flatten()
        .filter(|&&d| d % 2 == 0 && d > CORNER_DISTANCE)
        .count();
    let odd_corners = distance_map
        .iter()
        .flatten()
        .filter(|&&d| d % 2 == 1 && d > CORNER_DISTANCE)
        .count();
    let even_full = distance_map
        .iter()
        .flatten()
        .filter(|&&d| d % 2 == 0)
        .count();
    let odd_full = distance_map
        .iter()
        .flatten()
        .filter(|&&d| d % 2 == 1)
        .count();
    ((N + 1) * (N + 1)) * odd_full + (N * N) * even_full - (N + 1) * odd_corners + N * even_corners
}

struct GridSet(Grid<bool>);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SRC: &'static str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    fn sample() -> Input {
        generator(SAMPLE_SRC)
    }

    #[test]
    pub fn part1_sample() {
        assert_eq!(part1_generic::<6>(&sample()), 16);
    }

    #[test]
    pub fn part2_naive_sample() {
        assert_eq!(part2_naive_impl::<6>(&sample()), 16);
        assert_eq!(part2_naive_impl::<10>(&sample()), 50);
        assert_eq!(part2_naive_impl::<50>(&sample()), 1594);
        assert_eq!(part2_naive_impl::<100>(&sample()), 6536);
        assert_eq!(part2_naive_impl::<500>(&sample()), 167004);
        assert_eq!(part2_naive_impl::<1000>(&sample()), 668697);
        assert_eq!(part2_naive_impl::<5000>(&sample()), 16733044);
    }
}
