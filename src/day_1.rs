use std::{default, fmt::format};

fn part1(input: &str) -> usize {
    let mut first;
    let mut last;
    let mut result = 0;
    let mut number: usize;
    for line in input.lines() {
        let mut iter = line.chars().filter(|char| char.is_numeric());

        first = iter
            .next()
            .expect("No number in the Line was found, iterator is empty");

        last = match iter.last() {
            Some(value) => value,
            None => first,
        };

        number = format!("{first}{last}")
            .parse()
            .expect("parse should not fail");
        result = result + number;
    }

    result
}

fn part2(input: impl Into<String>) -> usize {
    let input: String = input.into();
    let input: String = input
        .lines()
        .map(|line| {
            let mut line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");
            line.push_str("\n");
            line
        })
        .collect();

    return part1(input.as_str());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_1_part_1() {
        let input = include_str!("../resources/day1-1.txt");

        let result = part1(input);
        dbg!(result);

        assert_eq!(54877, result);
    }

    #[test]
    fn test_day_1_part_2() {
        let input = include_str!("../resources/day1-1.txt");

        let result = part2(input);
        dbg!(result);

        assert_eq!(54100, result);
    }
}
