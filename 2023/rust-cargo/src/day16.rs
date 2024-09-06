use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::utils::grid::{parse_grid_with_u8, GridCoords};

#[derive(Debug, Default, PartialEq, Eq, FromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Tile {
    #[default]
    Empty = '.' as u8,
    MirrorNegative = '/' as u8,
    MirrorPositive = '\\' as u8,
    SplitterVertical = '|' as u8,
    SplitterHorizontal = '-' as u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    West,
    North,
    East,
    South,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Ray(GridCoords, Direction);

impl Ray {
    pub fn traverse(self, &(width, height): &(usize, usize)) -> Option<Self> {
        use Direction::*;
        let (i, j) = self.0;
        let direction = self.1;
        if (direction == North && i == 0) || (direction == West && j == 0) {
            return None;
        }
        let next_position = match direction {
            North => (i - 1, j),
            West => (i, j - 1),
            South => (i + 1, j),
            East => (i, j + 1),
        };
        if next_position.0 >= height || next_position.1 >= width {
            None
        } else {
            Some(Self(next_position, direction))
        }
    }

    pub fn mirror_negative(self) -> Self {
        use Direction::*;
        let new_direction = match self.1 {
            West => South,
            North => East,
            South => West,
            East => North,
        };
        Self(self.0, new_direction)
    }

    fn mirror_positive(self) -> Self {
        use Direction::*;
        let new_direction = match self.1 {
            West => North,
            North => West,
            South => East,
            East => South,
        };
        Self(self.0, new_direction)
    }
}

#[aoc_generator(day16)]
pub fn generator(src: &str) -> Grid<Tile> {
    parse_grid_with_u8(src, |_, c| FromPrimitive::from_u8(c).unwrap())
}

#[aoc(day16, part1)]
pub fn part1(layout: &Grid<Tile>) -> usize {
    beam_from(Ray((0, 0), Direction::East), &layout)
}

#[aoc(day16, part2)]
pub fn part2(layout: &Grid<Tile>) -> usize {
    let rows = layout.rows();
    let cols = layout.cols();
    let top_row = (0..cols).map(|j| Ray((0, j), Direction::South));
    let right_col = (0..rows).map(|i| Ray((i, cols - 1), Direction::West));
    let bottom_row = (0..cols).map(|j| Ray((rows - 1, j), Direction::North));
    let left_col = (0..rows).map(|i| Ray((i, 0), Direction::East));
    top_row
        .chain(right_col)
        .chain(bottom_row)
        .chain(left_col)
        .map(|r| beam_from(r, layout))
        .max()
        .unwrap()
}

fn beam_from(initial_ray: Ray, layout: &Grid<Tile>) -> usize {
    use Direction::*;
    use Tile::*;
    let cols = layout.cols();
    let rows = layout.rows();
    let dimensions = (cols, rows);
    let mut rays = vec![initial_ray];
    let mut energized_tiles = Grid::new(rows, cols);
    let mut previous_rays = HashSet::new();
    while let Some(ray) = rays.pop() {
        if previous_rays.contains(&ray) {
            continue;
        }
        previous_rays.insert(ray);
        energized_tiles[ray.0] = true;
        let tile = layout[ray.0];
        match (tile, ray.1) {
            (Empty, _) | (SplitterHorizontal, East | West) | (SplitterVertical, North | South) => {
                rays.extend(ray.traverse(&dimensions))
            }
            (MirrorNegative, _) => rays.extend(ray.mirror_negative().traverse(&dimensions)),
            (MirrorPositive, _) => rays.extend(ray.mirror_positive().traverse(&dimensions)),
            (SplitterHorizontal | SplitterVertical, _) => {
                rays.extend(ray.mirror_negative().traverse(&dimensions));
                rays.extend(ray.mirror_positive().traverse(&dimensions));
            }
        }
    }
    energized_tiles.iter().copied().filter(|&e| e).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SRC: &'static str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    fn sample_input() -> Grid<Tile> {
        generator(SAMPLE_SRC)
    }

    #[test]
    pub fn part1_sample() {
        assert_eq!(part1(&sample_input()), 46);
    }

    #[test]
    pub fn part2_sample() {
        assert_eq!(part2(&sample_input()), 51);
    }
}
