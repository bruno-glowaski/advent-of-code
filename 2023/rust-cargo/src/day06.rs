use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(src: &str) -> usize {
    let mut lines = src.lines().map(|line| {
        line.split_at(line.find(':').unwrap() + 2)
            .1
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
    });
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    times
        .zip(distances)
        .map(|(time, distance)| get_possible_wins(time, distance))
        .product()
}

#[aoc(day6, part2)]
pub fn part2(src: &str) -> usize {
    let mut lines = src.lines().map(|line| {
        line.chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap()
    });
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    get_possible_wins(time, distance)
}

#[inline]
fn get_possible_wins(time: usize, distance: usize) -> usize {
    let delta = time * time - 4 * distance;
    let delta_sqrt = integer_sqrt(delta - 1);
    if time % 2 == 0 {
        1 + delta_sqrt - delta_sqrt % 2
    } else {
        delta_sqrt + delta_sqrt % 2
    }
}

#[inline]
fn integer_sqrt(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    let mut x0 = n / 2;
    let mut x1 = (x0 + n / x0) / 2;
    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + n / x0) / 2;
    }
    x0
}
