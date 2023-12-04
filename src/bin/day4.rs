use std::collections::{HashMap, HashSet};

use aoc_2023::aoc;

fn parse(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, numbers) = numbers.split_once(" | ").unwrap();
        let winning: HashSet<u32> = winning
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let numbers: HashSet<u32> = numbers
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        winning.intersection(&numbers).count() as u32
    })
}

fn part1(input: &str) -> u32 {
    parse(input)
        .map(|won| {
            if won == 0 {
                0
            } else {
                (1..won).fold(1, |acc, _| acc * 2)
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut counts = HashMap::new();
    parse(input)
        .zip(1..)
        .map(|(won, i)| {
            let copies = *counts.entry(i).or_insert(0) + 1;
            for n in 1..=won {
                *counts.entry(i + n).or_insert(0) += copies;
            }
            copies
        })
        .sum()
}

aoc!(
    4,
    r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    13,
    r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    30
);
