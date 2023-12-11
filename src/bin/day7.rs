use std::{cmp::Ordering, collections::HashMap};

use aoc_2023::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn parse_hand(hand: &str) -> Hand {
    let counts = hand.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
        acc
    });

    let mut hand = counts
        .into_values()
        .fold(([0; 5], 0), |(mut arr, i), value| {
            arr[i] = value;
            (arr, i + 1)
        })
        .0;
    hand.sort_by(|a, b| b.cmp(a));

    match hand {
        [5, _, _, _, _] => Hand::FiveKind,
        [4, _, _, _, _] => Hand::FourKind,
        [3, 2, _, _, _] => Hand::FullHouse,
        [3, 1, 1, _, _] => Hand::ThreeKind,
        [2, 2, 1, _, _] => Hand::TwoPair,
        [2, 1, 1, 1, _] => Hand::OnePair,
        [1, 1, 1, 1, 1] => Hand::HighCard,
        _ => Hand::HighCard,
    }
}

fn parse_hand_joker(hand: &str) -> Hand {
    let counts = hand.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
        acc
    });
    let hand = if let Some(jokers) = counts.get(&'J').copied() {
        let mut hand = counts
            .into_iter()
            .filter_map(|(k, v)| (k != 'J').then_some(v))
            .fold(([0; 5], 0), |(mut arr, i), value| {
                arr[i] = value;
                (arr, i + 1)
            })
            .0;
        hand.sort_by(|a, b| b.cmp(a));
        hand[0] += jokers;
        hand
    } else {
        let mut hand = counts
            .into_values()
            .fold(([0; 5], 0), |(mut arr, i), value| {
                arr[i] = value;
                (arr, i + 1)
            })
            .0;
        hand.sort_by(|a, b| b.cmp(a));
        hand
    };

    match hand {
        [5, _, _, _, _] => Hand::FiveKind,
        [4, _, _, _, _] => Hand::FourKind,
        [3, 2, _, _, _] => Hand::FullHouse,
        [3, 1, 1, _, _] => Hand::ThreeKind,
        [2, 2, 1, _, _] => Hand::TwoPair,
        [2, 1, 1, 1, _] => Hand::OnePair,
        _ => Hand::HighCard,
    }
}

fn solve<C, H>(input: &str, card_value: C, parse_hand: H) -> u32
where
    C: Fn(char) -> u32,
    H: Fn(&str) -> Hand,
{
    let mut hands = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect::<Vec<_>>();
    hands.sort_by(|(cards1, _), (cards2, _)| {
        let hand1 = parse_hand(cards1);
        let hand2 = parse_hand(cards2);

        let cmp = hand1.cmp(&hand2);
        if cmp != Ordering::Equal {
            cmp
        } else {
            let cards1 = cards1.chars().map(&card_value);
            let cards2 = cards2.chars().map(&card_value);
            for (c1, c2) in cards1.zip(cards2) {
                let cmp = c1.cmp(&c2);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        }
    });

    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid.parse::<u32>().unwrap() * (i + 1) as u32)
        .sum()
}

fn part1(input: &str) -> u32 {
    solve(
        input,
        |c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        },
        parse_hand,
    )
}

fn part2(input: &str) -> u32 {
    solve(
        input,
        |c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 0,
            _ => c.to_digit(10).unwrap(),
        },
        parse_hand_joker,
    )
}

aoc!(
    7,
    r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    6440,
    5905
);
