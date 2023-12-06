fn main() {
    let input = include_str!("../../input.txt");

    let output = play_cards(input);

    // 5422730
    dbg!(output);
}

fn play_cards(input: &str) -> u32 {
    let mut cards = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let mut points: usize = 0;
        let start_w = line.find(": ").unwrap() + 2;
        let end_w = line.find(" |").unwrap();

        let winning = line.get(start_w..end_w).unwrap();
        let winning: Vec<u8> =
            winning.split(" ").filter_map(|v| v.parse().ok()).collect();

        let hand = line.get(end_w + 3..).unwrap();
        let hand: Vec<u8> =
            hand.split(" ").filter_map(|v| v.parse().ok()).collect();

        for h in &hand {
            if winning.iter().any(|v| *v == *h) {
                points += 1;
            }
        }

        let end = (i + points).min(cards.len() - 1);

        for y in i..end {
            cards[y + 1] += cards[i];
        }
    }

    cards.into_iter().map(|v| v as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = play_cards(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );
        assert_eq!(output, 30);
    }

    #[test]
    fn example_02() {
        let output = play_cards(
            "Card   1: 42 68 56  3 28 97  1 78 55 48 | 78 54  3 38 94 73 72 57 51 31 86 43  7 81  4 27 26 58 75 69 74 55  5 28 40
",
        );
        assert_eq!(output, 1);
    }
}
