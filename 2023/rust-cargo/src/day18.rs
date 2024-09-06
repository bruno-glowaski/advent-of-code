use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(match s {
            "R" => Right,
            "D" => Down,
            "L" => Left,
            "U" => Up,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    delta: isize,
}

#[aoc_generator(day18, part1)]
pub fn part1(src: &str) -> Vec<Command> {
    src.lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let direction = iter.next().unwrap().parse().unwrap();
            let delta = iter.next().unwrap().parse().unwrap();
            Command { direction, delta }
        })
        .collect()
}

#[aoc_generator(day18, part2)]
pub fn part2(src: &str) -> Vec<Command> {
    src.lines()
        .map(|l| {
            let code = l.split_whitespace().skip(2).next().unwrap();
            let delta = isize::from_str_radix(&code[2..7], 16).unwrap();
            let direction = FromPrimitive::from_u8(code[7..].as_bytes()[0] - '0' as u8).unwrap();
            Command { direction, delta }
        })
        .collect()
}

#[aoc(day18, part1)]
#[aoc(day18, part2)]
pub fn solve(input: &[Command]) -> usize {
    (input
        .into_iter()
        .fold((0, 0), |(area, y), Command { direction, delta }| {
            use Direction::*;
            let determinant = match direction {
                Left => -2 * y * delta,
                Right => 2 * y * delta,
                _ => 0,
            };
            let delta_y = match direction {
                Up => *delta,
                Down => -*delta,
                _ => 0,
            };
            (area + determinant + delta, y + delta_y)
        })
        .0
        / 2
        + 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_sample() {
        assert_eq!(solve(&part1(SAMPLE)), 62);
    }
}
