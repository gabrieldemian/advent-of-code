use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../input.txt");
    let output = rearrange(input);
    dbg!(output);
}

// rearrange crates
fn rearrange(input: &str) -> String {
    let end_index = input.lines().position(|v| v.starts_with(" 1")).unwrap();
    let end_line = input.lines().nth(end_index - 1).unwrap();

    // index of the numbers in the base
    let cols: Vec<usize> = end_line
        .chars()
        .enumerate()
        .filter(|v| v.1.is_alphabetic())
        .map(|v| v.0)
        .collect();

    // transform the text crates into a data structure
    let mut crates = vec![VecDeque::<char>::new(); cols.len()];

    // parse the crates from bottom to top
    // A
    // B ^ push -> crates[i]
    // C ^ push -> crates[i]
    for (col_i, col) in cols.iter().enumerate() {
        for line_i in (0..=end_index - 1).rev() {
            let line = input.lines().nth(line_i).unwrap();
            let char = line.chars().nth(*col).unwrap();
            if char == ' ' {
                continue;
            };
            crates[col_i].push_front(char);
        }
    }
    // dbg!(&crates);

    for line in input.lines().skip(end_index + 2) {
        // move from to
        let mut mft: Vec<usize> = Vec::new();

        let mut tmp = String::new();

        // parse string numbers into number
        for (i, char) in line.chars().enumerate() {
            if char.is_numeric() {
                tmp.extend([char]);
            }
            if (!char.is_numeric() && !tmp.is_empty())
                || i == line.chars().count() - 1
            {
                mft.push(tmp.parse().unwrap());
                tmp = String::new();
            }
        }

        // dbg!(&mft);
        for _ in 0..mft[0] {
            let item = crates[mft[1] - 1].pop_front();
            if let Some(item) = item {
                crates[mft[2] - 1].push_front(item);
            }
        }
    }

    // dbg!(&crates);

    let mut result = String::new();

    for mut cr in crates {
        let top = cr.pop_front();
        if let Some(top) = top {
            result.push(top);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = rearrange(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!(output, "CMZ");
    }
}
