fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + play(line));

    // 2101
    dbg!(output);
}

fn play(input: &str) -> u32 {
    let dots_index = input.chars().position(|v| v == ':').unwrap();
    let id = input.get(5..dots_index);
    let id = id.unwrap().parse::<u32>().unwrap();

    let input = input.split_at(dots_index + 2).1.replace(";", ",");
    let input: Vec<&str> = input.split(", ").collect();

    for line in input {
        let pair: Vec<&str> = line.split(" ").collect();
        let number: u32 = pair[0].parse().unwrap();
        let color: String = pair[1].parse().unwrap();

        match color.as_str() {
            "red" if number > 12 => {
                return 0;
            }
            "green" if number > 13 => {
                return 0;
            }
            "blue" if number > 14 => {
                return 0;
            }
            _ => {}
        }
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
