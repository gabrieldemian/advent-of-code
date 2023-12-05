fn main() {
    let input = include_str!("../../input.txt");

    let output: u32 = fix_engine(input);

    // 544433
    dbg!(output);
}

fn fix_engine(input: &str) -> u32 {
    let mut r: Vec<u32> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for (symbol_index, symbol) in line.chars().enumerate() {
            if symbol.is_ascii_digit() || symbol == '.' {
                continue;
            }
            dbg!(symbol);

            for adjacent_line_index in -1..2 {
                if adjacent_line_index == -1 && i == 0 {
                    continue;
                };
                let Some(adjacent_line) = input.lines().nth(
                    (i as isize + adjacent_line_index).try_into().unwrap(),
                ) else {
                    continue;
                };
                dbg!(adjacent_line);
                let mut n_char = String::new();
                let mut n_indexes = Vec::new();

                for (y, c) in adjacent_line.chars().enumerate() {
                    if c.is_ascii_digit() {
                        if n_char.len() == 0 {
                            n_indexes.push(y);
                        }
                        n_char.push(c);
                    }

                    let next = adjacent_line.chars().nth(y + 1);

                    if c.is_ascii_digit()
                        && !n_char.is_empty()
                        && (next.is_some_and(|c| !c.is_ascii_digit())
                            || next.is_none())
                    {
                        if n_char.len() > 1 {
                            n_indexes.push(y);
                        }
                        let n: u32 = n_char.parse().unwrap();

                        'outer: for number_index in &n_indexes {
                            for y in -1..2 {
                                let a = *number_index as isize;
                                if !a.overflowing_add(y).1 {
                                    if a + y == symbol_index as isize {
                                        r.push(n);
                                        dbg!(n);
                                        break 'outer;
                                    }
                                }
                            }
                        }
                        n_char.clear();
                        n_indexes.clear();
                    }
                }
            }
        }
    }
    dbg!(&r);

    r.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = fix_engine(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(input, 4361);
    }

    #[test]
    fn example_02() {
        let input = fix_engine(
            "
..........
...*......
....111111
",
        );
        assert_eq!(input, 4361);
    }
}
