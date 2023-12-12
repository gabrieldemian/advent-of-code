use std::ops::Range;

fn main() {
    let input = include_str!("../../input.txt");

    let output: u64 = lowest_location(input);

    // 100165128
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Portal {
    dst: u64,
    src: u64,
    len: u64,
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

    let mut seeds: Vec<Range<u64>> = Vec::with_capacity(seed.len());

    for p in seed.chunks(2) {
        seeds.push(p[0]..p[0] + p[1]);
    }
    // # Flow
    // initial seed: 79..93, 55..68
    //
    // seed-to-soil map:--
    // 50 98 2  -portal  |--layer
    // 52 50 48 -portal---
    //
    // seed (after map): 81..95, 57..70
    // ...other maps
    //
    // # Partition
    // map_range  50..98
    //        ------------------------------------
    // seed_range 79..93
    //                 ---------------------
    //      [ 50..79 ][  79..93             ][93..98]
    //                   81..93 <- new seed range
    // for each fold iteration, the starting seed will be modified.
    // one seed range will pass through all maps of the input.
    let results: Vec<Range<u64>> = maps.iter().fold(seeds, |seeds, map| {
        println!("fold {seeds:?} maps {maps:?}");
        let seeds = seeds
            .into_iter()
            .flat_map(|seed| {
                // for each seed, there will be partitions and maybe a
                // transformed seed that fits the map.
                let mut mapped_seeds: Vec<Range<u64>> = vec![];
                // all partitions of the `seed` of the flat_map, starting with
                // itself.
                let mut partitions: Vec<Range<u64>> = vec![seed];

                for portal in map {
                    // smaller partitions of each `partition` above.
                    let mut temp_partitions: Vec<Range<u64>> = Vec::new();
                    // println!("un_map {un_map:?}");

                    // for each map, we will also match the smaller partitions
                    // of the original seed range.
                    for partition in partitions {
                        let first: Range<u64> =
                            partition.start..partition.end.min(portal.src);
                        let second: Range<u64> = partition.start.max(portal.src)
                            ..partition.end.min(portal.src + portal.len);
                        let third: Range<u64> = (portal.src + portal.len)
                            .max(partition.start)
                            ..partition.end;

                        // first and third partition will never fit,
                        // however, if they are valid, we want to test
                        // them on the next `map` of the iteration.
                        for partition in [first, third] {
                            if !partition.is_empty() {
                                temp_partitions.push(partition);
                            }
                        }
                        // only the second partition will be the one to fit the
                        // range
                        if !second.is_empty() {
                            mapped_seeds.push(
                                portal.dst + second.start - portal.src
                                    ..portal.dst + second.end - portal.src,
                            );
                        }
                    }
                    // the result of each layer will be the starting point of
                    // the next layer of maps.
                    // println!("new_map {new_map:?}");
                    partitions = temp_partitions;
                }
                mapped_seeds.extend(partitions);
                println!("mapped_seeds {mapped_seeds:?}");
                mapped_seeds
            })
            .collect();
        println!("seeds {seeds:?}");
        seeds
    });
    dbg!(&results);
    results.iter().map(|s| s.start).min().unwrap()
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
        assert_eq!(output, 56);
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
