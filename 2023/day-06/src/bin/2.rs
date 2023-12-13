fn main() {
    let input = include_str!("../../input.txt");

    let output = race(input);

    // 39594072
    dbg!(output);
}

fn race(input: &str) -> u64 {
    let (time, distance) = input.split_once('\n').unwrap();

    let time: u64 =
        time.replace("Time: ", "").replace(" ", "").parse().unwrap();
    let distance: u64 = distance
        .trim_end()
        .replace("Distance: ", "")
        .replace(" ", "")
        .parse()
        .unwrap();

    let mut n = 0;

    for i in 0..time {
        if i * (time - i) >= distance {
            n = i;
            break;
        }
    }

    time + 1 - n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = race(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(output, 71503);
    }
}
