use aoc_2023::aoc;

#[derive(Debug)]
struct Part {
    number: u64,
    star: (usize, usize),
}

fn is_symbol(c: char) -> bool {
    c != '.' && c.is_ascii_punctuation()
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        let mut iter = row
            .iter()
            .enumerate()
            .skip_while(|(_, c)| **c == '.')
            .peekable();
        while iter.peek().is_some() {
            let mut number = iter.by_ref().take_while(|(_, c)| c.is_numeric());
            if let Some((start, _)) = number.next() {
                let end = number.last().map_or(start, |x| x.0);
                let _ = iter.by_ref().skip_while(|(_, c)| **c == '.');

                if (start != 0 && is_symbol(row[start - 1]))
                    || (end != row.len() - 1 && is_symbol(row[end + 1]))
                    || (i != 0
                        && grid[i - 1]
                            [start.saturating_sub(1)..=end.saturating_add(1).min(row.len() - 1)]
                            .iter()
                            .any(|c| is_symbol(*c)))
                    || (i != grid.len() - 1
                        && grid[i + 1]
                            [start.saturating_sub(1)..=end.saturating_add(1).min(row.len() - 1)]
                            .iter()
                            .any(|c| is_symbol(*c)))
                {
                    let number = row[start..=end]
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    sum += number;
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut parts: Vec<Part> = vec![];
    let mut sum = 0;

    for (i, row) in grid.iter().enumerate() {
        let mut iter = row
            .iter()
            .enumerate()
            .skip_while(|(_, c)| **c == '.')
            .peekable();
        while iter.peek().is_some() {
            let mut number = iter.by_ref().take_while(|(_, c)| c.is_numeric());
            if let Some((start, _)) = number.next() {
                let end = number.last().map_or(start, |x| x.0);
                let _ = iter.by_ref().skip_while(|(_, c)| **c == '.');

                let number = row[start..=end]
                    .iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let mut is_part = false;
                let mut star = None;

                if start != 0 && is_symbol(row[start - 1]) {
                    is_part = true;
                    if row[start - 1] == '*' {
                        star = Some((i, start - 1));
                    }
                }
                if end != row.len() - 1 && is_symbol(row[end + 1]) {
                    is_part = true;
                    if row[end + 1] == '*' {
                        star = Some((i, end + 1));
                    }
                }
                if i != 0
                    && (start.saturating_sub(1)..=end.saturating_add(1).min(row.len() - 1))
                        .map(|j| (j, grid[i - 1][j]))
                        .any(|(j, c)| {
                            if c == '*' {
                                star = Some((i - 1, j));
                            }
                            is_symbol(c)
                        })
                {
                    is_part = true;
                }
                if i != grid.len() - 1
                    && (start.saturating_sub(1)..=end.saturating_add(1).min(row.len() - 1))
                        .map(|j| (j, grid[i + 1][j]))
                        .any(|(j, c)| {
                            if c == '*' {
                                star = Some((i + 1, j));
                            }
                            is_symbol(c)
                        })
                {
                    is_part = true;
                }

                if !is_part || star.is_none() {
                    continue;
                }

                let part = Part {
                    number,
                    star: star.unwrap(),
                };

                if let Some(other) = parts.iter().find(|p| p.star == part.star) {
                    sum += other.number * part.number;
                }

                parts.push(part);
            }
        }
    }

    sum
}

aoc!(
    3,
    r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    4361,
    r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    467835
);
