fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + calibrate(line));

    // 54078
    dbg!(output);
}

fn calibrate(input: &str) -> u32 {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut numbers = Vec::new();

    for (start_index, c) in input.chars().enumerate() {
        if let Some(c) = c.to_digit(10) {
            numbers.push(c);
        }
        for len in [3, 4, 5] {
            let candidate = input.get(start_index..start_index + len);
            if let Some(i) =
                words.into_iter().position(|v| Some(v) == candidate)
            {
                numbers.push(i as u32 + 1);
            }
        }
    }

    numbers.first().unwrap() * 10 + numbers.last().unwrap()
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
