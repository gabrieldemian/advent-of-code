fn main() {
    let input = include_str!("../../input.txt");
    let output = find_marker(input);
    dbg!(output);
}

// rearrange crates
fn find_marker(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = find_marker("");
        assert_eq!(output, 7);
    }
}
