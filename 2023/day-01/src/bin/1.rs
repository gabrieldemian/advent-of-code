fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = input.lines().fold(0, |acc, line| acc + calibrate(line));

    dbg!(output);
}

fn calibrate(input: &str) -> u32 {
    let mut r = 0;
    let it = input
        .chars()
        .filter(|v| v.is_numeric())
        .map(|v| v.to_digit(10).unwrap());

    let first = it.clone().nth(0).expect("no numbers in the input");
    let last = it.last().expect("no numbers in the input");

    r += first * 10 + last;

    r
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
