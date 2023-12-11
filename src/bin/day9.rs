use aoc_2023::*;

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            std::iter::successors(Some(nums), |nums| {
                (!nums.iter().all(|num| *num == 0))
                    .then_some(nums.windows(2).map(|w| w[1] - w[0]).collect())
            })
            .map(|v| *v.last().unwrap())
            .sum::<i64>()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            std::iter::successors(Some(nums), |nums| {
                (!nums.iter().all(|num| *num == 0))
                    .then_some(nums.windows(2).map(|w| w[1] - w[0]).collect())
            })
            .map(|v| *v.first().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .fold(0, |acc, num| num - acc)
        })
        .sum::<i64>()
}

aoc!(
    9,
    r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    114,
    2
);
