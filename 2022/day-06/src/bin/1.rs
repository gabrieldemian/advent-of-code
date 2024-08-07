fn main() {
    let input = include_str!("../../input.txt");
    let output = find_marker(input);
    dbg!(output);
}

// rearrange crates
fn find_marker(input: &str) -> usize {
    let mut r = 0;
    'outer: for i in 0..input.len() {
        let Some(marker) = input.get(i..i + 4) else { break };
        for (y, c) in marker.chars().enumerate() {
            if marker
                .chars()
                .enumerate()
                .find(|v| v.1 == c && v.0 != y)
                .is_some()
            {
                continue 'outer;
            }
        }
        r = i + 4;
        break;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(output, 7);
    }
}
