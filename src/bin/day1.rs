fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_numeric());
            let first = digits
                .next()
                .expect("should be a digit")
                .to_digit(10)
                .expect("should be a digit");
            match digits.last() {
                Some(num) => first * 10 + num.to_digit(10).expect("should be a digit"),
                None => first * 10 + first,
            }
        })
        .sum()
}

// I would solve it optimally using aho_corasick, but this is fine
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = (0..line.len()).filter_map(|i| {
                let line = &line[i..];

                if line.starts_with("one") {
                    Some(1)
                } else if line.starts_with("two") {
                    Some(2)
                } else if line.starts_with("three") {
                    Some(3)
                } else if line.starts_with("four") {
                    Some(4)
                } else if line.starts_with("five") {
                    Some(5)
                } else if line.starts_with("six") {
                    Some(6)
                } else if line.starts_with("seven") {
                    Some(7)
                } else if line.starts_with("eight") {
                    Some(8)
                } else if line.starts_with("nine") {
                    Some(9)
                } else {
                    line.chars().next().and_then(|c| c.to_digit(10))
                }
            });

            let first = iter.next().expect("should be a digit");
            let last = iter.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

fn main() {
    let input = include_str!("./day1.txt");
    println!("part1: {:?}", part1(input));
    println!("part2: {:?}", part2(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part1() {
        const SAMPLE_INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(super::part1(SAMPLE_INPUT), 142);
    }
    #[test]
    fn part2() {
        const SAMPLE_INPUT: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(super::part2(SAMPLE_INPUT), 281);
    }
}
