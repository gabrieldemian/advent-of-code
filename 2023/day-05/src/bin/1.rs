fn main() {
    let input = include_str!("../../input.txt");

    let output: u64 = lowest_location(input);

    // 178159714
    dbg!(output);
}

/// A str of integers separated by single space
/// that can safely be converted to a vec/array of integers.
#[derive(Debug, Clone)]
struct NString<'a>(&'a str);

impl<'a> From<NString<'a>> for [u64; 3] {
    fn from(value: NString<'a>) -> Self {
        let mut r: [u64; 3] = [0; 3];
        for (i, v) in value.0.split(" ").enumerate() {
            r[i] = v.parse().unwrap();
        }
        r
    }
}

impl<'a> From<NString<'a>> for Vec<u64> {
    fn from(value: NString<'a>) -> Self {
        value.0.split(" ").map(|c| c.parse().unwrap()).collect()
    }
}

fn lowest_location(input: &str) -> u64 {
    // dr sr len
    // dr sr len
    let mut seeds: Vec<u64> = NString(
        input.lines().nth(0).unwrap().split("seeds: ").collect::<Vec<&str>>()
            [1],
    )
    .into();
    println!("seeds {seeds:?}");

    let mut maps: Vec<(usize, usize)> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if i < 2 {
            continue;
        };
        if line.find(':').is_some() {
            maps.push((i + 1, 0));
        }
        let is_last = i == input.lines().count() - 1;
        if line.is_empty() || is_last {
            let pair = maps.last_mut().unwrap();
            pair.1 = if is_last { i + 1 } else { i };
        }
    }

    for (_, (start, end)) in maps.iter().enumerate() {
        let lines: Vec<[u64; 3]> = input
            .lines()
            .skip(*start)
            .take(end - start)
            .map(|v| NString(v).into())
            .collect();

        // println!("maps of line {lines:?}");
        'seed_loop: for (seed_i, seed) in seeds.iter_mut().enumerate() {
            for (_, [dr, sr, len]) in lines.iter().enumerate() {
                if (*seed >= *sr) && *seed <= *sr + *len {
                    let diff = *seed - *sr;
                    let output = dr + diff;
                    *seed = output;
                    continue 'seed_loop;
                }
                // if seed_i == 1 {
                //     println!("seed {seed} sr {sr}");
                //     println!("---");
                // }
            }
        }
    }

    dbg!(&seeds);
    seeds.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = lowest_location(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
        );
        assert_eq!(output, 35);
    }
}