fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + calibrate(line));

    dbg!(output);
}

fn calibrate(input: &str) -> u32 {
    dbg!(input);
    let words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let lens: [usize; 3] = [3, 4, 5];
    let mut pair = [0, 0];
    let mut valid_numbers = Vec::new();

    for len in lens {
        for start_index in 0..input.len() {
            let c = input.chars().nth(start_index).unwrap();

            if c.is_numeric() {
                let c = c.to_digit(10).unwrap();
                dbg!(c);
                // if !valid_numbers.iter().any(|v| *v == c) {
                valid_numbers.push(c);
                // }
            }

            if start_index > input.len() - 1 {
                break;
            };

            let end_index = start_index + len;
            let candidate = input.get(start_index..end_index);

            if let Some(v) = words.iter().find(|v| Some(v.0) == candidate) {
                dbg!(v);
                valid_numbers.push(v.1 as u32);
            }
        }
    }

    if let Some(number) = valid_numbers.first() {
        pair[0] = *number;
    }

    if let Some(number) = valid_numbers.last() {
        pair[1] = *number;
    }
    dbg!(valid_numbers);

    let it = input
        .chars()
        .filter(|v| v.is_numeric())
        .map(|v| v.to_digit(10).unwrap());

    if pair[0] == 0 {
        pair[0] = it.clone().nth(0).expect("no numbers in the input");
    }

    if pair[1] == 0 {
        pair[1] = pair[0];
    }

    pair[0] * 10 + pair[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = calibrate("two1nine");
        assert_eq!(input, 29);
    }

    #[test]
    fn example_02() {
        let input = calibrate("eightwothree");
        assert_eq!(input, 83);
    }

    #[test]
    fn example_03() {
        let input = calibrate("abcone2threexyz");
        assert_eq!(input, 13);
    }

    #[test]
    fn example_04() {
        let input = calibrate("xtwone3four");
        assert_eq!(input, 24);
    }

    #[test]
    fn example_05() {
        let input = calibrate("4nineeightseven2");
        assert_eq!(input, 42);
    }

    #[test]
    fn example_06() {
        let input = calibrate("zoneight234");
        assert_eq!(input, 14);
    }

    #[test]
    fn example_07() {
        let input = calibrate("7pqrstsixteen");
        assert_eq!(input, 76);
    }
}
