fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + play(line));

    // 58269
    dbg!(output);
}

fn play(input: &str) -> u32 {
    let dots_index = input.chars().position(|v| v == ':').unwrap();

    let input = input.split_at(dots_index + 2).1.replace(";", ",");
    let input: Vec<&str> = input.split(", ").collect();

    let mut numbers: [u32; 3] = [0, 0, 0];

    for line in input {
        let pair: Vec<&str> = line.split(" ").collect();
        let number: u32 = pair[0].parse().unwrap();
        let color: String = pair[1].parse().unwrap();

        match color.as_str() {
            "red" if number > numbers[0] => {
                numbers[0] = number;
            }
            "green" if number > numbers[1] => {
                numbers[1] = number;
            }
            "blue" if number > numbers[2] => {
                numbers[2] = number;
            }
            _ => {}
        }
    }

    numbers.into_iter().fold(1, |acc, v| acc * v)
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
