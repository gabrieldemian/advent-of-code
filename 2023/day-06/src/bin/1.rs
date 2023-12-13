use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");

    let output = race(input);

    // 128700
    dbg!(output);
}

fn race(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)").unwrap();
    let (_time, _distance) = input.split_once('\n').unwrap();

    let mut time = Vec::new();
    let mut distance = Vec::new();

    for (_, [v]) in re.captures_iter(_time).map(|v| v.extract()) {
        time.push(v);
    }

    for (_, [v]) in re.captures_iter(_distance).map(|v| v.extract()) {
        distance.push(v);
    }

    let mut time_distance = Vec::new();

    let time: Vec<u32> =
        time.into_iter().map(|v| v.parse::<u32>().unwrap()).collect();
    let distance: Vec<u32> =
        distance.into_iter().map(|v| v.parse::<u32>().unwrap()).collect();

    for pair in time.into_iter().zip(distance.into_iter()) {
        time_distance.push(pair);
    }

    dbg!(&time_distance);

    let result: Vec<u32> = time_distance.into_iter().fold(
        Vec::new(),
        |mut acc, (time, distance)| {
            let mut result = 0;
            for i in 1..=time {
                let remaining = time - i;
                let r = remaining * i;
                if r > distance {
                    result += 1;
                }
            }
            println!("time {time} distance {distance} r={result}");
            acc.push(result);
            acc
        },
    );
    dbg!(&result);
    result.into_iter().product()
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
        assert_eq!(output, 288);
    }
}
