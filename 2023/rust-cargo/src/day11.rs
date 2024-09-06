use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (usize, usize);

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(points: &Vec<Point>) -> usize {
    let mut points = points.clone();
    expand_universe(&mut points, 2);
    (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| manhattan_distance(&points[i], &points[j]))
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(points: &Vec<Point>) -> usize {
    let mut points = points.clone();
    expand_universe(&mut points, 1000000);
    sum_of_distances(&points)
}

fn expand_universe(points: &mut [Point], times: usize) {
    let times = times - 1;
    points.sort_by_key(|(_, y)| *y);
    let mut previous_y = 0;
    let mut empty_rows = 0;
    for (_, y) in points.iter_mut() {
        let y_diff = *y - previous_y;
        previous_y = *y;
        empty_rows += y_diff.saturating_sub(1);
        *y += empty_rows * times;
    }
    points.sort_by_key(|(x, _)| *x);
    let mut previous_x = 0;
    let mut empty_columns = 0;
    for (x, _) in points.iter_mut() {
        let x_diff = *x - previous_x;
        previous_x = *x;
        empty_columns += x_diff.saturating_sub(1);
        *x += empty_columns * times;
    }
}

fn sum_of_distances(points: &[Point]) -> usize {
    (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| manhattan_distance(&points[i], &points[j]))
        .sum()
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn part1_sample() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part1(&generator(input)), 374);
    }

    #[test]
    pub fn expand_universe_sample() {
        let sample_points = vec![
            (3, 0),
            (7, 1),
            (0, 2),
            (6, 4),
            (1, 5),
            (9, 6),
            (7, 8),
            (0, 9),
            (4, 9),
        ];

        let mut points = sample_points.clone();
        expand_universe(&mut points, 2);
        assert_eq!(sum_of_distances(&points), 374);

        let mut points = sample_points.clone();
        expand_universe(&mut points, 10);
        assert_eq!(sum_of_distances(&points), 1030);

        let mut points = sample_points.clone();
        expand_universe(&mut points, 100);
        assert_eq!(sum_of_distances(&points), 8410);
    }
}
