use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;

use crate::utils::grid::{
    a_star::{grid_a_star, Cursor},
    grid_distance, grid_translate, parse_grid_with_u8, GridCoords, GridDimensions, S4Direction,
    ALL_S4_DIRECTIONS,
};

#[aoc_generator(day17)]
pub fn generator(src: &str) -> Grid<u8> {
    parse_grid_with_u8(src, |_, c| c - '0' as u8)
}

#[aoc(day17, part1)]
pub fn part1(input: &Grid<u8>) -> usize {
    let dimensions = (input.rows(), input.cols());
    let start = (0, 0);
    let start_cursors = ALL_S4_DIRECTIONS.clone().into_iter().flat_map(|d| {
        grid_translate(start, d.vector(1), &dimensions)
            .map(|coords| CrucibleCursor::<0, 3>::new(coords, d, 0))
    });
    let end = (dimensions.0 - 1, dimensions.1 - 1);
    let end_fn = |cursor: &CrucibleCursor<0, 3>| cursor.coords == end;
    grid_a_star(&input, start_cursors, end_fn)
}

#[aoc(day17, part2)]
pub fn part2(input: &Grid<u8>) -> usize {
    let dimensions = (input.rows(), input.cols());
    let start = (0, 0);
    let start_cursors = ALL_S4_DIRECTIONS.clone().into_iter().flat_map(|d| {
        grid_translate(start, d.vector(1), &dimensions)
            .map(|coords| CrucibleCursor::<4, 10>::new(coords, d, 0))
    });
    let end = (dimensions.0 - 1, dimensions.1 - 1);
    let end_fn = |cursor: &CrucibleCursor<4, 10>| cursor.coords == end;
    grid_a_star(&input, start_cursors, end_fn)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct CrucibleCursor<const M: u8, const N: u8> {
    coords: GridCoords,
    direction: S4Direction,
    steps_walked: u8,
}

impl<const M: u8, const N: u8> CrucibleCursor<M, N> {
    fn new(coords: GridCoords, direction: S4Direction, steps_walked: u8) -> Self {
        Self {
            coords,
            direction,
            steps_walked,
        }
    }

    fn walk_straight(&self, dimensions: &GridDimensions) -> Option<Self> {
        let steps_walked = self.steps_walked + 1;
        if steps_walked < N {
            let coords = grid_translate(self.coords, self.direction.vector(1), dimensions)?;
            Some(Self::new(coords, self.direction, steps_walked))
        } else {
            None
        }
    }

    fn walk_left(&self, dimensions: &GridDimensions) -> Option<Self> {
        if self.steps_walked + 1 >= M {
            let d = self.direction.turn_left();
            Some(Self::new(
                grid_translate(self.coords, d.vector(1), dimensions)?,
                d,
                0,
            ))
        } else {
            None
        }
    }

    fn walk_right(&self, dimensions: &GridDimensions) -> Option<Self> {
        if self.steps_walked + 1 >= M {
            let d = self.direction.turn_right();
            Some(Self::new(
                grid_translate(self.coords, d.vector(1), dimensions)?,
                d,
                0,
            ))
        } else {
            None
        }
    }
}

impl<const M: u8, const N: u8> Cursor<u8> for CrucibleCursor<M, N> {
    type Neighbors<'s> =  std::iter::Flatten<<[Option<Self>;3] as IntoIterator>::IntoIter>
    where
        Self: 's;

    fn neighbors<'s: 'd, 'd>(&'s self, dimensions: &'d GridDimensions) -> Self::Neighbors<'d> {
        [
            self.walk_left(dimensions),
            self.walk_right(dimensions),
            self.walk_straight(dimensions),
        ]
        .into_iter()
        .flatten()
    }

    fn h(&self, grid: &Grid<u8>) -> usize {
        let target = (grid.rows() - 1, grid.cols() - 1);
        grid_distance(self.coords, target) + M.saturating_sub(self.steps_walked) as usize
    }

    fn c(&self, grid: &Grid<u8>) -> usize {
        grid[self.coords] as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    fn sample_input() -> Grid<u8> {
        generator(SAMPLE)
    }

    #[test]
    pub fn part1_sample() {
        assert_eq!(part1(&sample_input()), 102);
    }

    #[test]
    pub fn part2_sample() {
        assert_eq!(part2(&sample_input()), 94);
    }
}
