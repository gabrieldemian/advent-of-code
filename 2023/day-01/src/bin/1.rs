fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + calibrate(line));

    // 54601
    dbg!(output);
}

fn calibrate(input: &str) -> u32 {
    let values: Vec<u32> =
        input.chars().filter_map(|v| v.to_digit(10)).collect();
    let first = values.first().unwrap();
    let last = values.last().unwrap_or(first);
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = calibrate("1abc2");
        assert_eq!(input, 12);
    }

    #[test]
    fn example_02() {
        let input = calibrate("pqr3stu8vwx");
        assert_eq!(input, 38);
    }

    #[test]
    fn example_03() {
        let input = calibrate("a1b2c3d4e5f");
        assert_eq!(input, 15);
    }

    #[test]
    fn example_04() {
        let input = calibrate("treb7uchet");
        assert_eq!(input, 77);
    }
}
