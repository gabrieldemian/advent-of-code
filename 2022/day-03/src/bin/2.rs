#![feature(iter_array_chunks)]
// 2703
fn main() {
    let input = include_str!("../../input.txt");
    let output = find_common(input);
    dbg!(output);
}

fn find_common(input: &str) -> usize {
    let mut sum = 0;

    'outer: for [first, second, third] in
        input.lines().into_iter().array_chunks()
    {
        for char in first.chars() {
            if second.contains(char) && third.contains(char) {
                sum += match char as u8 {
                    b'a'..=b'z' => 1 + char as u8 - b'a',
                    _ => 27 + char as u8 - b'A',
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
        let output = find_common(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(output, 70);
    }
}
