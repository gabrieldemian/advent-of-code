fn main() {
    let input = include_str!("../../input.txt");
    let output = count_overlaps(input);
    dbg!(output);
}

// how many pairs partially overlap each other
// -----
//    -----
//     -----
// ------
fn count_overlaps(input: &str) -> usize {
    let mut count = 0;

    for line in input.lines() {
        let (first, second) = line.split_once(',').unwrap();
        let (first_start, first_end) = first.split_once('-').unwrap();
        let (second_start, second_end) = second.split_once('-').unwrap();

        let first_start: u32 = first_start.parse().unwrap();
        let first_end: u32 = first_end.parse().unwrap();
        let second_start: u32 = second_start.parse().unwrap();
        let second_end: u32 = second_end.parse().unwrap();

        if 
            first_start <= second_end && first_end >= second_start
        {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_02() {
        let output = count_overlaps(
            "1-92,5-92
5-92,1-92
10-200,10-20",
        );
        assert_eq!(output, 3);
    }

    #[test]
    fn example_01() {
        let output = count_overlaps(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );
        assert_eq!(output, 4);
    }
}
