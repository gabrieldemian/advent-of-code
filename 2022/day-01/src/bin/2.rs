fn main() {
    let input = include_str!("../../input.txt");
    let output = count(input);
    dbg!(output);
}

/// return the sum of the top 3 highest calories
fn count(input: &str) -> u32 {
    let mut top_3 = [0u32; 3];

    let mut calories: u32 = 0;
    let last_i = input.lines().count() - 1;

    for (i, line) in input.lines().enumerate() {
        if i == last_i {
            calories += line.parse::<u32>().unwrap();
        }
        if line.is_empty() || i == last_i {
            let min = top_3.iter_mut().min().unwrap();
            if calories >= *min {
                *min = calories;
            }
            calories = 0;
            continue;
        }
        calories += line.parse::<u32>().unwrap();
    }

    top_3.into_iter().sum()
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
        assert_eq!(output, 45000);
    }
}
