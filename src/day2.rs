use regex::Regex;

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

struct Bag {
    blue: usize,
    red: usize,
    green: usize,
}

#[derive(Debug)]
struct Round {
    pulls: Vec<Pull>,
}

#[derive(Debug)]
struct Pull {
    color: Color,
    amount: usize,
}

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

fn parse_input(input: String) -> Vec<Game> {
    let game_matcher = Regex::new(r"(Game) ([0-9]+)").unwrap();
    let color_matcher = Regex::new(r"([0-9]+) ([a-z]+)").unwrap();

    let mut games = Vec::new();

    for line in input.lines() {
        let game_capture = game_matcher.captures(line).unwrap();

        let game_id = game_capture
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .expect("Capture Group 2 of game_matcher shoudl be a valid digit");

        let mut rounds = Vec::new();

        let mut possible = true;
        for round in line.split(";") {
            let mut pulls = Vec::new();

            for color_capture in color_matcher.captures_iter(round) {
                let cap_number = color_capture
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let color = match color_capture.get(2).unwrap().as_str() {
                    "red" => Color::RED,
                    "green" => Color::GREEN,
                    "blue" => Color::BLUE,
                    _ => panic!("Color capture got color which is neither red, green or blue"),
                };

                pulls.push(Pull {
                    color,
                    amount: cap_number,
                })
            }
            rounds.push(Round { pulls })
        }

        let game = Game {
            id: game_id,
            rounds,
        };

        games.push(game);
    }

    games
}

fn part1(input: String) -> usize {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = parse_input(input);

    games
        .iter()
        .filter(|game| {
            for round in &game.rounds {
                for pull in &round.pulls {
                    let result = match pull.color {
                        Color::RED => pull.amount <= bag.red,
                        Color::GREEN => pull.amount <= bag.green,
                        Color::BLUE => pull.amount <= bag.blue,
                    };

                    //if the round is valid we check the next round and if not we can already
                    //reaturn here and discard the game as not possible
                    match result {
                        true => continue,
                        false => return result,
                    }
                }
            }
            return true;
        })
        .fold(0, |acc, game| acc + game.id)
}

fn part2(input: String) -> usize {
    let games = parse_input(input);

    let mut result = 0;

    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for round in &game.rounds {
            for pull in &round.pulls {
                match pull.color {
                    Color::RED => {
                        if pull.amount > max_red {
                            max_red = pull.amount;
                        }
                    }
                    Color::GREEN => {
                        if pull.amount > max_green {
                            max_green = pull.amount;
                        }
                    }
                    Color::BLUE => {
                        if pull.amount > max_blue {
                            max_blue = pull.amount;
                        }
                    }
                }
            }
        }

        result = result + (max_green * max_blue * max_red);
    }

    result
}

#[cfg(test)]
mod day2_test {
    use super::*;

    #[test]
    fn day_2_part_1() {
        let input = include_str!("../resources/day2.txt");

        let result = part1(input.to_owned());

        assert_eq!(2239, result);
    }

    #[test]
    fn day_2_part_2() {
        let input = include_str!("../resources/day2.txt");

        let result = part2(input.to_owned());

        assert_eq!(83435, result);
    }
}
