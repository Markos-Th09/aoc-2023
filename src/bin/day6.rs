use aoc_2023::*;

// Why use brute force, when you have math?
// D=record distance, T=time, B=button press time
// D = B * (T - B)
// D = TB - B^2
// B^2 - TB + D = 0
// B = (T ± sqrt(T^2 - 4D)) / 2
// Answer is the number of integers between the two roots (exclusive)
// so ceil(B_1) - floor(B_2) - 1
fn solve(time: u64, distance: u64) -> usize {
    let time = time as f64;
    let distance = distance as f64;
    let b1 = (time + (time * time - 4.0 * distance).sqrt()) / 2.0;
    let b2 = (time - (time * time - 4.0 * distance).sqrt()) / 2.0;
    b1.ceil() as usize - b2.floor() as usize - 1
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| solve(time, distance))
        .product()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    solve(time, distance)
}

aoc!(
    6,
    r"Time:      7  15   30
Distance:  9  40  200",
    288,
    71503
);
