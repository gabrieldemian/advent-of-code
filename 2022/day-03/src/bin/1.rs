// 7795
fn main() {
    let input = include_str!("../../input.txt");
    let output = rearange(input);
    dbg!(output);
}

// find items that are common in both compartiments,
// calculate their priority.
fn rearange(input: &str) -> usize {
    let mut sum = 0;

    'outer: for line in input.lines() {
        let half = line.len() / 2;
        for char in line[0..half].bytes() {
            let is_dup = line[half..].bytes().find(|v| *v == char).is_some();
            if is_dup {
                sum += match char {
                    b'a'..=b'z' => 1 + char - b'a',
                    _ => 27 + char - b'A',
                } as usize;
                continue 'outer;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        // half is 12
        let output = rearange(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(output, 157);
    }
}
