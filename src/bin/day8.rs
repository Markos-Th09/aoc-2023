use std::{collections::HashMap, mem};

use aoc_2023::*;

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    let nodes = lines
        .skip(1)
        .map(|line| {
            let (name, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            (name, (&left[1..], &right[..right.len() - 1]))
        })
        .collect::<HashMap<_, _>>();
    let mut current = "AAA";
    let mut counter = 0;
    for instruction in instructions {
        let (left, right) = nodes.get(current).unwrap();
        current = if instruction == 'L' { left } else { right };
        counter += 1;
        if current == "ZZZ" {
            break;
        }
    }
    counter
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            mem::swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    let rest = lines.skip(1);
    let nodes = rest
        .clone()
        .map(|line| {
            let (name, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            (name, (&left[1..], &right[..right.len() - 1]))
        })
        .collect::<HashMap<_, _>>();
    rest.filter_map(|line| {
        let node = line.split_once(" = ").unwrap().0;
        node.ends_with('A').then_some(node)
    })
    .map(|start| {
        let mut instructions = instructions.clone();
        let mut current = start;
        let mut counter = 0;
        while !current.ends_with('Z') {
            let (left, right) = nodes.get(current).unwrap();
            current = if instructions.next().unwrap() == 'L' {
                left
            } else {
                right
            };
            counter += 1;
        }
        counter
    })
    .reduce(lcm)
    .unwrap()
}

aoc!(
    8,
    r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    6,
    r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    6
);
