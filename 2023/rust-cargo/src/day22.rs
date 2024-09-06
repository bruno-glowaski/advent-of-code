use std::{collections::VecDeque, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use grid::Grid;
use itertools::Itertools;
use nalgebra::{SimdPartialOrd, Vector3};

pub type IVector3 = Vector3<isize>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AABB {
    pub min: IVector3,
    pub max: IVector3,
}

#[aoc_generator(day22)]
pub fn generator(src: &str) -> Vec<AABB> {
    src.lines().flat_map(AABB::from_str).collect()
}

#[aoc(day22, part1, naive)]
pub fn part1_naive(input: &[AABB]) -> usize {
    let mut falling_blocks = input.to_owned();
    falling_blocks.sort_unstable_by_key(|b| b.min.z);
    let mut critical_blocks = vec![false; input.len()];
    let mut settled_blocks = Vec::with_capacity(falling_blocks.len());
    for mut block in falling_blocks {
        let highest_blocks_bellow = settled_blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| block.xy_intersects(b))
            .max_set_by_key(|(_, b)| b.max.z);
        let target_z = highest_blocks_bellow.get(0).map_or(0, |(_, b)| b.max.z) + 1;
        let delta = (target_z - block.min.z) * IVector3::z_axis().into_inner();
        block.translate_by(delta);
        if highest_blocks_bellow.len() == 1 {
            for (i, _) in highest_blocks_bellow.into_iter() {
                critical_blocks[i] = true;
            }
        }
        settled_blocks.push(block);
    }
    settled_blocks.len() - critical_blocks.into_iter().filter(|&c| c).count()
}

#[aoc(day22, part2, naive)]
pub fn part2_naive(input: &[AABB]) -> usize {
    let mut blocks = input.to_owned();
    blocks.sort_unstable_by_key(|b| b.min.z);
    let mut height_map = Grid::new(10, 10);
    for block in &mut blocks {
        let floor_z = block
            .xy_points()
            .map(|(x, y)| height_map[(x as usize, y as usize)])
            .max()
            .unwrap_or(0);
        let delta = (floor_z + 1 - block.min.z) * IVector3::z_axis().into_inner();
        block.translate_by(delta);
        for (x, y) in block.xy_points() {
            height_map[(x as usize, y as usize)] = block.max.z;
        }
    }
    blocks.sort_unstable_by_key(|b| b.min.z);
    let block_count = blocks.len();
    let mut falling = vec![false; block_count];
    let blocks_above = |i: usize| {
        let block = &blocks[i];
        blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| block.xy_intersects(b) && b.min.z == block.max.z + 1)
            .map(|(i, _)| i)
    };
    let blocks_bellow = |i: usize| {
        let block = &blocks[i];
        blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| block.xy_intersects(b) && b.max.z == block.min.z - 1)
            .map(|(i, _)| i)
    };
    let mut acc = 0;
    let mut queue = VecDeque::with_capacity(block_count);
    for i in 0..block_count {
        falling.fill(false);
        falling[i] = true;
        queue.push_back(i);
        while let Some(j) = queue.pop_front() {
            for k in blocks_above(j) {
                if !falling[k] && blocks_bellow(k).all(|l| falling[l]) {
                    queue.push_back(k);
                    falling[k] = true;
                    acc += 1;
                }
            }
        }
    }
    acc
}

fn parse_vec3(src: &str) -> Result<IVector3, ()> {
    let [x, y, z] = src
        .split(',')
        .map(str::parse)
        .flatten()
        .collect::<ArrayVec<isize, 3>>()
        .into_inner()
        .or(Err(()))?;
    Ok(Vector3::new(x, y, z))
}

impl AABB {
    pub fn translate_by(&mut self, delta: IVector3) {
        self.min += delta;
        self.max += delta;
    }

    pub fn xy_intersects(&self, other: &Self) -> bool {
        (self.min.x <= other.max.x && self.max.x >= other.min.x)
            && (self.min.y <= other.max.y && self.max.y >= other.min.y)
    }

    pub fn xy_points(&self) -> impl Iterator<Item = (isize, isize)> {
        (self.min.x..=self.max.x).cartesian_product((self.min.y..=self.max.y).into_iter())
    }
}

impl FromStr for AABB {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [v0, v1] = s
            .split('~')
            .flat_map(parse_vec3)
            .collect::<ArrayVec<IVector3, 2>>()
            .into_inner()
            .or(Err(()))?;
        let min = v0.simd_min(v1);
        let max = v0.simd_max(v1);
        Ok(Self { min, max })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SRC: &'static str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    fn sample() -> Vec<AABB> {
        generator(SAMPLE_SRC)
    }

    #[test]
    pub fn part1_naive_sample() {
        assert_eq!(part1_naive(&sample()), 5);
    }

    #[test]
    pub fn part2_naive_sample() {
        assert_eq!(part2_naive(&sample()), 7);
    }
}
