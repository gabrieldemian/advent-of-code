use std::{collections::HashMap, path::PathBuf};

// 1644735
fn main() {
    let input = include_str!("../../input.txt");
    let output = calc_dir(input);
    dbg!(output);
}

#[derive(Debug)]
struct Ls;

#[derive(Debug)]
struct Cd(PathBuf);

#[derive(Debug)]
enum Command {
    Ls(Ls),
    Cd(Cd),
}

#[derive(Debug)]
enum Entry {
    Dir(PathBuf),
    File(u64, PathBuf),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

impl<'a> TryFrom<&'a str> for Line {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // its a command
        if let Some((_, command)) = value.split_once("$ ") {
            return match command {
                "ls" => Ok(Self::Command(Command::Ls(Ls))),
                v if v.starts_with("cd") => {
                    Ok(Self::Command(Command::Cd(Cd(v
                        .replace("cd ", "")
                        .into()))))
                }
                _ => Err("not a valid command"),
            };
        }
        // an output of a command, an entry
        match value.split_once(' ') {
            Some(("dir", dir)) => Ok(Self::Entry(Entry::Dir(dir.into()))),
            Some((size, filename)) => Ok(Self::Entry(Entry::File(
                size.parse().unwrap(),
                filename.into(),
            ))),
            _ => Err("error"),
        }
    }
}

// find all of the directories with a total size <= 100_000 and sum all of them
fn calc_dir(input: &str) -> u64 {
    // map of dirs and total size
    let mut fs: HashMap<PathBuf, u64> = HashMap::new();

    let lines: Vec<Line> =
        input.lines().map(|v| v.try_into().unwrap()).collect();

    // dbg!(&lines);

    let mut curr_dir = PathBuf::new();

    for line in lines {
        match line {
            Line::Command(Command::Cd(Cd(path))) => match path.to_str() {
                Some("..") => {
                    curr_dir.pop();
                }
                Some(p) => curr_dir.push(p),
                _ => {}
            },
            Line::Entry(Entry::File(size, _name)) => {
                let mut tmp = curr_dir.clone();
                *fs.entry(curr_dir.clone()).or_default() += size;
                // recursively sum their parents as well
                while tmp.pop() == true {
                    *fs.entry(tmp.clone()).or_default() += size;
                }
            }
            _ => {}
        }
    }

    // dbg!(&fs);
    fs.into_values().fold(0, |acc, v| if v <= 100_000 { acc + v } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = calc_dir(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );
        assert_eq!(output, 95_437);
    }
}
