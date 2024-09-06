use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
pub struct Scratchcard<const N: usize, const M: usize> {
    winning_numbers: [usize; N],
    card_numbers: [usize; M],
}

#[aoc_generator(day4)]
pub fn input_generator(src: &str) -> [Scratchcard<10, 25>; 220] {
    src.lines()
        .map(|line| line.find(':').map(|p| &line[p + 1..]).unwrap())
        .map(|src| {
            let [winning_numbers, card_numbers] = src
                .split('|')
                .map(|src| {
                    src.split_whitespace()
                        .map(str::parse::<usize>)
                        .map(Result::unwrap)
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (
                winning_numbers.collect::<Vec<_>>().try_into().unwrap(),
                card_numbers.collect::<Vec<_>>().try_into().unwrap(),
            )
                .into()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[aoc(day4, part1)]
pub fn part1<const N: usize, const M: usize, const O: usize>(
    games: &[Scratchcard<N, M>; O],
) -> usize {
    games.into_iter().map(Scratchcard::points).sum()
}

#[aoc(day4, part2)]
pub fn part2<const N: usize, const M: usize, const O: usize>(
    games: &[Scratchcard<N, M>; O],
) -> usize {
    games
        .into_iter()
        .map(Scratchcard::win_count)
        .enumerate()
        .scan([1; O], |copy_counts, (i, win_count)| {
            let current_copy_counts = copy_counts[i];
            for copy_count in &mut copy_counts[i + 1..i + 1 + win_count] {
                *copy_count += current_copy_counts;
            }
            Some(copy_counts[i])
        })
        .sum()
}

impl<const N: usize, const M: usize> Scratchcard<N, M> {
    #[inline]
    pub fn win_count(&self) -> usize {
        self.winning_numbers
            .into_iter()
            .filter(|n| self.card_numbers.contains(n))
            .count()
    }

    #[inline]
    pub fn points(&self) -> usize {
        match self.win_count() {
            0 => 0,
            c => 2usize.pow(c as u32 - 1),
        }
    }

    #[inline]
    pub fn into_points(self) -> usize {
        self.points()
    }
}

impl<const N: usize, const M: usize> From<([usize; N], [usize; M])> for Scratchcard<N, M> {
    #[inline]
    fn from((winning_numbers, card_numbers): ([usize; N], [usize; M])) -> Self {
        Scratchcard {
            winning_numbers,
            card_numbers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let games = [([41, 48, 83, 86, 17], [83, 86, 6, 31, 17, 9, 48, 53]).into()];
        assert_eq!(part1(&games), 8);
    }

    #[test]
    fn part1_example2() {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        let games = [
            ([41, 48, 83, 86, 17], [83, 86, 6, 31, 17, 9, 48, 53]).into(),
            ([13, 32, 20, 16, 61], [61, 30, 68, 82, 17, 32, 24, 19]).into(),
            ([1, 21, 53, 59, 44], [69, 82, 63, 72, 16, 21, 14, 1]).into(),
            ([41, 92, 73, 84, 69], [59, 84, 76, 51, 58, 5, 54, 83]).into(),
            ([87, 83, 26, 28, 32], [88, 30, 70, 12, 93, 22, 82, 36]).into(),
            ([31, 18, 13, 56, 72], [74, 77, 10, 23, 35, 67, 36, 11]).into(),
        ];
        assert_eq!(part1(&games), 13);
    }

    #[test]
    fn part2_example1() {
        let games = [
            ([41, 48, 83, 86, 17], [83, 86, 6, 31, 17, 9, 48, 53]).into(),
            ([13, 32, 20, 16, 61], [61, 30, 68, 82, 17, 32, 24, 19]).into(),
            ([1, 21, 53, 59, 44], [69, 82, 63, 72, 16, 21, 14, 1]).into(),
            ([41, 92, 73, 84, 69], [59, 84, 76, 51, 58, 5, 54, 83]).into(),
            ([87, 83, 26, 28, 32], [88, 30, 70, 12, 93, 22, 82, 36]).into(),
            ([31, 18, 13, 56, 72], [74, 77, 10, 23, 35, 67, 36, 11]).into(),
        ];
        assert_eq!(part2(&games), 30);
    }
}
