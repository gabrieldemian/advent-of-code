fn main() {
    let input = include_str!("../../input.txt");
    let output = count(input);
    dbg!(output);
}

/// return the highest calorie
fn count(input: &str) -> u32 {
    let mut highest: u32 = 0;
    let mut calories: u32 = 0;

    for line in input.lines() {
        if !line.is_empty() {
            calories += line.parse::<u32>().unwrap();
            continue;
        }
        if calories > highest {
            highest = calories;
        }
        calories = 0;
    }

    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = count(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(output, 24000);
    }
}
