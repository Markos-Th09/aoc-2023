use std::mem::{self, MaybeUninit};

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
    ($day:literal, $part1_sample:literal, $part1_expected:literal, $part2_expected:literal) => {
        aoc!(
            $day,
            $part1_sample,
            $part1_expected,
            $part1_sample,
            $part2_expected
        );
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

pub trait IteratorExt: Iterator {
    fn chunks<const N: usize>(self) -> Chunks<N, Self>
    where
        Self: Sized,
    {
        Chunks { iter: self }
    }
}

impl<I: Iterator> IteratorExt for I {}

pub struct Chunks<const N: usize, I: Iterator> {
    iter: I,
}

impl<const N: usize, I: Iterator> Iterator for Chunks<N, I> {
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk: [MaybeUninit<I::Item>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut chunk {
            *elem = MaybeUninit::new(self.iter.next()?);
        }
        let chunk = unsafe { mem::transmute_copy::<_, [I::Item; N]>(&chunk) };
        Some(chunk)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let lower = lower / N;
        let upper = upper.map(|upper| upper / N);
        (lower, upper)
    }
}
