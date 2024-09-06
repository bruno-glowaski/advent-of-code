use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::Itertools;
use nalgebra::{ArrayStorage, Matrix2, Vector2, Vector3};

pub type Vector2F = Vector2<f32>;

pub type Vector3F = Vector3<f32>;

pub struct Hailstone<V> {
    pub pos: V,
    pub vel: V,
}

#[aoc_generator(day24, part1)]
pub fn generator_part1(src: &str) -> Vec<Hailstone<Vector2F>> {
    fn parse_vec2(src: &str) -> Vector2<f32> {
        Vector2F::from_array_storage(ArrayStorage([src
            .split(", ")
            .map(str::trim)
            .map(str::parse)
            .map(Result::unwrap)
            .take(2)
            .collect::<ArrayVec<f32, 2>>()
            .into_inner()
            .unwrap()]))
    }

    src.lines()
        .map(|line| {
            line.split("@")
                .map(parse_vec2)
                .collect_tuple::<(Vector2F, Vector2F)>()
                .map(|(pos, vel)| Hailstone { pos, vel })
                .unwrap()
        })
        .collect_vec()
}

#[aoc_generator(day24, part2)]
pub fn generator_part2(src: &str) -> Vec<Hailstone<Vector3F>> {
    fn parse_vec3(src: &str) -> Vector3<f32> {
        Vector3F::from_array_storage(ArrayStorage([src
            .split(", ")
            .map(str::trim)
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<ArrayVec<f32, 3>>()
            .into_inner()
            .unwrap()]))
    }

    src.lines()
        .map(|line| {
            line.split("@")
                .map(parse_vec3)
                .collect_tuple::<(Vector3F, Vector3F)>()
                .map(|(pos, vel)| Hailstone { pos, vel })
                .unwrap()
        })
        .collect_vec()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Hailstone<Vector2F>]) -> usize {
    part1_impl::<200000000000000, 400000000000000>(input)
}

pub fn part1_impl<const MIN: usize, const MAX: usize>(input: &[Hailstone<Vector2F>]) -> usize {
    input
        .into_iter()
        .tuple_combinations()
        .filter(|(h1, h2)| {
            let a1 = Matrix2::from_columns(&[h2.pos - h1.pos, -h2.vel]).determinant();
            let a2 = Matrix2::from_columns(&[h1.vel, h2.pos - h1.pos]).determinant();
            let b = Matrix2::from_columns(&[h1.vel, -h2.vel]).determinant();

            if b.abs() < f32::EPSILON {
                return false;
            }
            let t1 = a1 / b;
            let t2 = a2 / b;
            if t1 < 0.0 || t2 < 0.0 {
                return false;
            }
            let intersection = h1.pos + t1 * h1.vel;
            intersection
                .iter()
                .all(|&v| v >= MIN as f32 && v <= MAX as f32)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SRC: &str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    fn sample_part1() -> Vec<Hailstone<Vector2F>> {
        generator_part1(SAMPLE_SRC)
    }

    #[test]
    pub fn part1_sample() {
        assert_eq!(part1_impl::<7, 27>(&sample_part1()), 2);
    }
}
