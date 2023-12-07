use std::ops::Range;

fn main() {
    let input = include_str!("../../input.txt");

    let output: u64 = lowest_location(input);

    // 260119618
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Portal {
    dst: u64,
    src: u64,
    len: u64,
}

impl Portal {
    fn range(&self) -> Range<u64> {
        self.src..self.src + self.len
    }
}

fn lowest_location(input: &str) -> u64 {
    let (seed, layer) = input.split_once("\n\n").unwrap();
    let seed = seed
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let maps: Vec<Vec<Portal>> = layer
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|m| {
                    let trio: Vec<u64> =
                        m.split(' ').map(|n| n.parse().unwrap()).collect();
                    Portal { dst: trio[0], src: trio[1], len: trio[2] }
                })
                .collect::<Vec<Portal>>()
        })
        .collect::<Vec<_>>();

    let mut range_seed: Vec<Range<u64>> = Vec::with_capacity(seed.len());

    for p in seed.chunks(2) {
        range_seed.push(p[0]..p[0] + p[1]);
    }

    let seed = range_seed.clone();

    // dbg!(&seed);
    // dbg!(&maps);
    // dbg!(&range_seed);

    let result = maps.iter().fold(seed, |seed, maps| {
        seed.into_iter()
            .flat_map(|s| {
                println!("seed range: {s:?}");
                // map_range  50..98
                //        ------------------------------------
                // seed_range 79..93
                //                 ---------------------
                //      [ 50..79 ][  79..93             ][93..98]
                //                   81..93
                // map_range
                //        ------------------------------------
                // seed_range
                // --------------------
                //[       ][           ][                    ]
                let mut new_map: Vec<Range<u64>> = vec![];

                for map in maps {
                    let map_range = map.range();

                    let first: Range<u64> = Range {
                        start: s.start.min(map_range.start),
                        end: s.start.max(map_range.start),
                    };

                    let second: Range<u64> = Range {
                        start: s.start.max(map_range.start),
                        end: s.end.min(map_range.end),
                    };

                    let third: Range<u64> = Range {
                        start: s.end.min(map_range.end),
                        end: s.end.max(map_range.end),
                    };

                    println!("map range: {map_range:?}");

                    for mut partition in [first, second, third] {
                        let fits = partition.start > map_range.start
                            && partition.end < map_range.end;

                        println!(
                            "range: {partition:?} fits {}",
                            partition.start > map_range.start
                                && partition.end < map_range.end
                        );

                        if fits {
                            let start = map.dst + (partition.start - map.src);
                            partition.start = start;
                            partition.end =
                                start + (partition.end - partition.start);
                            println!("transformed: {partition:?}");
                        }
                        new_map.push(partition);
                        if fits {
                            break;
                        };
                    }
                    println!("---");
                }

                new_map
            })
            .collect()
    });
    result.iter().map(|s| s.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = lowest_location(
            "seeds: 79 14

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
        assert_eq!(output, 46);
    }

    #[test]
    fn example_02() {
        let output = lowest_location(
            "seeds: 55 13

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
        assert_eq!(output, 86);
    }

    #[test]
    fn example_03() {
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
        assert_eq!(output, 46);
    }
}
