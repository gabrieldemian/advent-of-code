use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + play(line));

    // 2101
    dbg!(output);
}

fn play(input: &str) -> u32 {
    let id_re = Regex::new(r"^\w+ (\d+)").unwrap();
    let colors_re = Regex::new(r"(\d+) (red|blue|green)").unwrap();

    let id: u32 = id_re.captures(input).unwrap()[1].parse().unwrap();
    let pairs: Vec<&str> =
        colors_re.find_iter(input).map(|v| v.as_str()).collect();

    for pair in pairs {
        let number: u32 = colors_re.captures(pair).unwrap()[1].parse().unwrap();
        let color = &colors_re.captures(pair).unwrap()[2];

        if color == "red" && number > 12
            || color == "green" && number > 13
            || color == "blue" && number > 14
        {
            return 0;
        };
    }

    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input =
            play("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(input, 1);
    }

    #[test]
    fn example_02() {
        let input = play(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        );
        assert_eq!(input, 2);
    }

    #[test]
    fn example_03() {
        let input = play(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        assert_eq!(input, 0);
    }

    #[test]
    fn example_04() {
        let input = play(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        );
        assert_eq!(input, 0);
    }

    #[test]
    fn example_05() {
        let input =
            play("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(input, 5);
    }
}
