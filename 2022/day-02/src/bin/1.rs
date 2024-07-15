fn main() {
    let input = include_str!("../../input.txt");
    let output = play(input);
    dbg!(output);
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum Hand {
    R,
    P,
    S,
}

impl Hand {
    fn points(&self) -> u32 {
        (match self {
            Self::R => 1,
            Self::P => 2,
            Self::S => 3,
        }) as u32
    }
    fn beat(&self, enemy: &Hand) -> u32 {
        (match (self, enemy) {
            // draw
            (s, e) if s == e => 3,
            // win
            (Self::R, Self::S) | (Self::P, Self::R) | (Self::S, Self::P) => 6,
            // lost
            _ => 0,
        }) as u32
    }
}

impl TryFrom<&str> for Hand {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::R),
            "B" | "Y" => Ok(Self::P),
            "C" | "Z" => Ok(Self::S),
            _ => Err(format!("Invalid char {value}")),
        }
    }
}

/// play rock paper and scissors
fn play(input: &str) -> u32 {
    let mut score: u32 = 0;

    for line in input.lines() {
        let pair: Vec<Hand> =
            line.split(" ").map(|c| c.try_into().unwrap()).collect();
        let (enemy, me) = (pair[0], pair[1]);

        score += me.beat(&enemy) + me.points();
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = play(
            "A Y
B X
C Z",
        );
        assert_eq!(output, 15);
    }
}
