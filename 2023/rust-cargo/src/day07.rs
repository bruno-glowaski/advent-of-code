use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

const HAND_VALUE_DIGIT_LENGTH: usize = 16;

pub type Hand = [Card; 5];

#[derive(Debug)]
pub enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::N9,
            '8' => Card::N8,
            '7' => Card::N7,
            '6' => Card::N6,
            '5' => Card::N5,
            '4' => Card::N4,
            '3' => Card::N3,
            '2' => Card::N2,
            _ => panic!(),
        }
    }
}

#[aoc_generator(day7)]
pub fn generator(src: &str) -> Vec<(Hand, usize)> {
    src.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let cards = iter
                .next()
                .unwrap()
                .chars()
                .map(char::into)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let bid = iter.next().unwrap().parse().unwrap();
            (cards, bid)
        })
        .map(Into::into)
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[(Hand, usize)]) -> usize {
    fn get_hand_kind(hand: &Hand) -> HandKind {
        let mut bucket = [0; HAND_VALUE_DIGIT_LENGTH];
        for card in hand {
            bucket[*card as usize] += 1;
        }
        let card_type_count = bucket.into_iter().filter(|c| *c > 0).count();
        let max_freq = bucket.into_iter().max().unwrap();
        match card_type_count {
            0 | 1 => HandKind::FiveOfAKind,
            2 if max_freq == 4 => HandKind::FourOfAKind,
            2 => HandKind::FullHouse,
            3 if max_freq == 3 => HandKind::ThreeOfAKind,
            3 => HandKind::TwoPair,
            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => unreachable!(),
        }
    }

    let mut hands = input.into_iter().collect::<Vec<_>>();
    hands.sort_by_key(|(a, _)| get_hand_value(a, get_hand_kind, |c| c as usize));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (1 + i) * bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[(Hand, usize)]) -> usize {
    fn get_hand_kind(hand: &Hand) -> HandKind {
        let mut bucket = [0; HAND_VALUE_DIGIT_LENGTH];
        for card in hand {
            bucket[*card as usize] += 1;
        }
        let last_index = bucket.len() - 1;
        let joker_index = Card::J as usize;
        bucket.swap(joker_index, last_index);
        let joker_index = last_index;
        let joker_count = bucket[joker_index];
        let bucket = &bucket[..joker_index];
        let card_type_count = bucket.into_iter().filter(|c| **c > 0).count();
        let max_freq = bucket.into_iter().max().copied().unwrap() + joker_count;
        match card_type_count {
            0 | 1 => HandKind::FiveOfAKind,
            2 if max_freq == 4 => HandKind::FourOfAKind,
            2 => HandKind::FullHouse,
            3 if max_freq == 3 => HandKind::ThreeOfAKind,
            3 => HandKind::TwoPair,
            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => unreachable!(),
        }
    }

    fn get_card_value(card: Card) -> usize {
        use std::cmp::Ordering::*;

        match card.cmp(&Card::J) {
            Equal => 0,
            Greater => card as usize,
            Less => card as usize + 1,
        }
    }

    let mut hands = input.into_iter().collect::<Vec<_>>();
    hands.sort_by_key(|(a, _)| get_hand_value(a, get_hand_kind, get_card_value));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (1 + i) * bid)
        .sum()
}

fn get_hand_value<K: Fn(&Hand) -> HandKind, C: Fn(Card) -> usize>(
    hand: &Hand,
    get_hand_kind: K,
    get_card_value: C,
) -> usize {
    hand.iter()
        .copied()
        .map(get_card_value)
        .rev()
        .chain(std::iter::once(get_hand_kind(hand) as usize))
        .enumerate()
        .map(|(i, d)| HAND_VALUE_DIGIT_LENGTH.pow(i as u32) * d)
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> Vec<(Hand, usize)> {
        generator(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&sample()), 6440);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&sample()), 5905);
    }
}
