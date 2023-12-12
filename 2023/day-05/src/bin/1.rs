#[derive(Debug, Clone)]
struct Portal {
    dst: u64,
    src: u64,
    len: u64,
}

fn main() {
    let input = include_str!("../../input.txt");

    let output: u64 = lowest_location(input);

    // 178159714
    dbg!(output);
}

fn lowest_location(input: &str) -> u64 {
    let (seed, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> =
        seed.split(' ').skip(1).map(|n| n.parse().unwrap()).collect();

    let maps: Vec<Vec<Portal>> = maps
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .skip(1)
                .filter(|v| !v.is_empty())
                .map(|line| {
                    let trio: Vec<u64> =
                        line.split(' ').map(|n| n.parse().unwrap()).collect();
                    Portal { dst: trio[0], src: trio[1], len: trio[2] }
                })
                .collect()
        })
        .collect();

    let result = maps.iter().fold(seeds, |seeds, map| {
        let new_seeds = seeds
            .into_iter()
            .map(|mut seed| {
                for portal in map {
                    if seed <= portal.src + portal.len && seed >= portal.src {
                        seed = portal.dst + (seed - portal.src);
                        break;
                    }
                }
                seed
            })
            .collect();
        new_seeds
    });
    dbg!(&result);
    *result.iter().min().unwrap()
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
