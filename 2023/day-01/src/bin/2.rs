fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + calibrate(line));

    // 54078
    dbg!(output);
}

fn calibrate(input: &str) -> u32 {
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

    'outer: for start_index in 0..input.len() {
        let c = input.chars().nth(start_index).unwrap();
        if c.is_numeric() {
            let c = c.to_digit(10).unwrap();
            valid_numbers.push(c);
            continue;
        }

        for len in lens {
            let candidate = input.get(start_index..start_index + len);
            if candidate.is_none() {
                continue;
            }
            if let Some(v) = words.iter().find(|v| Some(v.0) == candidate) {
                valid_numbers.push(v.1 as u32);
                continue 'outer;
            }
        }
    }

    if let Some(number) = valid_numbers.first() {
        pair[0] = *number;
    }

    if let Some(number) = valid_numbers.last() {
        pair[1] = *number;
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

    #[test]
    fn example_08() {
        let input = calibrate("pkqxk7");
        assert_eq!(input, 77);
    }

    #[test]
    fn example_09() {
        let input = calibrate("7kqxkk");
        assert_eq!(input, 77);
    }
}
