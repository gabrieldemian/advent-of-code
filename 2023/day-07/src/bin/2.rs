fn main() {
    let input = include_str!("../../input.txt");

    let output = play(input);

    //
    dbg!(output);
}

fn play(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = play(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(output, 288);
    }
}
