use aoc_runner_derive::aoc;
use num::Zero;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i32 {
    let sequences = input.lines().map(|l| {
        l.split_whitespace()
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    });

    sequences
        .into_iter()
        .map(|s| {
            let mut derivatives = vec![s];
            while let Some(base) = derivatives
                .last()
                .and_then(|s| (!s.iter().all(|i| i.is_zero())).then_some(s))
            {
                derivatives.push(
                    (0..base.len() - 1)
                        .into_iter()
                        .map(|i| base[i + 1] - base[i])
                        .collect(),
                );
            }
            derivatives
                .into_iter()
                .rev()
                .fold(0, |prev, d| d.last().unwrap() + prev)
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i32 {
    let sequences = input.lines().map(|l| {
        l.split_whitespace()
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    });

    sequences
        .into_iter()
        .map(|s| {
            let mut derivatives = vec![s];
            while let Some(base) = derivatives
                .last()
                .and_then(|s| (!s.iter().all(|i| i.is_zero())).then_some(s))
            {
                derivatives.push(
                    (0..base.len() - 1)
                        .into_iter()
                        .map(|i| base[i + 1] - base[i])
                        .collect(),
                );
            }
            derivatives.into_iter().rev().fold(0, |prev, d| d[0] - prev)
        })
        .sum()
}
