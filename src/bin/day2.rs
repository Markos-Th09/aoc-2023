use aoc_2023::aoc;
#[derive(Debug, Default)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_games(
    input: &str,
) -> impl Iterator<Item = (usize, impl Iterator<Item = GameSet> + '_)> + '_ {
    input.lines().enumerate().map(|(i, line)| {
        let (_game, sets) = line.split_once(':').unwrap();
        (
            i,
            sets.split(';').map(|set| {
                set.split(',').fold(
                    GameSet {
                        blue: 0,
                        red: 0,
                        green: 0,
                    },
                    |mut game_set, item| {
                        let (count, color) = item.trim().split_once(' ').unwrap();
                        let count = count.parse::<u32>().unwrap();
                        match color {
                            "blue" => game_set.blue = count,
                            "red" => game_set.red = count,
                            "green" => game_set.green = count,
                            _ => unreachable!("unknown color"),
                        }
                        game_set
                    },
                )
            }),
        )
    })
}

fn part1(input: &str) -> usize {
    let red = 12;
    let green = 13;
    let blue = 14;

    parse_games(input)
        .filter_map(|(i, mut games)| {
            games
                .all(|set| set.red <= red && set.green <= green && set.blue <= blue)
                .then_some(i + 1)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_games(input)
        .map(|(_, game)| {
            let min_set = game.fold(GameSet::default(), |acc, set| GameSet {
                red: acc.red.max(set.red),
                green: acc.green.max(set.green),
                blue: acc.blue.max(set.blue),
            });

            min_set.red * min_set.green * min_set.blue
        })
        .sum()
}

aoc!(
    2,
    r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    8,
    r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    2286
);
