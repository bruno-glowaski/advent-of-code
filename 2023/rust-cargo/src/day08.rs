use std::collections::{hash_map::RandomState, HashMap};

use aoc_runner_derive::aoc;
use num::Integer;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut instructions = lines.next().unwrap().chars().cycle();
    lines.next().unwrap();
    let network_map = lines
        .map(|l| (&l[..3], (&l[7..10], &l[12..15])))
        .collect::<HashMap<_, _, RandomState>>();
    let mut count = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        count += 1;
        let instruction = instructions.next().unwrap();
        let paths = network_map.get(current_node).unwrap();
        current_node = match instruction {
            'L' => paths.0,
            'R' => paths.1,
            _ => unreachable!(),
        }
    }
    count
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    lines.next().unwrap();
    let network_map = lines
        .map(|l| (&l[..3], (&l[7..10], &l[12..15])))
        .collect::<HashMap<_, _, RandomState>>();

    let starting_nodes = network_map
        .keys()
        .filter(|n| n.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();

    let loop_lengths = starting_nodes.iter().copied().map(|start| {
        let mut current = start;
        let mut count = 0;
        let mut instructions = instructions.clone();
        loop {
            count += 1;
            let instruction = instructions.next().unwrap();
            let paths = network_map.get(current).unwrap();
            current = match instruction {
                'L' => paths.0,
                'R' => paths.1,
                _ => unreachable!(),
            };
            if current.ends_with('Z') {
                break count;
            }
        }
    });

    loop_lengths.reduce(|acc, i| acc.lcm(&i)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_sample() {
        assert_eq!(
            part2(
                r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
