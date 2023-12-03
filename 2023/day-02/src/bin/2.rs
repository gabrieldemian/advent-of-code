use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static COLORS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d+) (red|blue|green)").unwrap());

fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + play(line));

    // 58269
    dbg!(output);
}

fn play(input: &str) -> u32 {
    let input: Vec<(u32, &str)> = COLORS_RE
        .captures_iter(input)
        .map(|v| {
            let (_, [a, b]) = v.extract();
            (a.parse().unwrap(), b)
        })
        .collect();

    let mut numbers: HashMap<&str, u32> =
        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for (number, color) in input {
        if number > numbers[color] {
            *numbers.get_mut(color).unwrap() = number;
        }
    }

    numbers.values().fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input =
            play("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(input, 48);
    }

    #[test]
    fn example_02() {
        let input = play(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        );
        assert_eq!(input, 12);
    }

    #[test]
    fn example_03() {
        let input = play(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        assert_eq!(input, 1560);
    }

    #[test]
    fn example_04() {
        let input = play(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        );
        assert_eq!(input, 630);
    }

    #[test]
    fn example_05() {
        let input =
            play("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(input, 36);
    }
}
