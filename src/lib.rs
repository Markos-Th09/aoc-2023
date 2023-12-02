#[macro_export]
macro_rules! aoc {
    ($day:literal, $part1_sample:literal, $part1_expected:literal) => {
        fn main() {
            let input = include_str!(concat!("./day", $day, ".txt"));
            println!("part1: {:?}", part1(input));
        }
        #[cfg(test)]
        mod test {
            #[test]
            fn part1() {
                assert_eq!(super::part1($part1_sample), $part1_expected);
            }
        }
    };
    ($day:literal, $part1_sample:literal, $part1_expected:literal, $part2_sample:literal, $part2_expected:literal) => {
        fn main() {
            let input = include_str!(concat!("./day", $day, ".txt"));
            println!("part1: {:?}", part1(input));
            println!("part2: {:?}", part2(input));
        }
        #[cfg(test)]
        mod test {
            #[test]
            fn part1() {
                assert_eq!(super::part1($part1_sample), $part1_expected);
            }
            #[test]
            fn part2() {
                assert_eq!(super::part2($part2_sample), $part2_expected);
            }
        }
    };
}
