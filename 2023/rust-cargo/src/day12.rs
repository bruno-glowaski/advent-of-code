use std::iter::repeat;

use aoc_runner_derive::aoc;
use itertools::Itertools;
const OPERATIONAL_CHAR: char = '.';
const DAMAGED_CHAR: char = '#';
const UNKNOWN_CHAR: char = '?';

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            (
                iter.next().unwrap().chars().collect::<Vec<_>>(),
                iter.next()
                    .unwrap()
                    .split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(row, groups)| MonogramSolver::new(row, groups).solve())
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let row = iter.next().unwrap().chars();
            let groups = iter
                .next()
                .unwrap()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap);

            (
                repeat(row.clone().chain(std::iter::once(UNKNOWN_CHAR)))
                    .take(4)
                    .flatten()
                    .chain(row)
                    .collect::<Vec<_>>(),
                repeat(groups).take(5).flatten().collect::<Vec<_>>(),
            )
        })
        .map(|(row, groups)| MonogramSolver::new(row, groups).solve())
        .sum()
}

struct MonogramSolver {
    row: Vec<char>,
    groups: Vec<usize>,
}

impl MonogramSolver {
    pub fn new(row: Vec<char>, groups: Vec<usize>) -> Self {
        Self { row, groups }
    }

    pub fn solve(&self) -> usize {
        let mut memo = vec![None; self.row.len() * self.groups.len()];
        self.row
            .iter()
            .take_while_inclusive(|c| **c != DAMAGED_CHAR)
            .enumerate()
            .map(|(i, _)| self.memoised_recursive(i, 0, &mut memo))
            .sum()
    }

    fn memoised_recursive(
        &self,
        position: usize,
        group_id: usize,
        memo: &mut [Option<usize>],
    ) -> usize {
        let id = position + group_id * self.row.len();
        if let Some(r) = memo[id] {
            r
        } else {
            let n = self.recursive(position, group_id, memo);
            memo[id] = Some(n);
            n
        }
    }

    fn recursive(&self, start: usize, group_id: usize, memo: &mut [Option<usize>]) -> usize {
        let end = start + self.groups[group_id];
        let row = self.row.as_slice();

        if end > row.len()
            || row[start..end].contains(&OPERATIONAL_CHAR)
            || (end < row.len() && row[end] == DAMAGED_CHAR)
        {
            return 0;
        }
        if group_id < self.groups.len() - 1 {
            return (end + 1..row.len())
                .into_iter()
                .take_while_inclusive(|i| row[*i] != DAMAGED_CHAR)
                .map(|i| self.memoised_recursive(i, group_id + 1, memo))
                .sum();
        }
        if row[end..].contains(&DAMAGED_CHAR) {
            return 0;
        }
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = r"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";
        assert_eq!(part1(input), 6);

        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn part2_sample() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part2(input), 525152);
    }
}
