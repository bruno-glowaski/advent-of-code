use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::vec::matrix_transpose;

pub struct Pattern {
    row_major: Vec<char>,
    column_major: Vec<char>,
    width: usize,
    height: usize,
}

#[aoc_generator(day13)]
pub fn generator(src: &str) -> Vec<Pattern> {
    src.split("\n\n")
        .map(|l| {
            let iter = l.lines();
            let width = iter.clone().next().unwrap().len();
            let height = iter.clone().count();
            let row_major = iter.flat_map(str::chars).collect::<Vec<_>>();
            let column_major = matrix_transpose(&row_major, width, height);
            Pattern {
                row_major,
                column_major,
                width,
                height,
            }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[Pattern]) -> usize {
    input
        .into_iter()
        .map(
            |Pattern {
                 row_major,
                 column_major,
                 width,
                 height,
             }| {
                let width = *width;
                let height = *height;
                let h_reflect = (1..width).find(|j| {
                    let hl = usize::min(*j, width - j);
                    (0..height)
                        .all(|i| is_palindrome(&row_major[i * width + j - hl..i * width + j + hl]))
                });
                let v_reflect = (1..height)
                    .find(|j| {
                        let hl = usize::min(*j, height - j);
                        (0..width).all(|i| {
                            is_palindrome(&column_major[i * height + j - hl..i * height + j + hl])
                        })
                    })
                    .map(|j| j * 100);
                h_reflect.or(v_reflect).unwrap()
            },
        )
        .map(|s| s)
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[Pattern]) -> usize {
    input
        .into_iter()
        .map(
            |Pattern {
                 row_major,
                 column_major,
                 width,
                 height,
             }| {
                let width = *width;
                let height = *height;
                let h_reflect = (1..width).find(|j| {
                    let hl = usize::min(*j, width - j);
                    (0..height)
                        .map(|i| {
                            maybe_palindrome(&row_major[i * width + j - hl..i * width + j + hl])
                        })
                        .sum::<usize>()
                        == 1
                });
                let v_reflect = (1..height)
                    .find(|j| {
                        let hl = usize::min(*j, height - j);
                        (0..width)
                            .map(|i| {
                                maybe_palindrome(
                                    &column_major[i * height + j - hl..i * height + j + hl],
                                )
                            })
                            .sum::<usize>()
                            == 1
                    })
                    .map(|j| j * 100);
                h_reflect.or(v_reflect).unwrap()
            },
        )
        .sum()
}

fn maybe_palindrome(target: &[char]) -> usize {
    (0..target.len() / 2)
        .filter(|&i| target[i] != target[target.len() - i - 1])
        .count()
}

fn is_palindrome(target: &[char]) -> bool {
    (0..target.len()).all(|i| target[i] == target[target.len() - i - 1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = generator(
            r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(part1(&input), 405);
    }

    #[test]
    fn part2_sample() {
        let input = generator(
            r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(part2(&input), 400);
    }
}
